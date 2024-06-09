pub mod collider;
pub mod pawn;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use pawn::Pawn;

pub const PIXELS_PER_METER: f32 = 128.;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pawn>();

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
