use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Transform)]
pub struct Root;

impl Root {
    pub fn bundle() -> impl Bundle {
        (Root, Name::new(Root::type_path()))
    }
}
