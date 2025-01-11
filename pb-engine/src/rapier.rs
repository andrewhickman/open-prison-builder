use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

pub fn startup(mut config: Single<&mut RapierConfiguration>) {
    config.gravity = Vec2::ZERO;
}
