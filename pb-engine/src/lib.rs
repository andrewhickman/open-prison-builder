pub mod pawn;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const PIXELS_PER_METER: f32 = 64.;

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ));

        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}
