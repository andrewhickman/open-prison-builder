use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::event::ComponentEvent;
use spade::handles::FixedUndirectedEdgeHandle;

use crate::{picking::Layer, root::ChildOfRoot};

use super::door::Door;

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility)]
#[component(immutable)]
pub struct Wall {
    edge: FixedUndirectedEdgeHandle,
    length: f32,
    position: Vec2,
    rotation: Rot2,
    corners: [Entity; 2],
}

pub fn add_colliders(
    mut commands: Commands,
    mut wall_e: EventReader<ComponentEvent<OnInsert, Wall>>,
    wall_q: Query<(&Wall, Has<Door>)>,
    root_q: Query<&ChildOfRoot>,
) -> Result {
    for event in wall_e.read() {
        if root_q.contains(event.target) {
            let (wall, is_door) = wall_q.get(event.target)?;
            if !is_door {
                commands.entity(event.target).insert((
                    RigidBody::Static,
                    Collider::rectangle(wall.length(), Wall::RADIUS * 2.),
                    CollisionLayers::new(Layer::Wall, LayerMask::ALL),
                ));
            }
        }
    }
    Ok(())
}

impl Wall {
    pub const RADIUS: f32 = 0.125;

    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn corners(&self) -> [Entity; 2] {
        self.corners
    }

    pub fn start(&self) -> Entity {
        self.corners[0]
    }

    pub fn end(&self) -> Entity {
        self.corners[1]
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn rotation(&self) -> Rot2 {
        self.rotation
    }

    pub fn isometry(&self) -> Isometry2d {
        Isometry2d {
            translation: self.position,
            rotation: self.rotation,
        }
    }

    pub(crate) fn edge(&self) -> FixedUndirectedEdgeHandle {
        self.edge
    }

    pub(crate) fn bundle(
        edge: FixedUndirectedEdgeHandle,
        corners: [Entity; 2],
        [position1, position2]: [Vec2; 2],
    ) -> impl Bundle {
        let length = position1.distance(position2);
        let position = position1.midpoint(position2);
        let rotation = (position2 - position1).to_angle();

        (
            Name::new(format!(
                "wall ({}, {}) to ({}, {})",
                position1.x, position1.y, position2.x, position2.y
            )),
            Wall {
                edge,
                length,
                position,
                rotation: Rot2::radians(rotation),
                corners,
            },
            Transform {
                scale: Vec3::ONE,
                translation: position.extend(0.),
                rotation: Quat::from_rotation_z(rotation),
            },
        )
    }
}
