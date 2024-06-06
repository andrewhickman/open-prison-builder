#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod diagnostic;
mod window;

use bevy::{
    app::{App, PluginGroup, Startup},
    asset::AssetMetaCheck,
    DefaultPlugins,
};

use pb_assets::AssetsPlugin;
use pb_engine::EnginePlugin;
use pb_render::RenderPlugin;
use pb_store::StorePlugin;
use pb_ui::UiPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(AssetMetaCheck::Never);
    app.add_plugins(DefaultPlugins.set(window::plugin()))
        .add_systems(Startup, window::set_icon);

    #[cfg(feature = "dev")]
    app.add_plugins(diagnostic::DiagnosticsPlugin);

    app.add_plugins((
        AssetsPlugin,
        EnginePlugin,
        RenderPlugin,
        StorePlugin,
        UiPlugin,
    ));

    app.run()
}
