use bevy::prelude::*;
use pb_engine::{pawn::PawnBundle, EngineState};
use pb_util::ChildBuildExt;

use crate::input::{action::InputAction, picking::point::ClickPoint};

pub fn pawn(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands
        .spawn((PawnAction, Name::new(PawnAction::type_path())))
        .with_children(|builder| {
            builder.add_observer(click_point);
        });
}

#[derive(Default, Debug, Component, TypePath)]
#[require(InputAction, Transform, Visibility)]
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

    commands
        .spawn(PawnBundle::new(trigger.point))
        .set_parent(root);
}
