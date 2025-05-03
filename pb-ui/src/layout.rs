use bevy::prelude::*;

use crate::{theme::Theme, widget::UiBuilder};

#[derive(Resource)]
pub struct Layout {
    pub root: Entity,
    pub menu: Entity,
    pub ribbon: Entity,
    pub messages: Entity,
}

pub fn init(mut commands: Commands, theme: Res<Theme>) {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            Pickable::IGNORE,
            Name::new("pb_ui::layout::root"),
        ))
        .id();

    let mut builder = UiBuilder::new(commands.reborrow(), root);

    let menu = builder
        .menu_root(&theme)
        .named("pb_ui::layout::menu_root")
        .id();

    let ribbon = builder
        .ribbon_root()
        .named("pb_ui::layout::ribbon_root")
        .id();

    let messages = builder.messages().named("pb_ui::layout::messages").id();

    commands.insert_resource(Layout {
        root,
        menu,
        ribbon,
        messages,
    })
}
