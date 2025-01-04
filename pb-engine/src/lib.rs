pub mod collider;
pub mod pawn;
pub mod save;
pub mod wall;

mod rapier;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use pawn::Pawn;
use serde::{Deserialize, Serialize};
use wall::{Vertex, Wall};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum EngineState {
    #[default]
    Disabled,
    Loading,
    Running(Entity),
}

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Root;

#[derive(Default, Clone, Bundle)]
pub struct RootBundle {
    pub root: Root,
    pub transform: Transform,
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Root>()
            .register_type::<Pawn>()
            .register_type::<Wall>()
            .register_type::<Vertex>();

        app.init_state::<EngineState>();

        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_systems(Startup, rapier::startup);

        app.add_systems(PostUpdate, (collider::init_pawn, collider::init_wall));

        #[cfg(feature = "dev")]
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}
