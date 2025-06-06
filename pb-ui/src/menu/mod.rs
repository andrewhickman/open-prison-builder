mod saves;

use bevy::{app::AppExit, prelude::*};

use pb_assets::AssetHandles;
use pb_engine::{EngineState, map::Map, root::Root};

use crate::{
    UiState,
    layout::Layout,
    theme::Theme,
    widget::{UiBuilder, disabled::Disabled},
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

pub fn show(commands: Commands, layout: Res<Layout>, theme: Res<Theme>, assets: Res<AssetHandles>) {
    UiBuilder::new(commands, layout.menu).menu(&theme, &assets);
}

pub fn hide(mut commands: Commands, layout: Res<Layout>) {
    commands.entity(layout.menu).despawn_related::<Children>();
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

    pub fn menu(&mut self, theme: &Theme, assets: &AssetHandles) -> UiBuilder<'w, '_> {
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
        menu.named("pb_ui::menu::menu_panel");

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
            .on_click(|_: Trigger<'_, Pointer<Click>>| {
                open_url("https://bevyengine.org/");
                Ok(())
            });
        icon_bar
            .icon_button(theme, assets.github_icon.clone(), theme.large_icon_size())
            .insert(MenuButton::OpenGithub)
            .on_click(|_: Trigger<'_, Pointer<Click>>| {
                open_url("https://github.com/andrewhickman/open-prison-builder/");
                Ok(())
            });

        self.reborrow()
    }

    fn settings_panel(&mut self, theme: &Theme, assets: &AssetHandles) -> UiBuilder<'w, '_> {
        self.menu_panel(theme, assets, "Settings")
    }

    pub fn menu_panel(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
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
    engine_state: Res<State<EngineState>>,
    mut next_engine_state: ResMut<NextState<EngineState>>,
) -> Result {
    if let &EngineState::Running(root) = engine_state.get() {
        commands.entity(root).despawn();
    }

    let root = commands.spawn((Root, children![Map::new()])).id();
    ui_state.set(UiState::Game);
    next_engine_state.set(EngineState::Running(root));
    Ok(())
}

fn settings_panel_button(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    theme: Res<Theme>,
    assets: Res<AssetHandles>,
    layout: Res<Layout>,
    panel_q: Query<(Entity, &MenuPanel)>,
) -> Result {
    for (id, &panel) in &panel_q {
        commands.entity(id).despawn();
        if panel == MenuPanel::Settings {
            return Ok(());
        }
    }

    UiBuilder::new(commands, layout.menu)
        .settings_panel(&theme, &assets)
        .insert(MenuPanel::Settings);
    Ok(())
}

fn exit_button(_: Trigger<Pointer<Click>>, mut exit_e: EventWriter<AppExit>) -> Result {
    info!("Exiting application");
    exit_e.write(AppExit::Success);
    Ok(())
}

fn open_url(url: &str) {
    info!("Opening url '{url}'");
    if let Err(err) = webbrowser::open(url) {
        error!("Failed to open url {url}: {err}");
    }
}
