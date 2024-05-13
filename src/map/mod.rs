mod material;
mod wireframe;

use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
    TilemapPlugin,
};

use crate::{
    control::{ControlSystem, CursorPos},
    GameState,
};

pub const TILE_SIZE: u32 = 32;
pub const LARGE_TILE_SIZE: u32 = 1024;
pub const LARGE_TILE_SUBTILES: u32 = LARGE_TILE_SIZE / TILE_SIZE;
pub const LARGE_TILE_OFFSET: u32 = LARGE_TILE_SUBTILES * LARGE_TILE_SUBTILES;

pub const MATERIAL_Z_INDEX: f32 = 0.0;
pub const WIREFRAME_Z_INDEX: f32 = 1.0;

pub struct MapPlugin;

#[derive(Debug, Default, Component, PartialEq, Eq)]
pub struct HoveredTile(Option<TilePos>);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(
                OnEnter(GameState::Running),
                (material::startup, wireframe::startup),
            )
            .add_systems(
                Update,
                update_hovered_tile
                    .after(ControlSystem)
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                wireframe::update_tile_preview
                    .after(update_hovered_tile)
                    .run_if(in_state(GameState::Running)),
            );
    }
}

pub fn update_hovered_tile(
    q_cursor_pos: Query<&CursorPos, Changed<CursorPos>>,
    mut tilemaps: Query<(
        &mut HoveredTile,
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &Transform,
    )>,
) {
    if q_cursor_pos.is_empty() {
        return;
    }

    for (mut hovered_tile, map_size, grid_size, map_type, map_transform) in
        &mut tilemaps
    {
        let Some(cursor_pos) = q_cursor_pos.single().0 else {
            hovered_tile.set_if_neq(HoveredTile(None));
            continue;
        };

        let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
        let cursor_in_map_pos: Vec2 = (map_transform.compute_matrix().inverse() * cursor_pos).xy();

        let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        else {
            hovered_tile.set_if_neq(HoveredTile(None));
            continue;
        };

        hovered_tile.set_if_neq(HoveredTile(Some(tile_pos)));
    }
}
