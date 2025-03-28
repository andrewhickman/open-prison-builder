#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "dev")]
mod diagnostic;
mod window;

use bevy::{asset::AssetMetaCheck, prelude::*};

use pb_assets::PbAssetsPlugin;
use pb_engine::PbEnginePlugin;
use pb_render::PbRenderPlugin;
use pb_store::PbStorePlugin;
use pb_ui::PbUiPlugin;
use pb_util::CallbackPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(window::plugin()).set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    }))
    .add_systems(Startup, window::set_icon);

    #[cfg(feature = "dev")]
    app.add_plugins(diagnostic::DiagnosticsPlugin);

    app.add_plugins((
        CallbackPlugin,
        PbAssetsPlugin,
        PbEnginePlugin,
        PbRenderPlugin,
        PbStorePlugin,
        PbUiPlugin,
    ));

    app.run()
}
