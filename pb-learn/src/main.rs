#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rl_link;
mod learn;

use bevy::prelude::*;

use pb_assets::PbAssetsPlugin;
use pb_engine::PbEnginePlugin;
use pb_render::{projection::projection, PbRenderPlugin};
use pb_util::CallbackPlugin;
use learn::PbLearnPlugin;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(AssetPlugin {
            file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/../assets").to_owned(),
            ..default()
        }),
        CallbackPlugin,
        PbAssetsPlugin,
        PbEnginePlugin,
        PbRenderPlugin,
        PbLearnPlugin,
    ));

    app.world_mut().spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Srgba::hex("192a28").unwrap().into()),
            ..Default::default()
        },
        projection(),
        Msaa::Off,
    ));

    app.run()
}
