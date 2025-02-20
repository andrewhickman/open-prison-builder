use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Map {
    pub size: Vec2,
}

impl Default for Map {
    fn default() -> Self {
        Map {
            size: Vec2::new(100., 100.),
        }
    }
}
