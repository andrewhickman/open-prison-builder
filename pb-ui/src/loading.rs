use bevy::prelude::*;

use crate::{menu::MenuState, node::Nodes, theme::Theme, widget::UiBuilder};

#[derive(Component)]
pub struct Loading;

pub fn enter(commands: Commands, nodes: Res<Nodes>, theme: Res<Theme>) {
    UiBuilder::new(commands, nodes.root)
        .spinner(
            &theme,
            60.,
            Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
        )
        .with(Loading);
}

pub fn exit(
    mut commands: Commands,
    query: Query<Entity, With<Loading>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }

    menu_state.set(MenuState::Shown);
}
