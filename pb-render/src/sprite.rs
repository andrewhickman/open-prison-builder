use bevy::prelude::*;

use pb_assets::Assets;
use pb_engine::{
    pawn::{self, Pawn},
    PIXELS_PER_METER,
};

pub fn init(mut commands: Commands, pawn_q: Query<Entity, Added<Pawn>>, assets: Res<Assets>) {
    for pawn in &pawn_q {
        commands.entity(pawn).insert((
            Sprite {
                custom_size: Some(Vec2::splat(pawn::RADIUS * 2.5) * PIXELS_PER_METER),
                ..default()
            },
            assets.pawn_image.clone(),
            VisibilityBundle::default(),
        ));
    }
}
