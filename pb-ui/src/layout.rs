use bevy::prelude::*;
use bevy_mod_picking::picking_core::Pickable;

use pb_assets::Assets;

use crate::{theme::Theme, widget::UiBuilder};

#[derive(Resource)]
pub struct Layout {
    pub root: Entity,
    pub menu: Entity,
    pub messages: Entity,
}

pub fn init(mut commands: Commands, theme: Res<Theme>, assets: Res<Assets>) {
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

    let menu = builder.menu(&theme, &assets).id();

    let messages = builder.messages().id();

    commands.insert_resource(Layout {
        root,
        menu,
        messages,
    })
}
