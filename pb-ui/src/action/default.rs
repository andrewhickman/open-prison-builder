use bevy::prelude::*;
use pb_engine::pawn::ai::path::PathTaskBundle;

use crate::{
    action::Action,
    input::picking::{
        physics::pawn::{CancelPawn, ClickPawn, SelectPawn},
        point::ClickPoint,
    },
};

#[derive(Default, Debug, Component, TypePath)]
#[require(Action, Name = Name::new(DefaultAction::type_path()))]
pub enum DefaultAction {
    #[default]
    Default,
    SelectedPawn {
        pawn: Entity,
    },
}

pub fn spawn(mut commands: Commands) {
    commands.spawn((
        DefaultAction::default(),
        children![
            Observer::new(select_pawn),
            Observer::new(cancel_pawn),
            Observer::new(click_pawn),
            Observer::new(click_point),
        ],
    ));
}

fn select_pawn(_trigger: Trigger<SelectPawn>, _: Single<&mut DefaultAction>) {}

fn cancel_pawn(_trigger: Trigger<CancelPawn>, _: Single<&mut DefaultAction>) {}

fn click_pawn(trigger: Trigger<ClickPawn>, mut action: Single<&mut DefaultAction>) {
    action.click_pawn(trigger.pawn);
}

fn click_point(
    trigger: Trigger<ClickPoint>,
    mut commands: Commands,
    mut action: Single<&mut DefaultAction>,
) {
    action.click_point(&mut commands, trigger.point);
}

impl DefaultAction {
    fn click_pawn(&mut self, pawn: Entity) {
        *self = DefaultAction::SelectedPawn { pawn };
    }

    fn click_point(&mut self, commands: &mut Commands, to: Vec2) {
        match *self {
            DefaultAction::Default => (),
            DefaultAction::SelectedPawn { pawn } => {
                info!("move {pawn} to {to}");
                commands.spawn(PathTaskBundle::move_to(pawn, to));
            }
        }
    }
}
