#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod grid;
pub mod layer;
pub mod pawn;
pub mod projection;
pub mod wall;

use bevy::{prelude::*, sprite::Material2dPlugin};
use grid::GridMaterial;
use wall::{VisibleMap, WallMaterial};

pub struct PbRenderPlugin;

impl Plugin for PbRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (wall::startup, grid::startup));
        app.add_systems(Update, (wall::update_visibility, wall::update_geometry));
        app.add_systems(
            PostUpdate,
            pawn::clear_rotation.before(TransformSystem::TransformPropagate),
        );
        app.add_observer(wall::corner_inserted)
            .add_observer(wall::wall_inserted)
            .add_observer(wall::map_removed)
            .add_observer(pawn::root_added)
            .add_observer(pawn::pawn_added);

        app.init_resource::<VisibleMap>();

        app.add_plugins(Material2dPlugin::<GridMaterial>::default())
            .add_plugins(Material2dPlugin::<WallMaterial>::default());
    }
}
