mod saves;

use bevy::{app::AppExit, prelude::*};

use pb_assets::Assets;
use pb_engine::{pawn::PawnBundle, EngineState, RootBundle};
use pb_util::try_res;

use crate::{
    input::ToggleMenuCommand,
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

pub fn show(layout: Res<Layout>, mut style_q: Query<&mut Style>) {
    let mut style = try_res!(style_q.get_mut(layout.menu));
    style.display = Display::Flex;
}

pub fn hide(
    mut commands: Commands,
    layout: Res<Layout>,
    mut style_q: Query<&mut Style>,
    panel_q: Query<Entity, With<MenuPanel>>,
) {
    let mut style = try_res!(style_q.get_mut(layout.menu));
    style.display = Display::None;

    for id in &panel_q {
        commands.entity(id).despawn_recursive();
    }
}

pub fn toggle(
    mut toggle_e: EventReader<ToggleMenuCommand>,
    state: Res<State<MenuState>>,
    engine_state: Res<State<EngineState>>,
    mut next_state: ResMut<NextState<MenuState>>,
) {
    for ToggleMenuCommand in toggle_e.read() {
        if !matches!(engine_state.get(), EngineState::Running(_)) {
            continue;
        }

        let toggled_state = match state.get() {
            MenuState::Shown => MenuState::Hidden,
            MenuState::Hidden => MenuState::Shown,
        };

        next_state.set(toggled_state);
    }
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
    pub fn menu(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        let mut container = self.container(Style {
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Stretch,
            column_gap: theme.gutter,
            ..default()
        });

        let mut menu = container.panel(
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

        container
    }

    fn settings_panel(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        self.titled_panel(theme, assets, "Settings")
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
