use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct WallAction {
    _state: WallActionState,
}

#[derive(Debug, Default)]
enum WallActionState {
    #[default]
    Start,
    #[allow(unused)]
    End { start: Entity },
}
