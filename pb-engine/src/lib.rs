pub mod collider;
pub mod pawn;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use pawn::Pawn;
use serde::{Deserialize, Serialize};

pub const PIXELS_PER_METER: f32 = 128.;

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
    pub transform: TransformBundle,
}

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Root>().register_type::<Pawn>();

        app.init_state::<EngineState>();

        app.insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..RapierConfiguration::new(1.)
        });
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ));

        app.add_systems(PostUpdate, collider::init_pawn);

        #[cfg(feature = "dev")]
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}
