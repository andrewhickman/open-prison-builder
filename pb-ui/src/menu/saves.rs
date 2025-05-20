use bevy::{
    ecs::{
        error::HandleError,
        system::command::{run_system_cached, run_system_cached_with},
        world::CommandQueue,
    },
    prelude::*,
};
use pb_engine::{
    EngineState,
    save::{SaveModel, SaveParam},
};
use pb_store::{Metadata, Store};

use pb_assets::AssetHandles;
use pb_util::callback::{CallbackSender, spawn_io};
use smol_str::SmolStr;

use crate::{
    UiState,
    layout::Layout,
    menu::MenuPanel,
    message::Message,
    theme::Theme,
    widget::{
        UiBuilder,
        form::{self, Form, FormField, FormSubmit},
    },
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
    assets: Res<AssetHandles>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) -> Result {
    for (id, &panel) in &panel_q {
        commands.entity(id).despawn();
        if panel == MenuPanel::Save {
            return Ok(());
        }
    }

    UiBuilder::new(commands, layout.menu)
        .save_panel(&theme, &assets, store.clone(), callback.clone())
        .insert(MenuPanel::Save);
    Ok(())
}

pub fn load_panel_button(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<AssetHandles>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) -> Result {
    for (id, &panel) in &panel_q {
        commands.entity(id).despawn();
        if panel == MenuPanel::Load {
            return Ok(());
        }
    }

    UiBuilder::new(commands, layout.menu)
        .load_panel(&theme, &assets, store.clone(), callback.clone())
        .insert(MenuPanel::Load);
    Ok(())
}

fn refresh_save_panel(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<AssetHandles>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) {
    if !panel_q.iter().any(|(_, &panel)| panel == MenuPanel::Save) {
        return;
    }

    for (id, _) in &panel_q {
        commands.entity(id).despawn();
    }

    UiBuilder::new(commands, layout.menu)
        .save_panel(&theme, &assets, store.clone(), callback.clone())
        .insert(MenuPanel::Save);
}

fn load_button(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    save_q: Query<&SaveItem>,
    mut ui_state: ResMut<NextState<UiState>>,
    engine_state: Res<State<EngineState>>,
    mut next_engine_state: ResMut<NextState<EngineState>>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
) -> Result {
    let save_name = save_q.get(trigger.target())?.0.name.clone();

    if let &EngineState::Running(root) = engine_state.get() {
        commands.entity(root).despawn();
    }

    ui_state.set(UiState::LoadingSave);
    next_engine_state.set(EngineState::Disabled);

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store
            .get::<SaveModel>(&format!("saves/{save_name}.json"))
            .await;
        callback.run_system_cached_with(on_load_complete, res);
    });

    fn on_load_complete(
        In(save): In<Result<SaveModel>>,
        mut commands: Commands,
        mut ui_state: ResMut<NextState<UiState>>,
        mut engine_state: ResMut<NextState<EngineState>>,
        mut message_e: EventWriter<Message>,
    ) {
        match save {
            Ok(save) => {
                let root = save.spawn(&mut commands);

                ui_state.set(UiState::Game);
                engine_state.set(EngineState::Running(root));
                info!("Successfully loaded save");
            }
            Err(error) => {
                error!("Failed to load save: {error}");

                ui_state.set(UiState::Menu);
                message_e.write(Message::error(&error));
            }
        }
    }

    Ok(())
}

fn overwrite_button(
    trigger: Trigger<Pointer<Click>>,
    save_q: Query<&SaveItem>,
    save_p: SaveParam,
    store: Res<Store>,
    callback: Res<CallbackSender>,
) -> Result {
    let save_name = save_q.get(trigger.target())?.0.name.clone();
    save_impl(save_name, save_p, store.clone(), callback.clone())
}

fn save_button(
    mut trigger: Trigger<FormSubmit>,
    form_q: Query<&Form>,
    save_p: SaveParam,
    store: Res<Store>,
    callback: Res<CallbackSender>,
) -> Result {
    trigger.propagate(false);

    let save_name = form_q.get(trigger.target())?.value::<SaveForm>()?.name;
    save_impl(save_name.into(), save_p, store.clone(), callback.clone())
}

fn save_impl(name: SmolStr, save_p: SaveParam, store: Store, callback: CallbackSender) -> Result {
    let scene = save_p.save()?;

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = if name.is_empty() {
            Err("empty name".into())
        } else {
            store.set(&format!("saves/{name}.json"), scene).await
        };

        let mut queue = CommandQueue::default();
        queue.push(run_system_cached_with(on_save_complete, (name, res)).handle_error());
        queue.push(run_system_cached(refresh_save_panel).handle_error());
        callback.send_batch(queue);
    });

    fn on_save_complete(
        In((name, res)): In<(SmolStr, Result<()>)>,
        mut message_e: EventWriter<Message>,
    ) {
        match res {
            Ok(()) => {
                info!("Successfully saved '{name}'");
                message_e.write(Message::info(format!("Successfully saved '{name}'")));
            }
            Err(error) => {
                error!("Failed to save: {error}");
                message_e.write(Message::error(&error));
            }
        }
    }

    Ok(())
}

impl<'w> UiBuilder<'w, '_> {
    fn save_panel(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
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
        assets: &AssetHandles,
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
            let res = store.iter("saves").await;
            callback.run_system_cached_with(on_list_complete, (res, container_id, action));
        });

        fn on_list_complete(
            In((res, container_id, action)): In<(Result<Vec<Metadata>>, Entity, SaveAction)>,
            mut commands: Commands,
            theme: Res<Theme>,
            assets: Res<AssetHandles>,
        ) {
            let Ok(mut container) = commands.get_entity(container_id) else {
                return;
            };

            let mut builder = UiBuilder::from(&mut container);
            builder.clear();

            match res {
                Ok(items) => {
                    builder.saves_table_items(&theme, &assets, items, action);
                }
                Err(error) => {
                    error!("failed to load saves: {error}");
                    builder.error_message(
                        &theme,
                        &assets,
                        error
                            .to_string()
                            .lines()
                            .next()
                            .unwrap_or_default()
                            .to_owned(),
                    );
                }
            }
        }

        self.reborrow()
    }

    fn saves_table_items(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
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
