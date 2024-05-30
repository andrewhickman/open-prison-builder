use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Res, Resource},
    },
    ui::{node_bundles::NodeBundle, Style, Val},
    utils::default,
};

use crate::{theme::Theme, widget::UiBuilder};

#[derive(Resource)]
pub struct Nodes {
    pub root: Entity,
    pub menu: Entity,
}

pub fn spawn(mut commands: Commands, theme: Res<Theme>) {
    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            ..default()
        })
        .id();

    let mut builder = UiBuilder::new(commands.reborrow(), root);

    let menu = builder.main_menu(&theme).id();

    commands.insert_resource(Nodes { root, menu })
}
