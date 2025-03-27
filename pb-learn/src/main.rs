#![allow(unused)]

use std::f32::consts::FRAC_PI_4;

use glam::Vec2;
use gym::{Action, Gym};

mod gym;

fn main() {
    let mut gym = Gym::new();

    let mut observations = Vec::new();

    gym.reset();
    gym.observe(&mut observations);
    dbg!(&observations);
    gym.step(&[Action {
        entity: observations[0].entity,
        force: Vec2::new(100.0, 200.0),
        torque: Vec2::from_angle(FRAC_PI_4),
    }]);
    gym.observe(&mut observations);
    dbg!(&observations);
}
