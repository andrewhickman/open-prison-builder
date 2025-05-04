use bevy::{app::AppExit, asset::LoadState, prelude::*};

use pb_assets::AssetHandles;

use crate::{UiState, input::Settings};

pub fn update(
    mut state: ResMut<NextState<UiState>>,
    assets: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    settings: Option<Res<Settings>>,
    mut exit_e: EventWriter<AppExit>,
) {
    match assets.load_state(&asset_server) {
        LoadState::NotLoaded | LoadState::Loading => return,
        LoadState::Loaded => (),
        LoadState::Failed(error) => {
            error!("Failed to load all assets, exiting: {error}");
            exit_e.write(AppExit::error());
        }
    }

    if settings.is_none() {
        return;
    }

    info!("Finished startup");
    state.set(UiState::Menu);
}
