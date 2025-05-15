use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::event::Inserted;

use crate::{map::Wall, picking::Layer, root::Root};

pub const RADIUS: f32 = 0.125;

pub fn add_colliders(
    mut commands: Commands,
    mut wall_e: EventReader<Inserted<Wall>>,
    wall_q: Query<&Wall>,
    parent_q: Query<&ChildOf>,
    root_q: Query<Has<Root>>,
) -> Result {
    for event in wall_e.read() {
        let root = parent_q.root_ancestor(event.target);
        if root_q.get(root).unwrap_or_default() {
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
