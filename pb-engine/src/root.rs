use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::map::Map;

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Transform)]
pub struct Root;

#[derive(Clone, Bundle)]
pub struct RootBundle {
    pub root: Root,
    pub name: Name,
    pub map: Map,
}

impl Default for RootBundle {
    fn default() -> Self {
        RootBundle {
            root: Root,
            name: Name::new(Root::type_path()),
            map: Map::default(),
        }
    }
}
