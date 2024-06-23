mod saves;

use bevy::{app::AppExit, prelude::*};

use pb_assets::Assets;
use pb_engine::{
    pawn::PawnBundle,
    wall::{VertexBundle, Wall},
    EngineState, RootBundle,
};

use crate::{
    layout::Layout,
    theme::Theme,
    widget::{Disabled, UiBuilder},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum MenuState {
    Shown,
    #[default]
    Hidden,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub enum MenuButton {
    New,
    Save,
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

pub fn show(commands: Commands, layout: Res<Layout>, theme: Res<Theme>, assets: Res<Assets>) {
    UiBuilder::new(commands, layout.menu).menu(&theme, &assets);
}

pub fn hide(mut commands: Commands, layout: Res<Layout>) {
    commands.entity(layout.menu).despawn_descendants();
}

pub fn update(mut button_q: Query<(&MenuButton, &mut Disabled)>, state: Res<State<EngineState>>) {
    for (&button, mut disabled) in &mut button_q {
        match button {
            MenuButton::New
            | MenuButton::Load
            | MenuButton::Settings
            | MenuButton::Exit
            | MenuButton::OpenBevy
            | MenuButton::OpenGithub => {
                disabled.set_if_neq(Disabled::ENABLED);
            }
            MenuButton::Save => match state.get() {
                EngineState::Running(_) => {
                    disabled.set_if_neq(Disabled::ENABLED);
                }
                _ => {
                    disabled.set_if_neq(Disabled::DISABLED);
                }
            },
        }
    }
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn menu_root(&mut self, theme: &Theme) -> UiBuilder<'w, '_> {
        self.container(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Stretch,
            column_gap: theme.gutter,
            ..default()
        })
    }

    pub fn menu(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        let mut menu = self.empty_panel(
            theme,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Start,
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
            saves::save_panel_button,
        )
        .insert(MenuButton::Save);
        menu.large_button(
            theme,
            assets,
            "Load Prison",
            default(),
            saves::load_panel_button,
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

        self.reborrow()
    }

    fn settings_panel(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        self.panel(theme, assets, "Settings")
    }
}

fn new_prison_button(
    mut commands: Commands,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut engine_state: ResMut<NextState<EngineState>>,
) {
    let parent = commands.spawn(RootBundle::default()).id();

    menu_state.set(MenuState::Hidden);
    engine_state.set(EngineState::Running(parent));

    commands
        .spawn(PawnBundle::new(Vec2::default()))
        .set_parent(parent);
    let v1 = commands
        .spawn(VertexBundle::new(Vec2::new(4.0, -4.0)))
        .set_parent(parent)
        .id();
    let v2 = commands
        .spawn(VertexBundle::new(Vec2::new(-2.0, -2.0)))
        .set_parent(parent)
        .id();
    let v3 = commands
        .spawn(VertexBundle::new(Vec2::new(-2.0, 2.0)))
        .set_parent(parent)
        .id();
    let v4 = commands
        .spawn(VertexBundle::new(Vec2::new(7.0, 7.0)))
        .set_parent(parent)
        .id();
    let v5 = commands
        .spawn(VertexBundle::new(Vec2::new(13.0, 15.0)))
        .set_parent(parent)
        .id();

    commands
        .spawn(Wall { start: v1, end: v2 })
        .set_parent(parent);
    commands
        .spawn(Wall { start: v2, end: v3 })
        .set_parent(parent);
    commands
        .spawn(Wall { start: v3, end: v4 })
        .set_parent(parent);
    commands
        .spawn(Wall { start: v4, end: v1 })
        .set_parent(parent);
    commands
        .spawn(Wall { start: v4, end: v5 })
        .set_parent(parent);
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
    exit_e.send(AppExit::Success);
}

fn open_url(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        error!("Failed to open url {url}: {err}");
    }
}
