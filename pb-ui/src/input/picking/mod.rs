use std::mem::take;

use bevy::prelude::*;
use pb_engine::EngineState;
use wall::CreateWallState;

pub mod point;
pub mod wall;

#[derive(Default, Debug, Resource)]
pub enum PickingState {
    #[default]
    Select,
    CreateWall(CreateWallState),
}

impl PickingState {
    pub fn grid_enabled(&self) -> bool {
        match self {
            PickingState::Select => false,
            PickingState::CreateWall(create_wall_state) => create_wall_state.grid_enabled(),
        }
    }

    pub fn vertex_over(&mut self, commands: &mut Commands, event: &Pointer<Over>) {
        match self {
            PickingState::Select => {}
            PickingState::CreateWall(create_wall_state) => {
                create_wall_state.vertex_over(commands, event)
            }
        }
    }

    pub fn vertex_move(&mut self, commands: &mut Commands, event: &Pointer<Move>) {
        match self {
            PickingState::Select => {}
            PickingState::CreateWall(create_wall_state) => {
                create_wall_state.vertex_move(commands, event)
            }
        }
    }

    pub fn vertex_out(&mut self, commands: &mut Commands, event: &Pointer<Out>) {
        match self {
            PickingState::Select => {}
            PickingState::CreateWall(create_wall_state) => {
                create_wall_state.vertex_out(commands, event)
            }
        }
    }

    pub fn vertex_click(
        &mut self,
        commands: &mut Commands,
        state: &EngineState,
        event: &Pointer<Click>,
    ) {
        match self {
            PickingState::Select => {}
            PickingState::CreateWall(create_wall_state) => {
                create_wall_state.vertex_click(commands, state, event)
            }
        }
    }

    pub fn cancel(&mut self, commands: &mut Commands) -> bool {
        match take(self) {
            PickingState::Select => false,
            PickingState::CreateWall(state) => {
                state.cancel(commands);
                true
            }
        }
    }
}
