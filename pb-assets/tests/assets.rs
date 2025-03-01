use bevy::{
    asset::{AssetPlugin, LoadState},
    core_pipeline::CorePipelinePlugin,
    log::LogPlugin,
    prelude::*,
    render::RenderPlugin,
    sprite::SpritePlugin,
    text::TextPlugin,
    window::ExitCondition,
};

use pb_assets::{AssetHandles, PbAssetsPlugin};

#[test]
fn assets() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        WindowPlugin {
            primary_window: None,
            exit_condition: ExitCondition::DontExit,
            ..default()
        },
        AssetPlugin {
            file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/../assets").to_owned(),
            ..default()
        },
        TextPlugin,
        RenderPlugin::default(),
        ImagePlugin::default(),
        CorePipelinePlugin,
        SpritePlugin { add_picking: false },
        LogPlugin::default(),
    ))
    .add_plugins(PbAssetsPlugin)
    .add_systems(Update, update);
    let exit_code = app.run();

    assert!(exit_code.is_success());
}

fn update(
    assets: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut exit_e: EventWriter<AppExit>,
) {
    match assets.load_state(&asset_server) {
        LoadState::NotLoaded | LoadState::Loading => info!("Waiting for assets to load..."),
        LoadState::Loaded => {
            info!("All assets loaded successfully");
            exit_e.send(AppExit::Success);
        }
        LoadState::Failed(error) => {
            error!("Failed to load all assets, exiting: {error}");
            exit_e.send(AppExit::error());
        }
    }
}
