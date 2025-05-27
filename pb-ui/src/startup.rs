use bevy::{app::AppExit, prelude::*};

use pb_assets::AssetHandles;
use pb_store::Store;
use pb_util::callback::{CallbackSender, spawn_io};

use crate::{UiState, input::Settings};

pub fn init(
    assets: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    callback: Res<CallbackSender>,
    store: Res<Store>,
    mut ui_state: ResMut<NextState<UiState>>,
) {
    let assets = assets.clone();
    let asset_server = asset_server.clone();
    let callback = callback.clone();
    let store = store.clone();

    ui_state.set(UiState::LoadingAssets);

    spawn_io(async move {
        let settings = Settings::load(&store).await;
        let asset_load_result = assets.wait_for_load(&asset_server).await;

        callback.run_system_cached_with(on_startup_complete, (settings, asset_load_result));
    });

    fn on_startup_complete(
        In((settings, asset_load_result)): In<(Settings, Result)>,
        mut commands: Commands,
        mut ui_state: ResMut<NextState<UiState>>,
        mut exit_e: EventWriter<AppExit>,
    ) {
        if let Err(error) = asset_load_result {
            error!("Failed to load all assets, exiting: {error}");
            exit_e.write(AppExit::error());
        } else {
            info!("Finished startup");
            commands.insert_resource(settings);
            ui_state.set(UiState::Menu);
        }
    }
}
