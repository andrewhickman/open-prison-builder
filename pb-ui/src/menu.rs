use anyhow::Result;
use bevy::{app::AppExit, prelude::*};

use bevy_mod_picking::prelude::*;
use pb_assets::Assets;
use pb_engine::pawn::PawnBundle;
use pb_store::{DynStore, SaveMetadata, Store};
use pb_util::{callback::CallbackSender, spawn_io, try_res, AsDynError};

use crate::{layout::Layout, message::Message, theme::Theme, widget::UiBuilder, EngineState};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum MenuState {
    Shown,
    #[default]
    Hidden,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub enum MenuButton {
    New,
    Load,
    Settings,
    Exit,
    OpenBevy,
    OpenGithub,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub enum MenuPanel {
    Load,
    Save,
    Settings,
}

#[derive(Clone, Copy, Debug)]
enum SaveAction {
    Save,
    Load,
}

pub fn show(layout: Res<Layout>, mut style_q: Query<&mut Style>) {
    let mut style = try_res!(style_q.get_mut(layout.menu));
    style.display = Display::Flex;
}

pub fn hide(layout: Res<Layout>, mut style_q: Query<&mut Style>) {
    let mut style = try_res!(style_q.get_mut(layout.menu));
    style.display = Display::None;
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn menu(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        let mut container = self.container(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: theme.gutter,
            ..default()
        });

        let mut menu = container.panel(
            theme,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(theme.gutter),
                row_gap: theme.gutter,
                ..default()
            },
        );

        menu.large_button(theme, assets, "New Prison", default(), new_prison_button)
            .insert(MenuButton::New);
        menu.large_button(
            theme,
            assets,
            "Save Prison",
            default(),
            save_prison_panel_button,
        )
        .insert(MenuButton::Load);
        menu.large_button(
            theme,
            assets,
            "Load Prison",
            default(),
            load_prison_panel_button,
        )
        .insert(MenuButton::Load);
        menu.large_button(theme, assets, "Settings", default(), settings_panel_button)
            .insert(MenuButton::Settings);
        menu.large_button(theme, assets, "Exit", default(), exit_button)
            .insert(MenuButton::Exit);

        let mut icon_bar = menu.container(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::RowReverse,
            column_gap: theme.gutter,
            ..default()
        });

        icon_bar
            .icon_button(assets.bevy_icon.clone(), theme.large_icon_size(), || {
                open_url("https://bevyengine.org/")
            })
            .insert(MenuButton::OpenBevy);
        icon_bar
            .icon_button(assets.github_icon.clone(), theme.large_icon_size(), || {
                open_url("https://github.com/andrewhickman/open-prison-builder/")
            })
            .insert(MenuButton::OpenGithub);

        container
    }

    fn save_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        store: DynStore,
        callback: CallbackSender,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.titled_panel(theme, assets, "Save prison");
        panel.saves_table(theme, store, callback, SaveAction::Save);

        let mut save_row = panel.container(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: theme.gutter,
            align_items: AlignItems::Center,
            ..default()
        });

        save_row.spawn(TextBundle::from_section("Name", theme.normal_text.clone()));
        save_row.input(theme);
        save_row.button(theme, assets, "Save", default(), move || {
            info!("Save save with input name")
        });

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
            min_width: Val::Px(400.),
            ..default()
        });
        let container_id = container.id();

        container.spinner(theme, theme.large_icon_size_px);

        spawn_io(async move {
            let res = store.list().await;
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

            container.despawn_descendants();

            let mut builder = UiBuilder::from(&mut container);

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
            grid_template_columns: vec![
                GridTrack::minmax(
                    MinTrackSizingFunction::Auto,
                    MaxTrackSizingFunction::Fraction(1.),
                ),
                GridTrack::minmax(
                    MinTrackSizingFunction::Auto,
                    MaxTrackSizingFunction::Fraction(1.),
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
                    move || info!("{action:?} save"),
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
                    load_prison_button,
                ),
            };

            button.insert(item);
        }

        container
    }

    fn settings_panel(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        self.titled_panel(theme, assets, "Settings")
    }
}

fn new_prison_button(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Hidden);

    commands.spawn(PawnBundle::new(Vec2::default()));
}

fn save_prison_panel_button(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<DynStore>,
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

fn load_prison_panel_button(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
    callback: Res<CallbackSender>,
    store: Res<DynStore>,
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

fn load_prison_button(
    event: Listener<Pointer<Click>>,
    save_q: Query<&SaveMetadata>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut engine_state: ResMut<NextState<EngineState>>,
    callback: Res<CallbackSender>,
    store: Res<DynStore>,
) {
    let save_name = try_res!(save_q.get(event.target())).name.clone();

    menu_state.set(MenuState::Hidden);
    engine_state.set(EngineState::Loading);

    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store.load(save_name).await;
        callback.send_oneshot_system_with_input(on_load_complete, res);
    });

    fn on_load_complete(
        In(res): In<Result<DynamicScene>>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut engine_state: ResMut<NextState<EngineState>>,
        assets: Res<AssetServer>,
        mut spawner: ResMut<SceneSpawner>,
        mut message_e: EventWriter<Message>,
    ) {
        match res {
            Ok(scene) => {
                engine_state.set(EngineState::Running);

                spawner.spawn_dynamic(assets.add(scene));
            }
            Err(error) => {
                menu_state.set(MenuState::Shown);
                engine_state.set(EngineState::Disabled);

                let error = error.as_dyn_error();
                error!(error, "failed to load saves");

                message_e.send(Message::error(&error));
            }
        }
    }
}

fn settings_panel_button(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
) {
    for (id, &panel) in &panel_q {
        commands.entity(id).despawn_recursive();
        if panel == MenuPanel::Settings {
            return;
        }
    }

    UiBuilder::new(commands, layout.menu)
        .settings_panel(&theme, &assets)
        .insert(MenuPanel::Settings);
}

fn exit_button(mut exit_e: EventWriter<AppExit>) {
    exit_e.send(AppExit);
}

fn open_url(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        error!("Failed to open url {url}: {err}");
    }
}
