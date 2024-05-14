use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    map::{HoveredTile, WireframeTilemap},
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
) {
    let hovered = hovered_q.single();

    match (&*action, hovered) {
        (Action::None, _) => (),
        (&Action::SelectStartPoint(material), &HoveredTile(Some(position))) => {
            *action = Action::SelectEndPoint(material, position);
        }
        _ => (),
    }
}
