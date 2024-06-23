#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod sprite;
mod wall;

use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, wall::startup);
        app.add_systems(
            PostUpdate,
            (
                sprite::init_root,
                sprite::init_pawn,
                ((wall::init_vertex, wall::init_wall), wall::update_wall).chain(),
            ),
        );
    }
}
