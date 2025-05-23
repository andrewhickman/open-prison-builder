use bevy::prelude::*;
use pb_engine::pawn::Pawn;

#[derive(Event, Debug, Clone, Copy)]
pub struct SelectPawn {
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
        .entity(trigger.target())
        .observe(over)
        .observe(out)
        .observe(click);
}

fn over(mut trigger: Trigger<Pointer<Over>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(SelectPawn {
        pawn: trigger.target(),
    });
}

fn out(mut trigger: Trigger<Pointer<Out>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(CancelPawn {
        pawn: trigger.target(),
    });
}

fn click(mut trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
    trigger.propagate(false);

    if trigger.button == PointerButton::Primary {
        commands.trigger(ClickPawn {
            pawn: trigger.target(),
        });
    }
}
