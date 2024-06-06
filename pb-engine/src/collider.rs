use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    pawn::{self, Pawn},
    PIXELS_PER_METER,
};

pub fn init_pawn(mut commands: Commands, pawn_q: Query<Entity, Added<Pawn>>) {
    for pawn in &pawn_q {
        commands.entity(pawn).insert((
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(pawn::RADIUS * PIXELS_PER_METER),
        ));
    }
}
