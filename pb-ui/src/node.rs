use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;

use crate::{theme::Theme, widget::UiBuilder};

#[derive(Resource)]
pub struct Nodes {
    pub root: Entity,
    pub menu: Entity,
}

pub fn init(mut commands: Commands, theme: Res<Theme>) {
    let root = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();

    let mut builder = UiBuilder::new(commands.reborrow(), root);

    let menu = builder.main_menu(&theme).id();

    commands.insert_resource(Nodes { root, menu })
}
