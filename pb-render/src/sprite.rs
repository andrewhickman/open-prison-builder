use bevy::prelude::*;

use pb_assets::AssetHandles;
use pb_engine::{
    pawn::{self, Pawn},
    root::Root,
};

pub fn root_added(trigger: Trigger<OnAdd, Root>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(Visibility::default());
}

pub fn pawn_added(
    trigger: Trigger<OnAdd, Pawn>,
    mut commands: Commands,
    assets: Res<AssetHandles>,
) {
    commands.entity(trigger.entity()).insert((
        Sprite {
            custom_size: Some(Vec2::splat(pawn::RADIUS * 2.5)),
            image: assets.pawn_image.clone(),
            ..default()
        },
        Visibility::default(),
    ));
}
