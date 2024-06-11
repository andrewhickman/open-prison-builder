#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod diagnostic;
mod window;

use bevy::{asset::AssetMetaCheck, prelude::*};

use pb_assets::AssetsPlugin;
use pb_engine::EnginePlugin;
use pb_render::RenderPlugin;
use pb_save::SavePlugin;
use pb_ui::UiPlugin;
use pb_util::CallbackPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(AssetMetaCheck::Never)
        .insert_resource(Msaa::Sample4);
    app.add_plugins(DefaultPlugins.set(window::plugin()))
        .add_systems(Startup, window::set_icon);

    #[cfg(feature = "dev")]
    app.add_plugins(diagnostic::DiagnosticsPlugin);

    app.add_plugins((
        CallbackPlugin,
        AssetsPlugin,
        EnginePlugin,
        RenderPlugin,
        SavePlugin,
        UiPlugin,
    ));

    app.run()
}
