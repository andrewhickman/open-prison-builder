use bevy::prelude::*;

use crate::{layout::Layout, theme::Theme, widget::UiBuilder};

#[derive(Component)]
pub struct Loading;

pub fn enter(commands: Commands, layout: Res<Layout>, theme: Res<Theme>) {
    UiBuilder::new(commands, layout.root)
        .spinner(&theme, 60.)
        .insert(Loading);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<Loading>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
