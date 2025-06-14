use bevy::prelude::*;
use pb_engine::{EngineState, pawn::Pawn};

use crate::{
    action::Action,
    input::{cancel::Cancellable, picking::point::ClickPoint},
};

pub fn pawn(_: Trigger<Pointer<Click>>, mut commands: Commands) -> Result {
    commands.spawn((PawnAction, children![Observer::new(click_point)]));
    Ok(())
}

#[derive(Default, Debug, Component, TypePath)]
#[require(Action, Cancellable, Name::new(PawnAction::type_path()))]
pub struct PawnAction;

fn click_point(
    trigger: Trigger<ClickPoint>,
    mut commands: Commands,
    engine_state: Res<State<EngineState>>,
) {
    let &EngineState::Running(root) = engine_state.get() else {
        warn!("engine not running");
        return;
    };

    commands.spawn((Pawn::bundle(trigger.point, 0.), ChildOf(root)));
}
