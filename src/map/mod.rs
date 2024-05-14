mod material;
mod wireframe;

pub use self::wireframe::WireframeTilemap;

use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::square_grid::neighbors::Neighbors,
    map::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileTextureIndex},
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
pub struct HoveredTile(pub Option<TilePos>);

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
                    .run_if(
                        in_state(GameState::Running).and_then(wireframe::run_update_tile_preview),
                    ),
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

    for (mut hovered_tile, map_size, grid_size, map_type, map_transform) in &mut tilemaps {
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

fn test_texture_index<P>(size: &TilemapSize, position: &TilePos, test: P) -> TileTextureIndex
where
    P: Fn(TilePos) -> bool,
{
    let neighbors = Neighbors::get_square_neighboring_positions(position, size, true);
    TileTextureIndex(wall_index(
        test_neighbour(neighbors.north_west, &test),
        test_neighbour(neighbors.north, &test),
        test_neighbour(neighbors.north_east, &test),
        test_neighbour(neighbors.east, &test),
        test_neighbour(neighbors.south_east, &test),
        test_neighbour(neighbors.south, &test),
        test_neighbour(neighbors.south_west, &test),
        test_neighbour(neighbors.west, &test),
    ))
}

fn test_neighbour<P>(position: Option<TilePos>, test: &P) -> bool
where
    P: Fn(TilePos) -> bool,
{
    position.map(test).unwrap_or(false)
}

#[allow(clippy::too_many_arguments)]
fn wall_index(tl: bool, t: bool, tr: bool, r: bool, br: bool, b: bool, bl: bool, l: bool) -> u32 {
    match (tl, t, tr, r, br, b, bl, l) {
        // Isolated wall
        (_, false, _, false, _, false, _, false) => 0,
        // End walls
        (_, true, _, false, _, false, _, false) => 1,
        (_, false, _, true, _, false, _, false) => 2,
        (_, false, _, false, _, true, _, false) => 3,
        (_, false, _, false, _, false, _, true) => 4,
        // Top-right corner
        (_, true, false, true, _, false, _, false) => 5,
        (_, true, true, true, _, false, _, false) => 6,
        // Bottom-right corner
        (_, false, _, true, false, true, _, false) => 7,
        (_, false, _, true, true, true, _, false) => 8,
        // Bottom-left corner
        (_, false, _, false, _, true, false, true) => 9,
        (_, false, _, false, _, true, true, true) => 10,
        // Top-left corner
        (false, true, _, false, _, false, _, true) => 11,
        (true, true, _, false, _, false, _, true) => 12,
        // Vertical straight
        (_, true, _, false, _, true, _, false) => 13,
        // Horizontal straight
        (_, false, _, true, _, false, _, true) => 14,
        // Top t-junction
        (false, true, false, true, _, false, _, true) => 15,
        (true, true, false, true, _, false, _, true) => 16,
        (false, true, true, true, _, false, _, true) => 17,
        (true, true, true, true, _, false, _, true) => 18,
        // Right t-junction
        (_, true, false, true, false, true, _, false) => 19,
        (_, true, true, true, false, true, _, false) => 20,
        (_, true, false, true, true, true, _, false) => 21,
        (_, true, true, true, true, true, _, false) => 22,
        // Bottom t-junction
        (_, false, _, true, false, true, false, true) => 23,
        (_, false, _, true, true, true, false, true) => 24,
        (_, false, _, true, false, true, true, true) => 25,
        (_, false, _, true, true, true, true, true) => 26,
        // Left t-junction
        (false, true, _, false, _, true, false, true) => 27,
        (true, true, _, false, _, true, false, true) => 28,
        (false, true, _, false, _, true, true, true) => 29,
        (true, true, _, false, _, true, true, true) => 30,
        // 4-way intersection
        (false, true, false, true, false, true, false, true) => 31,
        (true, true, false, true, false, true, false, true) => 32,
        (false, true, true, true, false, true, false, true) => 33,
        (true, true, true, true, false, true, false, true) => 34,
        (false, true, false, true, true, true, false, true) => 35,
        (true, true, false, true, true, true, false, true) => 36,
        (false, true, true, true, true, true, false, true) => 37,
        (true, true, true, true, true, true, false, true) => 38,
        (false, true, false, true, false, true, true, true) => 39,
        (true, true, false, true, false, true, true, true) => 40,
        (false, true, true, true, false, true, true, true) => 41,
        (true, true, true, true, false, true, true, true) => 42,
        (false, true, false, true, true, true, true, true) => 43,
        (true, true, false, true, true, true, true, true) => 44,
        (false, true, true, true, true, true, true, true) => 45,
        (true, true, true, true, true, true, true, true) => 46,
    }
}
