use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierConfiguration;

pub fn startup(mut config: Query<&mut RapierConfiguration>) {
    config.single_mut().gravity = Vec2::ZERO;
}
