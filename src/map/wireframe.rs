use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::{
    helpers::geometry::get_tilemap_center_transform,
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{
        TileBundle, TileColor, TileFlip, TilePos, TilePosOld, TileStorage, TileTextureIndex,
        TileVisible,
    },
    TilemapBundle,
};

use crate::{control::Action, assets::TextureAssets, map::TILE_SIZE};

use super::{test_texture_index, tiles_between, HoveredTile, WIREFRAME_Z_INDEX};

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
    let tile_size = TilemapTileSize { x: TILE_SIZE as f32, y: TILE_SIZE as f32 };
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
            transform: get_tilemap_center_transform(
                &size,
                &grid_size,
                &map_type,
                WIREFRAME_Z_INDEX,
            ),
            ..Default::default()
        },
        HoveredTile::default(),
    ));
}

pub fn run_update_tile_preview(
    action: Res<Action>,
    hovered: Query<(), (Changed<HoveredTile>, With<WireframeTilemap>)>,
) -> bool {
    action.is_changed() || !hovered.is_empty()
}

pub fn update_tile_preview(
    mut commands: Commands,
    action: Res<Action>,
    mut wireframe_q: Query<(Entity, &TilePos, &mut TileTextureIndex), With<WireframeTile>>,
    mut tilemap_q: Query<(Entity, Ref<HoveredTile>, &mut TileStorage), With<WireframeTilemap>>,
) {
    let (tilemap_entity, hovered, mut storage) = tilemap_q.single_mut();

    let tiles = action_preview(&action, &hovered);

    for position in &tiles {
        if storage.get(position).is_none() {
            let texture_index = test_texture_index(&storage.size, position, |neighbor| {
                tiles.contains(&neighbor)
            });
            let entity = commands
                .spawn((
                    WireframeTile,
                    TileBundle {
                        position: *position,
                        texture_index,
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                ))
                .id();
            storage.set(position, entity);
        }
    }

    for (entity, position, mut index) in &mut wireframe_q {
        let new_index = test_texture_index(&storage.size, position, |neighbor| {
            tiles.contains(&neighbor)
        });

        if tiles.contains(position) {
            if index.0 != new_index.0 {
                *index = new_index;
            }
        } else {
            commands.entity(entity).despawn();
            storage.remove(position);
        }
    }
}

fn action_preview(action: &Action, hovered: &HoveredTile) -> HashSet<TilePos> {
    match (action, hovered) {
        (&Action::SelectStartPoint(_), &HoveredTile(Some(position))) => HashSet::from([position]),
        (&Action::SelectEndPoint(_, start), &HoveredTile(Some(end))) if start == end => {
            HashSet::from([start])
        }
        (&Action::SelectEndPoint(_, start), &HoveredTile(None)) => HashSet::from([start]),
        (&Action::SelectEndPoint(_, start), &HoveredTile(Some(end))) => {
            tiles_between(start, end).collect()
        }
        _ => HashSet::new(),
    }
}
