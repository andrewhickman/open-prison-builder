#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod diagnostic;
mod window;

use bevy::{
    app::{App, PluginGroup, Startup},
    DefaultPlugins,
};

use pb_engine::EnginePlugin;
use pb_render::RenderPlugin;
use pb_ui::UiPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(window::plugin()))
        .add_systems(Startup, window::set_icon);

    #[cfg(feature = "dev")]
    app.add_plugins(diagnostic::DiagnosticsPlugin);

    app.add_plugins((EnginePlugin, RenderPlugin, UiPlugin));

    app.run()
}
