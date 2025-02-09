use bevy::prelude::*;

use crate::input::picking::{wall::CreateWallState, PickingState};

pub fn wall(_: Trigger<Pointer<Click>>, mut input: ResMut<PickingState>) {
    *input = PickingState::CreateWall(CreateWallState::SelectStart);
}

#[derive(Debug, Default, Component)]
pub struct WallAction {
    _state: WallActionState,
}

#[derive(Debug, Default)]
enum WallActionState {
    #[default]
    Start,
    #[expect(unused)]
    End { start: Entity },
}
