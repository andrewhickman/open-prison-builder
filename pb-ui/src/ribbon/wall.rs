use bevy::prelude::*;

pub fn wall(mut trigger: Trigger<Pointer<Click>>) {
    trigger.propagate(false);

    info!("wall");
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
