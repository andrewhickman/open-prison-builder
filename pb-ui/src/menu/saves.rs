use anyhow::Result;
use bevy::{
    ecs::system::{CommandQueue, SystemState},
    prelude::*,
};
use bevy_mod_picking::prelude::*;
use smol_str::SmolStr;

use pb_assets::Assets;
use pb_save::{
    save::{load, save, LoadParam, SaveMetadata, SaveParam},
    store::{DynStore, Store},
};
use pb_util::{
    callback::CallbackSender, run_oneshot_system, run_oneshot_system_with_input, spawn_io, try_res,
    AsDynError,
};

use crate::{
    layout::Layout,
    menu::{MenuPanel, MenuState},
    message::Message,
    theme::Theme,
    widget::{
        form::{self, Form, FormField, FormSubmit},
        UiBuilder,
    },
    EngineState,
};

#[derive(Clone, Copy, Debug)]
enum SaveAction {
    Save,
    Load,
}

#[derive(Debug, Clone, Reflect)]
struct SaveForm {
    name: String,
}

pub fn save_panel_button(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<Entity, With<MenuPanel>>,
    callback: Res<CallbackSender>,
    store: Res<DynStore>,
) {
    for id in &panel_q {
        commands.entity(id).despawn_recursive();
    }

    UiBuilder::new(commands, layout.menu)
        .save_panel(&theme, &assets, store.clone(), callback.clone())
        .insert(MenuPanel::Save);
}

pub fn load_panel_button(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<Entity, With<MenuPanel>>,
    callback: Res<CallbackSender>,
    store: Res<DynStore>,
) {
    for id in &panel_q {
        commands.entity(id).despawn_recursive();
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
    store: Res<DynStore>,
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
    mut commands: Commands,
    event: Listener<Pointer<Click>>,
    save_q: Query<&SaveMetadata>,
    mut menu_state: ResMut<NextState<MenuState>>,
    engine_state: Res<State<EngineState>>,
    mut next_engine_state: ResMut<NextState<EngineState>>,
    callback: Res<CallbackSender>,
    store: Res<DynStore>,
) {
    let save_name = try_res!(save_q.get(event.target())).name.clone();

    if let &EngineState::Running(root) = engine_state.get() {
        commands.entity(root).despawn_recursive();
    }

    menu_state.set(MenuState::Hidden);
    next_engine_state.set(EngineState::Loading);

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store.load_save(save_name.clone()).await;
        callback.send_oneshot_system_with_input(on_load_complete, res);
    });

    fn on_load_complete(
        In(scene): In<Result<DynamicScene>>,
        world: &mut World,
        load_p: &mut SystemState<LoadParam>,
        state: &mut SystemState<(
            ResMut<NextState<MenuState>>,
            ResMut<NextState<EngineState>>,
            EventWriter<Message>,
        )>,
    ) {
        let res = scene.and_then(|scene| load(world, load_p, scene));
        let (mut menu_state, mut engine_state, mut message_e) = state.get_mut(world);

        match res {
            Ok(root) => {
                engine_state.set(EngineState::Running(root));
                info!("Successfully loaded save");
            }
            Err(error) => {
                let error = error.as_dyn_error();
                error!(error, "Failed to load save");

                menu_state.set(MenuState::Shown);
                engine_state.set(EngineState::Disabled);

                message_e.send(Message::error(&error));
            }
        }
    }
}

fn overwrite_button(
    world: &World,
    event: Listener<Pointer<Click>>,
    save_q: Query<&SaveMetadata>,
    save_p: SaveParam,
    state: Res<State<EngineState>>,
    store: Res<DynStore>,
    callback: Res<CallbackSender>,
) {
    let save_name = try_res!(save_q.get(event.target())).name.clone();
    save_impl(
        save_name,
        world,
        save_p,
        state,
        store.clone(),
        callback.clone(),
    );
}

fn save_button(
    world: &World,
    event: Listener<FormSubmit>,
    form_q: Query<&Form>,
    save_p: SaveParam,
    state: Res<State<EngineState>>,
    store: Res<DynStore>,
    callback: Res<CallbackSender>,
) {
    let save_form = try_res!(form_q.get(event.listener()))
        .value::<SaveForm>()
        .unwrap();
    save_impl(
        SmolStr::from(&save_form.name),
        world,
        save_p,
        state,
        store.clone(),
        callback.clone(),
    );
}

fn save_impl(
    name: SmolStr,
    world: &World,
    save_p: SaveParam,
    state: Res<State<EngineState>>,
    store: DynStore,
    callback: CallbackSender,
) {
    if name.is_empty() {
        return;
    }

    let &EngineState::Running(root) = state.get() else {
        error!("Failed to save: not running");
        return;
    };

    let scene = save(world, &save_p, root);

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store
            .store_save(SaveMetadata::new(name.clone()), scene)
            .await;

        let mut queue = CommandQueue::default();
        queue.push(run_oneshot_system_with_input(on_save_complete, (name, res)));
        queue.push(run_oneshot_system(refresh_save_panel));
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

impl<'w, 's> UiBuilder<'w, 's> {
    fn save_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        store: DynStore,
        callback: CallbackSender,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.titled_panel(theme, assets, "Save prison");
        panel.saves_table(theme, store, callback, SaveAction::Save);

        let mut save_form = panel.form(
            Style {
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
        save_form.insert(On::<FormSubmit>::run(save_button));

        save_form.spawn(TextBundle::from_section("Name", theme.normal_text.clone()));
        save_form.input(theme).insert(FormField::new("name"));
        save_form.button(theme, assets, "Save", default(), form::submit);

        panel
    }

    fn load_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        store: DynStore,
        callback: CallbackSender,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.titled_panel(theme, assets, "Load prison");
        panel.saves_table(theme, store, callback, SaveAction::Load);
        panel
    }

    fn saves_table(
        &mut self,
        theme: &Theme,
        store: DynStore,
        callback: CallbackSender,
        action: SaveAction,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.container(Style {
            flex_grow: 1.,
            align_self: AlignSelf::Stretch,
            min_width: Val::Px(425.),
            ..default()
        });
        let container_id = container.id();

        container.spinner(theme, theme.large_icon_size_px);

        spawn_io(async move {
            let res = store.list_saves().await;
            callback.send_oneshot_system_with_input(on_list_complete, (res, container_id, action));
        });

        fn on_list_complete(
            In((res, container_id, action)): In<(Result<Vec<SaveMetadata>>, Entity, SaveAction)>,
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
        items: Vec<SaveMetadata>,
        action: SaveAction,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.container(Style {
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

        container.spawn(
            TextBundle::from_section("Name", theme.emphasis_text.clone()).with_style(Style {
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(1),
                ..default()
            }),
        );
        container.spawn(
            TextBundle::from_section("Modified", theme.emphasis_text.clone()).with_style(Style {
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(2),
                ..default()
            }),
        );

        for (row, item) in items.into_iter().enumerate() {
            container.spawn(
                TextBundle::from_section(item.name.clone(), theme.normal_text.clone()).with_style(
                    Style {
                        grid_row: GridPlacement::start(row as i16 + 2),
                        grid_column: GridPlacement::start(1),
                        ..default()
                    },
                ),
            );
            container.spawn(
                TextBundle::from_section(
                    item.modified_local().to_rfc2822(),
                    theme.normal_text.clone(),
                )
                .with_style(Style {
                    grid_row: GridPlacement::start(row as i16 + 2),
                    grid_column: GridPlacement::start(2),
                    ..default()
                }),
            );

            let mut button = match action {
                SaveAction::Save => container.button(
                    theme,
                    assets,
                    "Overwrite",
                    Style {
                        grid_row: GridPlacement::start(row as i16 + 2),
                        grid_column: GridPlacement::start(3),
                        ..default()
                    },
                    overwrite_button,
                ),
                SaveAction::Load => container.button(
                    theme,
                    assets,
                    "Load",
                    Style {
                        grid_row: GridPlacement::start(row as i16 + 2),
                        grid_column: GridPlacement::start(3),
                        ..default()
                    },
                    load_button,
                ),
            };

            button.insert(item);
        }

        container
    }
}
