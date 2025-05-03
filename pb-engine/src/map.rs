use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Map {
    pub extents: Rect,
}

#[derive(Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct MapLayer;

impl Default for Map {
    fn default() -> Self {
        Map {
            extents: Rect::new(-100., -100., 100., 100.),
        }
    }
}

pub fn map_added(trigger: Trigger<OnAdd, Map>, mut commands: Commands) {
    commands.spawn((Name::new("MapLayer"), ChildOf(trigger.target()), MapLayer));
}
