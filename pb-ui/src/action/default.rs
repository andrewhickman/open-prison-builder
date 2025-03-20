use bevy::prelude::*;
use pb_engine::pawn::{ai::PawnActor, Pawn};
use pb_util::{
    callback::{spawn_compute, CallbackSender},
    try_opt, try_res_s, ChildBuildExt,
};
use vleue_navigator::{prelude::ManagedNavMesh, NavMesh};

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

fn select_pawn(_trigger: Trigger<SelectPawn>, _: Single<&mut DefaultAction>) {}

fn cancel_pawn(_trigger: Trigger<CancelPawn>, _: Single<&mut DefaultAction>) {}

fn click_pawn(trigger: Trigger<ClickPawn>, mut action: Single<&mut DefaultAction>) {
    action.click_pawn(trigger.pawn);
}

fn click_point(
    trigger: Trigger<ClickPoint>,
    mut action: Single<&mut DefaultAction>,
    transform_q: Query<&Transform, With<Pawn>>,
    sender: Res<CallbackSender>,
    navmesh_q: Option<Single<&ManagedNavMesh>>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    let navmesh = try_opt!(navmeshes.get(try_opt!(navmesh_q).id()));

    action.click_point(trigger.point, &transform_q, &sender, navmesh);
}

impl DefaultAction {
    fn click_pawn(&mut self, pawn: Entity) {
        *self = DefaultAction::SelectedPawn { pawn };
    }

    fn click_point(
        &mut self,
        to: Vec2,
        transform_q: &Query<&Transform, With<Pawn>>,
        sender: &CallbackSender,
        navmesh: &NavMesh,
    ) {
        match *self {
            DefaultAction::Default => (),
            DefaultAction::SelectedPawn { pawn } => {
                let from = try_res_s!(transform_q.get(pawn)).translation.xy();
                let sender = sender.clone();
                let navmesh = navmesh.clone();

                spawn_compute(async move {
                    let res = PawnActor::new(pawn, sender)
                        .move_to(navmesh, from, to)
                        .await;
                    info!("move result: {res:?}");
                });
            }
        }
    }
}
