use bevy::prelude::*;
use pb_engine::pawn::Pawn;

#[derive(Event, Debug, Clone, Copy)]
pub struct SelectPawn {
    #[expect(unused)]
    pub pawn: Entity,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct CancelPawn {
    #[expect(unused)]
    pub pawn: Entity,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ClickPawn {
    pub pawn: Entity,
}

pub fn pawn_added(trigger: Trigger<OnAdd, Pawn>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(over)
        .observe(moved)
        .observe(out)
        .observe(click);
}

fn over(mut trigger: Trigger<Pointer<Over>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(SelectPawn {
        pawn: trigger.entity(),
    });
}

fn moved(mut trigger: Trigger<Pointer<Move>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(SelectPawn {
        pawn: trigger.entity(),
    });
}

fn out(mut trigger: Trigger<Pointer<Out>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(CancelPawn {
        pawn: trigger.entity(),
    });
}

fn click(mut trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(ClickPawn {
        pawn: trigger.entity(),
    });
}
