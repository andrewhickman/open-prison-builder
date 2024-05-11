#![allow(clippy::type_complexity)]

mod control;
mod loading;
mod map;
mod material;
mod menu;
mod theme;
mod ui;

pub use crate::theme::Theme;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use control::ControlPlugin;
use ui::UiPlugin;

use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::theme::ThemePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    Loading,
    LoadingFailed,
    Menu,
    Running,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            ControlPlugin,
            LoadingPlugin,
            MapPlugin,
            MenuPlugin,
            ThemePlugin,
            UiPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
