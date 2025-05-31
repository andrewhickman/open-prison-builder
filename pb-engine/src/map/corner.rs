use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::event::ComponentEvent;
use spade::handles::FixedVertexHandle;

use crate::{map::wall::Wall, picking::Layer, root::ChildOfRoot};

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility)]
#[component(immutable)]
pub struct Corner {
    vertex: FixedVertexHandle,
    position: Vec2,
}

pub fn add_colliders(
    mut commands: Commands,
    mut corner_e: EventReader<ComponentEvent<OnInsert, Corner>>,
    root_q: Query<&ChildOfRoot>,
) -> Result {
    for event in corner_e.read() {
        if root_q.contains(event.target) {
            commands.entity(event.target).insert((
                RigidBody::Static,
                Collider::circle(Wall::RADIUS),
                CollisionLayers::new(Layer::Wall, LayerMask::ALL),
            ));
        }
    }
    Ok(())
}

impl Corner {
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub(crate) fn vertex(&self) -> FixedVertexHandle {
        self.vertex
    }

    pub(crate) fn bundle(vertex: FixedVertexHandle, position: Vec2) -> impl Bundle {
        (
            Name::new(format!("corner ({}, {})", position.x, position.y)),
            Corner { vertex, position },
            Transform::from_translation(position.extend(0.)),
        )
    }
}
