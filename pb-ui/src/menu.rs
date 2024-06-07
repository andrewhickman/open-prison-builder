use bevy::{app::AppExit, prelude::*};

use pb_assets::Assets;
use pb_engine::pawn::PawnBundle;
use pb_util::try_res;

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

impl<'a> UiBuilder<'a> {
    pub fn main_menu(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'_> {
        let mut container = self.spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: theme.gutter,
                ..default()
            },
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

        menu.large_button(theme, "New Prison", new)
            .with(MenuButton::New);
        menu.large_button(theme, "Save Prison", noop)
            .with(MenuButton::Load);
        menu.large_button(theme, "Load Prison", load)
            .with(MenuButton::Load);
        menu.large_button(theme, "Settings", noop)
            .with(MenuButton::Settings);
        menu.large_button(theme, "Exit", exit)
            .with(MenuButton::Exit);

        let mut icon_bar = menu.spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::RowReverse,
                column_gap: theme.gutter,
                ..default()
            },
            ..default()
        });

        icon_bar
            .large_icon_button(theme, assets.bevy_icon_image.clone(), || {
                open_url("https://bevyengine.org/")
            })
            .with(MenuButton::OpenBevy);
        icon_bar
            .large_icon_button(theme, assets.github_icon_image.clone(), || {
                open_url("https://github.com/andrewhickman/open-prison-builder/")
            })
            .with(MenuButton::OpenGithub);

        container
    }

    fn load_panel(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'_> {
        let panel = self.titled_panel(theme, assets, "Load prison");

        panel
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
) {
    for panel in &panel_q {
        commands.entity(panel).despawn_recursive();
    }

    UiBuilder::new(commands, nodes.menu)
        .load_panel(&theme, &assets)
        .with(MenuPanel);
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
