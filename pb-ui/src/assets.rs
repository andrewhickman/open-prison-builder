use std::num::NonZeroU8;

use bevy::{app::AppExit, asset::LoadState, prelude::*};

use pb_assets::AssetHandles;
use pb_util::AsDynError;

use crate::{input::Settings, UiState};

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
            error!(
                error = error.as_dyn_error(),
                "Failed to load all assets, exiting"
            );
            exit_e.write(AppExit::Error(NonZeroU8::new(1).unwrap()));
        }
    }

    if settings.is_none() {
        return;
    }

    info!("Finished startup");
    state.set(UiState::Menu);
}
