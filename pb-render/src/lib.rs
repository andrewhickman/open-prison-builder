#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod grid;
pub mod map;
pub mod projection;
pub mod sprite;

use bevy::{prelude::*, sprite::Material2dPlugin};
use grid::GridMaterial;
use map::VisibleMap;

pub struct PbRenderPlugin;

impl Plugin for PbRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (map::startup, grid::startup));
        app.add_systems(Update, (map::update_visibility, map::update_geometry));
        app.add_observer(map::corner_inserted)
            .add_observer(map::wall_inserted)
            .add_observer(map::map_removed)
            .add_observer(sprite::root_added)
            .add_observer(sprite::pawn_added);

        app.init_resource::<VisibleMap>();

        app.add_plugins(Material2dPlugin::<GridMaterial>::default());
    }
}
