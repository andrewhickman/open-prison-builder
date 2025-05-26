use bevy::prelude::*;
use pb_engine::map::corner::Corner;

#[derive(Event, Debug, Clone, Copy)]
pub struct SelectCorner {
    pub corner: Entity,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct CancelCorner {
    #[expect(unused)]
    pub corner: Entity,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ClickCorner {
    pub corner: Entity,
}

pub fn corner_added(trigger: Trigger<OnAdd, Corner>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(over)
        .observe(moved)
        .observe(out)
        .observe(click);
}

fn over(mut trigger: Trigger<Pointer<Over>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(SelectCorner {
        corner: trigger.target(),
    });
}

fn moved(mut trigger: Trigger<Pointer<Move>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(SelectCorner {
        corner: trigger.target(),
    });
}

fn out(mut trigger: Trigger<Pointer<Out>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(CancelCorner {
        corner: trigger.target(),
    });
}

fn click(mut trigger: Trigger<Pointer<Click>>, mut commands: Commands) {
    trigger.propagate(false);

    if trigger.button == PointerButton::Primary {
        commands.trigger(ClickCorner {
            corner: trigger.target(),
        });
    }
}
