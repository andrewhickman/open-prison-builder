use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, PartialEq, Eq)]
pub struct Disabled(pub bool);

#[derive(Event, Debug, Clone, Copy)]
pub struct DisabledChanged(pub bool);

impl Disabled {
    pub const ENABLED: Self = Disabled(false);
    pub const DISABLED: Self = Disabled(true);
}

pub fn update(mut commands: Commands, mut changed: Query<(Entity, &Disabled), Changed<Disabled>>) {
    for (entity, disabled) in &mut changed {
        commands.trigger_targets(DisabledChanged(disabled.0), entity);
    }
}
