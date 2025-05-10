use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{map::Wall, picking::Layer, root::Root};

pub const RADIUS: f32 = 0.125;

pub fn add_colliders(
    mut commands: Commands,
    wall_q: Query<(Entity, &Wall), Without<Collider>>,
    parent_q: Query<&ChildOf>,
    root_q: Query<Has<Root>>,
) -> Result {
    for (id, wall) in &wall_q {
        let root = parent_q.root_ancestor(id);
        if root_q.get(root).unwrap_or_default() {
            commands.entity(id).insert((
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
