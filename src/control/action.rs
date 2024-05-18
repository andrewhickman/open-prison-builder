use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTextureIndex};

use crate::{
    map::{tiles_between, HoveredTile, MaterialTilemap, WireframeTilemap},
    material::Material,
    ui::ButtonCommandInput,
};

// State machine for user actions
#[derive(Debug, Default, Resource)]
pub enum Action {
    #[default]
    None,
    SelectStartPoint(Material),
    SelectEndPoint(Material, TilePos),
}

pub fn select_material(input: In<ButtonCommandInput>, mut action: ResMut<Action>) {
    let material = *input.get::<Material>();

    *action = Action::SelectStartPoint(material);
}

pub fn on_click(
    mut action: ResMut<Action>,
    hovered_q: Query<&HoveredTile, With<WireframeTilemap>>,
    storage_q: Query<&TileStorage, With<MaterialTilemap>>,
    mut material_q: Query<(&mut TileTextureIndex, &mut Material)>,
) {
    let hovered = hovered_q.single();
    let storage = storage_q.single();

    match (&*action, hovered) {
        (&Action::SelectStartPoint(material), &HoveredTile(Some(start))) => {
            *action = Action::SelectEndPoint(material, start);
        }
        (&Action::SelectEndPoint(new_material, start), &HoveredTile(Some(end))) => {
            for pos in tiles_between(start, end) {
                if let Some(tile_entity) = storage.get(&pos) {
                    if let Ok((mut texture, mut material)) = material_q.get_mut(tile_entity) {
                        *material = new_material;
                        *texture = new_material.index(pos);
                    }
                }
            }

            *action = Action::None;
        }
        _ => (),
    }
}
