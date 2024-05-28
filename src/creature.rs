use bevy::prelude::*;

use crate::ai::Brain;

#[derive(Bundle)]
pub struct CreatureBundle {
    brain: Brain,
    sprite: SpriteBundle,
}
