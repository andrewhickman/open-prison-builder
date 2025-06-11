use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::event::ComponentEvent;

use crate::{picking::Layer, root::ChildOfRoot};

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility)]
#[component(immutable)]
pub struct Perimeter {
    start: Vec2,
    end: Vec2,
}

pub fn add_colliders(
    mut commands: Commands,
    mut perimeter_e: EventReader<ComponentEvent<OnInsert, Perimeter>>,
    root_q: Query<&ChildOfRoot>,
) -> Result {
    for event in perimeter_e.read() {
        if root_q.contains(event.target) {
            commands.entity(event.target).insert((
                RigidBody::Static,
                Collider::half_space(Vec2::Y),
                CollisionLayers::new(Layer::Perimeter, LayerMask::ALL),
            ));
        }
    }
    Ok(())
}

impl Perimeter {
    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn end(&self) -> Vec2 {
        self.end
    }

    pub(crate) fn bundle(start: Vec2, end: Vec2) -> impl Bundle {
        let position = start.midpoint(end);
        let rotation = (start - end).to_angle();

        (
            Name::new(format!(
                "perimeter wall ({}, {}) to ({}, {})",
                start.x, start.y, end.x, end.y
            )),
            Perimeter { start, end },
            Transform {
                scale: Vec3::ONE,
                translation: position.extend(0.),
                rotation: Quat::from_rotation_z(rotation),
            },
        )
    }
}
