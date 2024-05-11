use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ecs_tilemap::{
    helpers::geometry::get_tilemap_center_transform,
    map::{TilemapGridSize, TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle, TilemapPlugin,
};

use crate::{
    control::{ControlSystem, CursorPos},
    loading::TextureAssets,
    material::Material,
    GameState,
};

pub const TILE_SIZE: u32 = 32;
pub const LARGE_TILE_SIZE: u32 = 1024;
pub const LARGE_TILE_SUBTILES: u32 = LARGE_TILE_SIZE / TILE_SIZE;
pub const LARGE_TILE_OFFSET: u32 = LARGE_TILE_SUBTILES * LARGE_TILE_SUBTILES;

pub struct MapPlugin;

#[derive(Default, Component)]
pub struct HoveredTile(Option<(Entity, TilePos)>);

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(OnEnter(GameState::Running), startup)
            .add_systems(
                Update,
                update_hovered_tile
                    .after(ControlSystem)
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                spawn_dirt
                    .after(update_hovered_tile)
                    .run_if(input_just_pressed(MouseButton::Left)),
            );
    }
}

fn startup(mut commands: Commands, textures: Res<TextureAssets>) {
    let map_size = TilemapSize { x: 500, y: 500 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos { x, y };
            let material = Material::Grass;
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: material.index(tile_pos),
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

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(textures.atlas.clone()),
            tile_size,
            transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
        HoveredTile::default(),
    ));
}

fn update_hovered_tile(
    q_cursor_pos: Query<&CursorPos, Changed<CursorPos>>,
    mut tilemaps: Query<(
        &mut HoveredTile,
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
) {
    if q_cursor_pos.is_empty() {
        return;
    }

    for (mut hovered_tile, map_size, grid_size, map_type, tile_storage, map_transform) in
        tilemaps.iter_mut()
    {
        let Some(cursor_pos) = q_cursor_pos.single().0 else {
            hovered_tile.0 = None;
            continue;
        };

        let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
        let cursor_in_map_pos: Vec2 = (map_transform.compute_matrix().inverse() * cursor_pos).xy();

        let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        else {
            hovered_tile.0 = None;
            continue;
        };

        let Some(tile_entity) = tile_storage.get(&tile_pos) else {
            hovered_tile.0 = None;
            continue;
        };

        hovered_tile.0 = Some((tile_entity, tile_pos));
    }
}

fn spawn_dirt(hovered_tile_q: Query<&HoveredTile>, mut texture_q: Query<&mut TileTextureIndex>) {
    for tile in hovered_tile_q.iter() {
        if let Some((entity, pos)) = tile.0 {
            if let Ok(mut texture) = texture_q.get_mut(entity) {
                *texture = Material::Dirt.index(pos);
            }
        }
    }
}
