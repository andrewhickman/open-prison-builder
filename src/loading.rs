use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {}
