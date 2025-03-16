use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use vleue_navigator::{
    prelude::{NavMeshSettings, NavMeshUpdateMode},
    Triangulation,
};

use crate::{map::Map, pawn};

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Transform)]
pub struct Root;

#[derive(Clone, Bundle)]
pub struct RootBundle {
    pub root: Root,
    pub name: Name,
    pub map: Map,
    pub nav_mesh_settings: NavMeshSettings,
    pub nav_mesh_update_mode: NavMeshUpdateMode,
}

impl Default for RootBundle {
    fn default() -> Self {
        RootBundle {
            root: Root,
            name: Name::new(Root::type_path()),
            map: Map::default(),
            nav_mesh_settings: NavMeshSettings {
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
            nav_mesh_update_mode: NavMeshUpdateMode::Direct,
        }
    }
}
