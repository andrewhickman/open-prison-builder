use bevy::prelude::*;

use pb_assets::Assets;
use pb_engine::{
    pawn::{self, Pawn},
    Root,
};

pub fn init_root(mut commands: Commands, root_q: Query<Entity, Added<Root>>) {
    for root in &root_q {
        commands.entity(root).insert(Visibility::default());
    }
}

pub fn init_pawn(mut commands: Commands, pawn_q: Query<Entity, Added<Pawn>>, assets: Res<Assets>) {
    for pawn in &pawn_q {
        commands.entity(pawn).insert((
            Sprite {
                custom_size: Some(Vec2::splat(pawn::RADIUS * 2.5)),
                image: assets.pawn_image.clone(),
                ..default()
            },
            Visibility::default(),
        ));
    }
}
