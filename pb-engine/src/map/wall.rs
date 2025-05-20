use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::event::Inserted;

use crate::{map::Wall, picking::Layer, root::RootQuery};

pub const RADIUS: f32 = 0.125;

pub fn add_colliders(
    mut commands: Commands,
    mut wall_e: EventReader<Inserted<Wall>>,
    wall_q: Query<&Wall>,
    root_q: RootQuery,
) -> Result {
    for event in wall_e.read() {
        if root_q.is_descendant_of_root(event.target) {
            let wall = wall_q.get(event.target)?;
            commands.entity(event.target).insert((
                RigidBody::Static,
                Collider::capsule_endpoints(
                    RADIUS,
                    Vec2::new(-wall.length() / 2., 0.),
                    Vec2::new(wall.length() / 2., 0.),
                ),
                CollisionLayers::new(Layer::Wall, LayerMask::ALL),
            ));
        }
    }
    Ok(())
}
