use bevy::prelude::*;
use bevy_ecs_tilemap::{
    helpers::geometry::get_tilemap_center_transform,
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage},
    TilemapBundle,
};

use crate::{
    assets::TextureAssets,
    map::{HoveredTile, TILE_SIZE},
    material::Material,
};

use super::MATERIAL_Z_INDEX;

#[derive(Component)]
pub struct MaterialTilemap;

pub fn startup(mut commands: Commands, textures: Res<TextureAssets>) {
    let map_size = TilemapSize { x: 500, y: 500 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos { x, y };
            let material = Material::Grass;
            let tile_entity = commands
                .spawn((
                    material,
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: material.index(tile_pos),
                        ..Default::default()
                    },
                ))
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

    commands.entity(tilemap_entity).insert((
        MaterialTilemap,
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(textures.materials.clone()),
            tile_size,
            transform: get_tilemap_center_transform(
                &map_size,
                &grid_size,
                &map_type,
                MATERIAL_Z_INDEX,
            ),
            ..Default::default()
        },
        HoveredTile::default(),
    ));
}
