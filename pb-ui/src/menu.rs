use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use pb_util::try_res;

use crate::{node::Nodes, theme::Theme, widget::UiBuilder};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum MenuState {
    #[default]
    Shown,
    Hidden,
}

pub fn show(nodes: Res<Nodes>, mut visibility_q: Query<&mut Visibility>) {
    let mut visibility = try_res!(visibility_q.get_mut(nodes.menu));
    visibility.set_if_neq(Visibility::Visible);
}

pub fn hide(nodes: Res<Nodes>, mut visibility_q: Query<&mut Visibility>) {
    let mut visibility = try_res!(visibility_q.get_mut(nodes.menu));
    visibility.set_if_neq(Visibility::Hidden);
}

impl<'a> UiBuilder<'a> {
    pub fn main_menu(&mut self, theme: &Theme) -> UiBuilder<'_> {
        let mut menu = self.spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(theme.gutter),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: theme.panel.into(),
                ..default()
            },
            theme.outline,
        ));

        menu.main_menu_button(theme, "Play");
        menu.main_menu_button(theme, "Play2");

        menu
    }

    fn main_menu_button(&mut self, theme: &Theme, text: &'static str) {
        self.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(theme.gutter),
                    padding: UiRect::all(theme.gutter),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: theme.accent.into(),
                ..default()
            },
            On::<Pointer<Click>>::run(move || info!("clicked {}", text)),
        ))
        .spawn(TextBundle::from_section(text, theme.header_text.clone()));
    }
}
