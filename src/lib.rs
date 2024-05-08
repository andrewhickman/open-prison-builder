#![allow(clippy::type_complexity)]

mod camera;
mod loading;
mod map;
mod material;
mod menu;
mod theme;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use crate::camera::InputPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::theme::ThemePlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Loading,
    Menu,
    Running,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            InputPlugin,
            LoadingPlugin,
            MenuPlugin,
            ThemePlugin,
            MapPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
