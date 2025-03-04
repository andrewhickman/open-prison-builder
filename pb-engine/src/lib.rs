#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod build;
pub mod map;
pub mod pawn;
pub mod picking;
pub mod save;
pub mod wall;

use avian2d::prelude::*;
use bevy::prelude::*;
use build::Blueprint;
use map::Map;
use pawn::Pawn;
use serde::{Deserialize, Serialize};
use wall::{Vertex, Wall, WallMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum EngineState {
    #[default]
    Disabled,
    Running(Entity),
}

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Root;

#[derive(Default, Clone, Bundle)]
pub struct RootBundle {
    pub root: Root,
    pub map: Map,
    pub transform: Transform,
}

pub struct PbEnginePlugin;

impl Plugin for PbEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WallMap>();

        app.register_type::<Root>()
            .register_type::<Blueprint>()
            .register_type::<Pawn>()
            .register_type::<Wall>()
            .register_type::<Vertex>();

        app.init_state::<EngineState>();

        app.add_plugins(PhysicsPlugins::default());

        app.add_observer(wall::wall_added)
            .add_observer(wall::wall_removed)
            .add_systems(Update, wall::add_colliders);

        #[cfg(feature = "dev")]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}
