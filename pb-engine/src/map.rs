use bevy::prelude::*;
use pb_util::uuid::Uuid;
use serde::{Deserialize, Serialize};
use vleue_navigator::prelude::*;

use crate::pawn;

#[derive(Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Map {
    pub extents: Rect,
}

#[derive(Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct MapLayer;

impl Default for Map {
    fn default() -> Self {
        Map {
            extents: Rect::new(-100., -100., 100., 100.),
        }
    }
}

pub fn map_added(trigger: Trigger<OnAdd, Map>, mut commands: Commands) {
    commands
        .spawn((
            Name::new("MapLayer"),
            MapLayer,
            ManagedNavMesh::from_id(Uuid::new_v4().as_u128()),
            NavMeshSettings {
                fixed: Triangulation::from_outer_edges(&[
                    Vec2::new(-500.0, -500.0),
                    Vec2::new(500.0, -500.0),
                    Vec2::new(500.0, 500.0),
                    Vec2::new(-500.0, 500.0),
                ]),
                agent_radius: pawn::RADIUS,
                simplify: 0.005,
                merge_steps: 1,
                ..default()
            },
            NavMeshUpdateMode::Direct,
        ))
        .set_parent(trigger.entity());
}
