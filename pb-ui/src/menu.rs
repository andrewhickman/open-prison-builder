use bevy::prelude::*;

use pb_assets::Assets;
use pb_engine::pawn::PawnBundle;
use pb_util::try_res;

use crate::{node::Nodes, theme::Theme, widget::UiBuilder};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum MenuState {
    Shown,
    #[default]
    Hidden,
}

pub fn show(nodes: Res<Nodes>, mut style_q: Query<&mut Style>) {
    let mut style = try_res!(style_q.get_mut(nodes.menu));
    style.display = Display::Flex;
}

pub fn hide(nodes: Res<Nodes>, mut style_q: Query<&mut Style>) {
    let mut style = try_res!(style_q.get_mut(nodes.menu));
    style.display = Display::None;
}

impl<'a> UiBuilder<'a> {
    pub fn main_menu(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'_> {
        let mut menu = self.spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    padding: UiRect::all(theme.gutter),
                    flex_direction: FlexDirection::Column,
                    display: Display::Flex,
                    ..default()
                },
                background_color: theme.panel.into(),
                ..default()
            },
            theme.outline,
        ));

        menu.large_button(theme, "Play", play);

        let mut icon_bar = menu.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::RowReverse,
                ..default()
            },
            ..default()
        });

        icon_bar.icon_button(theme, assets.bevy_icon_image.clone(), || {
            open_url("https://bevyengine.org/")
        });
        icon_bar.icon_button(theme, assets.github_icon_image.clone(), || {
            open_url("https://github.com/andrewhickman/open-prison-builder/")
        });

        menu
    }
}

fn play(mut commands: Commands, mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Hidden);

    commands.spawn(PawnBundle::new(Vec2::default()));
}

fn open_url(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        error!("Failed to open url {url}: {err}");
    }
}
