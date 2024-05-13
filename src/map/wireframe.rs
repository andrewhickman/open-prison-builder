use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_tilemap::{
    helpers::geometry::get_tilemap_center_transform,
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{TileColor, TileFlip, TilePos, TilePosOld, TileStorage, TileTextureIndex, TileVisible},
    TilemapBundle,
};

use crate::{control::Action, loading::TextureAssets, map::TILE_SIZE};

use super::{HoveredTile, WIREFRAME_Z_INDEX};

#[derive(Component)]
pub struct WireframeTilemap;

#[derive(Default, Component)]
pub struct WireframeTile;

#[derive(Default, Bundle)]
pub struct WireframeTileBundle {
    pub wireframe: WireframeTile,
    pub position: TilePos,
    pub texture_index: TileTextureIndex,
    pub tilemap_id: TilemapId,
    pub visible: TileVisible,
    pub flip: TileFlip,
    pub color: TileColor,
    pub old_position: TilePosOld,
}

pub fn startup(mut commands: Commands, textures: Res<TextureAssets>) {
    let size = TilemapSize { x: 500, y: 500 };
    let tile_size = TilemapTileSize {
        x: TILE_SIZE as f32,
        y: TILE_SIZE as f32,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.spawn((
        WireframeTilemap,
        TilemapBundle {
            grid_size,
            map_type,
            size,
            storage: TileStorage::empty(size),
            texture: TilemapTexture::Single(textures.wireframes.clone()),
            tile_size,
            transform: get_tilemap_center_transform(&size, &grid_size, &map_type, WIREFRAME_Z_INDEX),
            ..Default::default()
        },
        HoveredTile::default(),
    ));
}

pub fn update_tile_preview(
    mut commands: Commands,
    action: Res<Action>,
    wireframe_q: Query<(Entity, &TilePos), With<WireframeTile>>,
    mut tilemap_q: Query<(Entity, Ref<HoveredTile>, &mut TileStorage), With<WireframeTilemap>>,
) {
    let (tilemap_entity, hovered, mut storage) = tilemap_q.single_mut();
    if !action.is_changed() && !hovered.is_changed() {
        return;
    }

    let mut prev_positions: HashMap<Entity, &TilePos> = wireframe_q.iter().collect();

    let preview_tiles = action_preview(&action, &hovered, TilemapId(tilemap_entity));
    for tile in preview_tiles {
        let position = tile.position;
        if let Some(entity) = storage.get(&position) {
            prev_positions.remove(&entity);
            commands.entity(entity).insert(tile);
        } else {
            let entity = commands.spawn(tile).id();
            storage.set(&position, entity);
        };
    }

    for (entity, pos) in prev_positions {
        commands.entity(entity).despawn();
        storage.remove(pos);
    }
}

fn action_preview(
    action: &Action,
    hovered: &HoveredTile,
    tilemap_id: TilemapId,
) -> Vec<WireframeTileBundle> {
    match (action, hovered) {
        (Action::None, _) | (_, &HoveredTile(None)) => vec![],
        (Action::SelectStartPoint(_), &HoveredTile(Some(position))) => {
            vec![WireframeTileBundle {
                wireframe: WireframeTile,
                position,
                texture_index: TileTextureIndex(wall_index(
                    false, false, false, false, false, false, false, false,
                )),
                tilemap_id,
                ..Default::default()
            }]
        }
        (Action::SelectEndPoint(_, _), HoveredTile(Some(_))) => todo!(),
    }
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
