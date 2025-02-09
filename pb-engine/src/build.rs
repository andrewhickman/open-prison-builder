use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
/// Marker for entities which haven't been built yet.
pub struct Blueprint;
