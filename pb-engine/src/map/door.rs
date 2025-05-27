use bevy::{ecs::query::QueryEntityError, prelude::*};
use pb_util::event::ComponentEvent;

use crate::map::wall::Wall;

pub const INNER_WIDTH: f32 = 0.75;
pub const MIN_WIDTH: f32 = 0.9;
pub const WIDTH: f32 = 1.0;
pub const DEPTH: f32 = 0.2;
pub const MAX_WIDTH: f32 = 1.1;

pub const HALF_INNER_WIDTH: f32 = INNER_WIDTH / 2.;
pub const HALF_WIDTH: f32 = WIDTH / 2.;
pub const HALF_DEPTH: f32 = DEPTH / 2.;

#[derive(Clone, Debug, Component)]
#[component(immutable)]
pub struct Door;

pub fn validate(
    mut commands: Commands,
    mut door_e: EventReader<ComponentEvent<OnInsert, Door>>,
    wall_q: Query<&Wall>,
) {
    for door in door_e.read() {
        match wall_q.get(door.target) {
            Ok(wall) if wall.length() >= MIN_WIDTH => {}
            Err(QueryEntityError::EntityDoesNotExist(..)) => {}
            Err(QueryEntityError::AliasedMutability(..)) => unreachable!(),
            Ok(_) | Err(QueryEntityError::QueryDoesNotMatch(..)) => {
                commands.entity(door.target).try_remove::<Door>();
            }
        }
    }
}
