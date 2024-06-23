use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    pawn::{self, Pawn},
    wall::{self, Vertex, Wall},
};

pub fn init_pawn(mut commands: Commands, pawn_q: Query<Entity, Added<Pawn>>) {
    for pawn in &pawn_q {
        commands.entity(pawn).insert((
            RigidBody::Dynamic,
            Velocity::default(),
            Collider::ball(pawn::RADIUS),
        ));
    }
}

pub fn init_wall(
    mut commands: Commands,
    wall_q: Query<(Entity, &Wall), Added<Wall>>,
    vertex_q: Query<&Transform, With<Vertex>>,
) {
    for (id, wall) in &wall_q {
        let [start, end] = vertex_q.many([wall.start, wall.end]);
        commands.entity(id).insert((
            RigidBody::Fixed,
            Collider::capsule(start.translation.xy(), end.translation.xy(), wall::RADIUS),
        ));
    }
}
