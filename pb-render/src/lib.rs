#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod sprite;
mod wall;

use bevy::prelude::*;
use wall::WallChanged;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, wall::startup);
        app.add_event::<WallChanged>()
            .add_systems(PostUpdate, wall::update_wall)
            .add_observer(wall::init_vertex)
            .add_observer(wall::init_wall)
            .add_observer(sprite::init_root)
            .add_observer(sprite::init_pawn);
    }
}
