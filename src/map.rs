use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::geometry::get_tilemap_center_transform,
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};

use crate::{loading::TextureAssets, GameState};

const TILE_SIZE: u32 = 32;
const LARGE_TILE_SIZE: u32 = 1024;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(OnEnter(GameState::Running), startup);
    }
}

fn startup(mut commands: Commands, textures: Res<TextureAssets>) {
    let map_size = TilemapSize { x: 500, y: 500 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: large_texture_id(TileTextureIndex(0), tile_pos),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {
        x: TILE_SIZE as f32,
        y: TILE_SIZE as f32,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(textures.atlas.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn large_texture_id(base: TileTextureIndex, pos: TilePos) -> TileTextureIndex {
    let len = LARGE_TILE_SIZE / TILE_SIZE;
    TileTextureIndex(base.0 + (len - 1 - (pos.y % len)) * len + (pos.x % len))
}
