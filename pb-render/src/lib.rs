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
            Update,
            (
                wall::preview_moved,
                wall::update_wall.run_if(resource_changed::<WallMap>),
            )
                .chain(),
        )
        .add_observer(wall::vertex_inserted)
        .add_observer(wall::wall_inserted)
        .add_observer(wall::preview_removed)
        .add_observer(sprite::root_added)
        .add_observer(sprite::pawn_added);
    }
}
