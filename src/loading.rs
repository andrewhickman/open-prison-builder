use bevy::{app::AppExit, prelude::*};
use bevy_asset_loader::prelude::*;

use crate::{
    theme::Theme,
    ui::{spawn_spinner, Spinner, SpinnerBundle},
    GameState,
};

pub struct LoadingPlugin;

#[derive(Component)]
struct Loading;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .on_failure_continue_to_state(GameState::LoadingFailed)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
        app.add_systems(OnEnter(GameState::Loading), on_enter_loading)
            .add_systems(OnExit(GameState::Loading), on_exit_loading)
            .add_systems(OnEnter(GameState::LoadingFailed), on_failed);
    }
}

fn on_enter_loading(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: theme.ui_background().into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Loading,
    ));

    spawn_spinner(
        &mut commands,
        SpinnerBundle {
            spinner: Spinner {
                size: 60.0,
                color: BackgroundColor(theme.text()),
                ..Default::default()
            },
            style: Style {
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .insert(Loading);
}

fn on_exit_loading(mut commands: Commands, loading_q: Query<Entity, With<Loading>>) {
    for entity in loading_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn on_failed(mut quit: EventWriter<AppExit>) {
    error!("Failed to load assets, exiting application");
    quit.send_default();
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 32.0,
        tile_size_y = 32.0,
        columns = 64,
        rows = 32
    ))]
    pub materials_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "textures/atlas.png", image(sampler = nearest))]
    pub materials: Handle<Image>,
    // #[asset(texture_atlas_layout(tile_size_x = 32.0, tile_size_y = 32.0, columns = 32, rows = 8))]
    // pub wireframes_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "textures/wireframes.png", image(sampler = nearest))]
    pub wireframes: Handle<Image>,
}
