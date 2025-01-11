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
    widget::{disabled::Disabled, UiBuilder},
    UiState,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MenuState {
    Shown,
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

impl ComputedStates for MenuState {
    type SourceStates = UiState;

    fn compute(source: UiState) -> Option<Self> {
        match source {
            UiState::Startup => None,
            UiState::LoadingAssets | UiState::LoadingSave | UiState::Game => {
                Some(MenuState::Hidden)
            }
            UiState::Menu => Some(MenuState::Shown),
        }
    }
}

impl<'w> UiBuilder<'w, '_> {
    pub fn menu_root(&mut self, theme: &Theme) -> UiBuilder<'w, '_> {
        self.container(Node {
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
        let mut menu = self.panel(
            theme,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Start,
                padding: UiRect::all(theme.gutter),
                row_gap: theme.gutter,
                ..default()
            },
        );

        menu.large_button(theme, assets, "New Prison", default())
            .insert((MenuButton::New, Disabled::ENABLED))
            .on_click(new_prison_button);
        menu.large_button(theme, assets, "Save Prison", default())
            .insert((MenuButton::Save, Disabled::ENABLED))
            .on_click(saves::save_panel_button);
        menu.large_button(theme, assets, "Load Prison", default())
            .insert((MenuButton::Load, Disabled::ENABLED))
            .on_click(saves::load_panel_button);
        menu.large_button(theme, assets, "Settings", default())
            .insert((MenuButton::Settings, Disabled::ENABLED))
            .on_click(settings_panel_button);
        menu.large_button(theme, assets, "Exit", default())
            .insert((MenuButton::Exit, Disabled::ENABLED))
            .on_click(exit_button);

        let mut icon_bar = menu.container(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::RowReverse,
            column_gap: theme.gutter,
            ..default()
        });

        icon_bar
            .icon_button(theme, assets.bevy_icon.clone(), theme.large_icon_size())
            .insert(MenuButton::OpenBevy)
            .on_click(|_: Trigger<'_, Pointer<Click>>| open_url("https://bevyengine.org/"));
        icon_bar
            .icon_button(theme, assets.github_icon.clone(), theme.large_icon_size())
            .insert(MenuButton::OpenGithub)
            .on_click(|_: Trigger<'_, Pointer<Click>>| {
                open_url("https://github.com/andrewhickman/open-prison-builder/")
            });

        self.reborrow()
    }

    fn settings_panel(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        self.menu_panel(theme, assets, "Settings")
    }

    pub fn menu_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        title: impl Into<String>,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.panel(
            theme,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: theme.gutter,
                ..default()
            },
        );
        panel.cancellable();
        let panel_id = panel.id();

        let mut title_row = panel.container(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: theme.gutter,
            ..default()
        });

        title_row.spawn((Text::new(title), theme.header_text.clone()));
        title_row.panel_close_button(theme, assets, panel_id);

        panel
    }
}

fn new_prison_button(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut ui_state: ResMut<NextState<UiState>>,
    mut engine_state: ResMut<NextState<EngineState>>,
) {
    let parent = commands.spawn(RootBundle::default()).id();

    ui_state.set(UiState::Game);
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
}

fn settings_panel_button(
    _: Trigger<Pointer<Click>>,
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

fn exit_button(_: Trigger<Pointer<Click>>, mut exit_e: EventWriter<AppExit>) {
    info!("Exiting application");
    exit_e.send(AppExit::Success);
}

fn open_url(url: &str) {
    info!("Opening url '{url}'");
    if let Err(err) = webbrowser::open(url) {
        error!("Failed to open url {url}: {err}");
    }
}
