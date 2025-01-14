use bevy::prelude::*;

use pb_assets::Assets;
use pb_engine::{
    pawn::{self, Pawn},
    Root,
};

pub fn init_root(trigger: Trigger<OnAdd, Root>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .insert(Visibility::default());
}

pub fn init_pawn(trigger: Trigger<OnAdd, Pawn>, mut commands: Commands, assets: Res<Assets>) {
    commands.entity(trigger.entity()).insert((
        Sprite {
            custom_size: Some(Vec2::splat(pawn::RADIUS * 2.5)),
            image: assets.pawn_image.clone(),
            ..default()
        },
        Visibility::default(),
    ));
}
