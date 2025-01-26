#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod sprite;
pub mod wall;

use bevy::prelude::*;
use pb_engine::wall::WallMap;

#[derive(Component)]
pub struct Preview;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, wall::startup);
        app.add_systems(
            PostUpdate,
            wall::update_wall.run_if(resource_changed::<WallMap>),
        )
        .add_observer(wall::vertex_inserted)
        .add_observer(wall::wall_inserted)
        .add_observer(sprite::root_added)
        .add_observer(sprite::pawn_added);
    }
}
