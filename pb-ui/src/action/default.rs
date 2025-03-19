use bevy::prelude::*;
use pb_util::ChildBuildExt;

use crate::{
    action::Action,
    input::picking::{
        physics::pawn::{CancelPawn, ClickPawn, SelectPawn},
        point::ClickPoint,
    },
};

#[derive(Default, Debug, Component, TypePath)]
#[require(Action, Name(|| Name::new(DefaultAction::type_path())))]
pub enum DefaultAction {
    #[default]
    Default,
    SelectedPawn {
        pawn: Entity,
    },
}

pub fn spawn(mut commands: Commands) {
    commands
        .spawn(DefaultAction::default())
        .with_children(|builder| {
            builder
                .add_observer(select_pawn)
                .add_observer(cancel_pawn)
                .add_observer(click_pawn)
                .add_observer(click_point);
        });
}

fn select_pawn(trigger: Trigger<SelectPawn>, _: Single<&mut DefaultAction>) {
    info!("selected pawn {}", trigger.event().pawn);
}

fn cancel_pawn(trigger: Trigger<CancelPawn>, _: Single<&mut DefaultAction>) {
    info!("canceled pawn {}", trigger.event().pawn);
}

fn click_pawn(trigger: Trigger<ClickPawn>, mut action: Single<&mut DefaultAction>) {
    action.click_pawn(trigger.pawn);
}

fn click_point(trigger: Trigger<ClickPoint>, mut action: Single<&mut DefaultAction>) {
    action.click_point(trigger.point);
}

impl DefaultAction {
    fn click_pawn(&mut self, pawn: Entity) {
        *self = DefaultAction::SelectedPawn { pawn };
    }

    fn click_point(&mut self, point: Vec2) {
        match self {
            DefaultAction::Default => (),
            DefaultAction::SelectedPawn { pawn } => info!("move pawn {pawn} to {point}"),
        }
    }
}
