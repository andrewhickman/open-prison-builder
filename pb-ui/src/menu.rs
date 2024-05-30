use bevy::{
    ecs::{
        change_detection::DetectChangesMut,
        schedule::States,
        system::{Query, Res},
    },
    render::view::Visibility,
    text::TextStyle,
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, Outline, Style, UiRect, Val,
    },
    utils::default,
};

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
                    padding: UiRect::all(Val::Px(5.)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: theme.panel.into(),
                ..default()
            },
            Outline {
                color: theme.text,
                width: Val::Px(1.),
                ..default()
            },
        ));

        menu.main_menu_button(theme, "Play");
        menu.main_menu_button(theme, "Play2");

        menu
    }

    fn main_menu_button(&mut self, theme: &Theme, text: impl Into<String>) {
        self.spawn(ButtonBundle {
            style: Style {
                margin: UiRect::all(Val::Px(5.)),
                width: Val::Px(150.),
                height: Val::Px(80.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: theme.accent.into(),
            ..default()
        })
        .spawn(TextBundle::from_section(
            text,
            TextStyle {
                color: theme.text,
                font_size: 36.,
                ..default()
            },
        ));
    }
}
