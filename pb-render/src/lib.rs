#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod grid;
pub mod projection;
pub mod sprite;
pub mod wall;

use bevy::{prelude::*, sprite::Material2dPlugin};
use grid::GridMaterial;
use pb_engine::wall::WallMap;

pub struct PbRenderPlugin;

impl Plugin for PbRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (wall::startup, grid::startup));
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
        .add_observer(wall::hidden_inserted)
        .add_observer(wall::hidden_removed)
        .add_observer(sprite::root_added)
        .add_observer(sprite::pawn_added);

        app.add_plugins(Material2dPlugin::<GridMaterial>::default());
    }
}
