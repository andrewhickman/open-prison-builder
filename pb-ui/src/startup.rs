use bevy::{app::AppExit, asset::LoadState, prelude::*};

use pb_assets::Assets;
use pb_save::settings::Settings;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum StartupState {
    #[default]
    Pending,
    Ready,
}

pub fn update(
    mut state: ResMut<NextState<StartupState>>,
    assets: Res<Assets>,
    asset_server: Res<AssetServer>,
    settings: Option<Res<Settings>>,
    mut exit_e: EventWriter<AppExit>,
) {
    match assets.load_state(&asset_server) {
        LoadState::NotLoaded | LoadState::Loading => return,
        LoadState::Loaded => (),
        LoadState::Failed => {
            error!("Failed to load all assets, exiting");
            exit_e.send(AppExit);
        }
    }

    if settings.is_none() {
        return;
    }

    info!("Finished startup");
    state.set(StartupState::Ready);
}
