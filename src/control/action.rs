use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{material::Material, ui::ButtonCommandInput};

// State machine for user actions
#[derive(Default, Resource)]
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
