use anyhow::Result;
use bevy::{
    ecs::{system::SystemState, world::CommandQueue},
    prelude::*,
};
use pb_engine::{
    save::{load, save, LoadParam, LoadSeed, Save, SaveParam},
    Root,
};
use pb_store::{Metadata, Store};
use smol_str::SmolStr;

use pb_assets::Assets;
use pb_util::{
    callback::CallbackSender, run_system_cached, run_system_cached_with, spawn_io, try_res_s,
    AsDynError,
};

use crate::{
    layout::Layout,
    menu::MenuPanel,
    message::Message,
    theme::Theme,
    widget::{
        form::{self, Form, FormField, FormSubmit},
        UiBuilder,
    },
    EngineState, UiState,
};

#[derive(Clone, Copy, Debug)]
enum SaveAction {
    Save,
    Load,
}

#[derive(Component)]
struct SaveItem(Metadata);

#[derive(Debug, Clone, Reflect)]
struct SaveForm {
    name: String,
}

pub fn save_panel_button(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) {
    for (id, &panel) in &panel_q {
        commands.entity(id).despawn_recursive();
        if panel == MenuPanel::Save {
            return;
        }
    }

    UiBuilder::new(commands, layout.menu)
        .save_panel(&theme, &assets, store.clone(), callback.clone())
        .insert(MenuPanel::Save);
}

pub fn load_panel_button(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) {
    for (id, &panel) in &panel_q {
        commands.entity(id).despawn_recursive();
        if panel == MenuPanel::Load {
            return;
        }
    }

    UiBuilder::new(commands, layout.menu)
        .load_panel(&theme, &assets, store.clone(), callback.clone())
        .insert(MenuPanel::Load);
}

fn refresh_save_panel(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) {
    if !panel_q.iter().any(|(_, &panel)| panel == MenuPanel::Save) {
        return;
    }

    for (id, _) in &panel_q {
        commands.entity(id).despawn_recursive();
    }

    UiBuilder::new(commands, layout.menu)
        .save_panel(&theme, &assets, store.clone(), callback.clone())
        .insert(MenuPanel::Save);
}

fn load_button(
    event: Trigger<Pointer<Click>>,
    mut commands: Commands,
    save_q: Query<&SaveItem>,
    mut ui_state: ResMut<NextState<UiState>>,
    engine_root: Query<Entity, With<Root>>,
    mut next_engine_state: ResMut<NextState<EngineState>>,
    registry: Res<AppTypeRegistry>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) {
    let save_name = try_res_s!(save_q.get(event.target)).0.name.clone();

    if let Ok(root) = engine_root.get_single() {
        commands.entity(root).despawn_recursive();
    }

    ui_state.set(UiState::LoadingSave);
    next_engine_state.set(EngineState::Disabled);

    let store = store.clone();
    let callback = callback.clone();
    let seed = LoadSeed::new(registry.0.clone());
    spawn_io(async move {
        let res = store
            .get_with(&format!("saves/{save_name}.json"), seed)
            .await;
        callback.send_oneshot_system_with_input(on_load_complete, res);
    });

    fn on_load_complete(
        In(save): In<Result<Save>>,
        world: &mut World,
        load_p: &mut SystemState<LoadParam>,
        state: &mut SystemState<(
            ResMut<NextState<UiState>>,
            ResMut<NextState<EngineState>>,
            EventWriter<Message>,
        )>,
    ) {
        let res = save.and_then(|save| load(world, load_p, &save));
        let (mut ui_state, mut engine_state, mut message_e) = state.get_mut(world);

        match res {
            Ok(root) => {
                ui_state.set(UiState::Game);
                engine_state.set(EngineState::Running(root));
                info!("Successfully loaded save");
            }
            Err(error) => {
                let error = error.as_dyn_error();
                error!(error, "Failed to load save");

                ui_state.set(UiState::Menu);

                message_e.send(Message::error(&error));
            }
        }
    }
}

fn overwrite_button(
    event: Trigger<Pointer<Click>>,
    world: &World,
    save_q: Query<&SaveItem>,
    save_p: SaveParam,
    state: Res<State<EngineState>>,
    store: Res<Store>,
    callback: Res<CallbackSender>,
) {
    let save_name = try_res_s!(save_q.get(event.target)).0.name.clone();
    save_impl(
        save_name,
        world,
        save_p,
        *state.get(),
        store.clone(),
        callback.clone(),
    );
}

fn save_button(
    event: Trigger<FormSubmit>,
    world: &World,
    form_q: Query<&Form>,
    save_p: SaveParam,
    state: Res<State<EngineState>>,
    store: Res<Store>,
    callback: Res<CallbackSender>,
) {
    let save_form = try_res_s!(form_q.get(event.entity()))
        .value::<SaveForm>()
        .unwrap();
    save_impl(
        SmolStr::from(&save_form.name),
        world,
        save_p,
        *state.get(),
        store.clone(),
        callback.clone(),
    );
}

fn save_impl(
    name: SmolStr,
    world: &World,
    save_p: SaveParam,
    state: EngineState,
    store: Store,
    callback: CallbackSender,
) {
    if name.is_empty() {
        return;
    }

    let EngineState::Running(root) = state else {
        error!("Failed to save: not running");
        return;
    };

    let scene = save(world, &save_p, root);

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store.set(&format!("saves/{name}.json"), scene).await;

        let mut queue = CommandQueue::default();
        queue.push(run_system_cached_with(on_save_complete, (name, res)));
        queue.push(run_system_cached(refresh_save_panel));
        callback.send_batch(queue);
    });

    fn on_save_complete(
        In((name, res)): In<(SmolStr, Result<()>)>,
        mut message_e: EventWriter<Message>,
    ) {
        match res {
            Ok(()) => {
                info!("Successfully saved '{name}'");
                message_e.send(Message::info(format!("Successfully saved '{name}'")));
            }
            Err(error) => {
                let error = error.as_dyn_error();
                error!(error, "Failed to save");
                message_e.send(Message::error(&error));
            }
        }
    }
}

impl<'w> UiBuilder<'w, '_> {
    fn save_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        store: Store,
        callback: CallbackSender,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.menu_panel(theme, assets, "Save prison");
        panel.saves_table(theme, store, callback, SaveAction::Save);

        let mut save_form = panel.form(
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: theme.gutter,
                align_items: AlignItems::Center,
                ..default()
            },
            SaveForm {
                name: String::new(),
            },
        );
        save_form.observe(save_button);

        save_form.spawn((Text::new("Name"), theme.normal_text.clone()));
        save_form.input(theme).insert(FormField::new("name"));
        save_form
            .button(theme, assets, "Save", default())
            .on_click(form::submit);

        panel
    }

    fn load_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        store: Store,
        callback: CallbackSender,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.menu_panel(theme, assets, "Load prison");
        panel.saves_table(theme, store, callback, SaveAction::Load);
        panel
    }

    fn saves_table(
        &mut self,
        theme: &Theme,
        store: Store,
        callback: CallbackSender,
        action: SaveAction,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.container(Node {
            flex_grow: 1.,
            align_self: AlignSelf::Stretch,
            min_width: Val::Px(425.),
            ..default()
        });
        let container_id = container.id();

        container.spinner(theme, theme.large_icon_size_px);

        spawn_io(async move {
            let res = store.iter("saves/").await;
            callback.send_oneshot_system_with_input(on_list_complete, (res, container_id, action));
        });

        fn on_list_complete(
            In((res, container_id, action)): In<(Result<Vec<Metadata>>, Entity, SaveAction)>,
            mut commands: Commands,
            theme: Res<Theme>,
            assets: Res<Assets>,
        ) {
            let Some(mut container) = commands.get_entity(container_id) else {
                return;
            };

            let mut builder = UiBuilder::from(&mut container);
            builder.clear();

            match res {
                Ok(items) => {
                    builder.saves_table_items(&theme, &assets, items, action);
                }
                Err(error) => {
                    error!(error = error.as_dyn_error(), "failed to load saves");
                    builder.error_message(&theme, &assets, error.to_string_compact());
                }
            }
        }

        self.reborrow()
    }

    fn saves_table_items(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        items: Vec<Metadata>,
        action: SaveAction,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.container(Node {
            display: Display::Grid,
            width: Val::Percent(100.),
            grid_template_columns: vec![
                GridTrack::minmax(
                    MinTrackSizingFunction::Auto,
                    MaxTrackSizingFunction::Fraction(1.),
                ),
                GridTrack::minmax(
                    MinTrackSizingFunction::Auto,
                    MaxTrackSizingFunction::Fraction(2.),
                ),
                GridTrack::auto(),
            ],
            grid_auto_rows: vec![GridTrack::max_content()],
            row_gap: theme.gutter,
            column_gap: theme.gutter,
            align_items: AlignItems::Center,
            ..default()
        });

        container.spawn((
            Text::new("Name"),
            theme.emphasis_text.clone(),
            Node {
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(1),
                ..default()
            },
        ));
        container.spawn((
            Text::new("Modified"),
            theme.emphasis_text.clone(),
            Node {
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(2),
                ..default()
            },
        ));

        for (row, item) in items.into_iter().enumerate() {
            container.spawn((
                Text::new(item.name.clone()),
                theme.normal_text.clone(),
                Node {
                    grid_row: GridPlacement::start(row as i16 + 2),
                    grid_column: GridPlacement::start(1),
                    ..default()
                },
            ));
            container.spawn((
                Text::new(item.modified_local().to_rfc2822()),
                theme.normal_text.clone(),
                Node {
                    grid_row: GridPlacement::start(row as i16 + 2),
                    grid_column: GridPlacement::start(2),
                    ..default()
                },
            ));

            match action {
                SaveAction::Save => container
                    .button(
                        theme,
                        assets,
                        "Overwrite",
                        Node {
                            grid_row: GridPlacement::start(row as i16 + 2),
                            grid_column: GridPlacement::start(3),
                            ..default()
                        },
                    )
                    .on_click(overwrite_button)
                    .insert(SaveItem(item)),
                SaveAction::Load => container
                    .button(
                        theme,
                        assets,
                        "Load",
                        Node {
                            grid_row: GridPlacement::start(row as i16 + 2),
                            grid_column: GridPlacement::start(3),
                            ..default()
                        },
                    )
                    .on_click(load_button)
                    .insert(SaveItem(item)),
            };
        }

        container
    }
}
