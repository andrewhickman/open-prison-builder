use anyhow::Result;
use bevy::{app::AppExit, prelude::*};

use pb_assets::Assets;
use pb_engine::pawn::PawnBundle;
use pb_store::{DynStore, SaveMetadata, Store};
use pb_util::{callback::CallbackSender, spawn_io, try_res, AsDynError};

use crate::{node::Nodes, theme::Theme, widget::UiBuilder};

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

#[derive(Component)]
pub struct MenuPanel;

pub fn show(nodes: Res<Nodes>, mut style_q: Query<&mut Style>) {
    let mut style = try_res!(style_q.get_mut(nodes.menu));
    style.display = Display::Flex;
}

pub fn hide(nodes: Res<Nodes>, mut style_q: Query<&mut Style>) {
    let mut style = try_res!(style_q.get_mut(nodes.menu));
    style.display = Display::None;
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn main_menu(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        let mut container = self.container(Style {
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

        menu.large_button(theme, assets, "New Prison", default(), new)
            .insert(MenuButton::New);
        menu.large_button(theme, assets, "Save Prison", default(), noop)
            .insert(MenuButton::Load);
        menu.large_button(theme, assets, "Load Prison", default(), load)
            .insert(MenuButton::Load);
        menu.large_button(theme, assets, "Settings", default(), noop)
            .insert(MenuButton::Settings);
        menu.large_button(theme, assets, "Exit", default(), exit)
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

    fn load_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        store: DynStore,
        callback: CallbackSender,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.titled_panel(theme, assets, "Load prison");
        panel.insert(MenuPanel);
        let panel_id = panel.id();

        let mut spinner_container = panel.container(Style {
            flex_grow: 1.,
            align_self: AlignSelf::Stretch,
            ..default()
        });
        spinner_container.spinner(theme, theme.large_icon_size_px);
        let spinner_container_id = spinner_container.id();

        spawn_io(async move {
            let res = store.list().await;
            callback.send_oneshot_system_with_input(
                move |In(res): In<Result<Vec<SaveMetadata>>>,
                      mut commands: Commands,
                      theme: Res<Theme>,
                      assets: Res<Assets>| {
                    let Some(mut panel) = commands.get_entity(panel_id) else {
                        return;
                    };

                    panel
                        .commands()
                        .entity(spinner_container_id)
                        .despawn_recursive();

                    let mut builder = UiBuilder::from(&mut panel);

                    match res {
                        Ok(items) => {
                            builder.load_panel_items(&theme, &assets, items);
                        }
                        Err(error) => {
                            let error = error.as_dyn_error();
                            error!(error, "failed to load saves");
                            builder.error_message(&theme, &assets, error);
                        }
                    }
                },
                res,
            );
        });

        panel
    }

    fn load_panel_items(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        items: Vec<SaveMetadata>,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.container(Style {
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::fr(1.), GridTrack::auto(), GridTrack::auto()],
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
            container.button(
                theme,
                assets,
                "Load",
                Style {
                    grid_row: GridPlacement::start(row as i16 + 2),
                    grid_column: GridPlacement::start(3),
                    ..default()
                },
                move || info!("Load save {}", item.name),
            );
        }

        container
    }
}

fn new(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Hidden);

    commands.spawn(PawnBundle::new(Vec2::default()));
}

fn load(
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<Assets>,
    nodes: Res<Nodes>,
    panel_q: Query<Entity, With<MenuPanel>>,
    callback: Res<CallbackSender>,
    store: Res<DynStore>,
) {
    for panel in &panel_q {
        commands.entity(panel).despawn_recursive();
    }

    UiBuilder::new(commands, nodes.menu).load_panel(
        &theme,
        &assets,
        store.clone(),
        callback.clone(),
    );
}

fn noop() {}

fn exit(mut exit_e: EventWriter<AppExit>) {
    exit_e.send(AppExit);
}

fn open_url(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        error!("Failed to open url {url}: {err}");
    }
}
