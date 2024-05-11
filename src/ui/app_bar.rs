use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

use crate::theme::Theme;

#[derive(Component)]
pub struct AppBar;

#[derive(Component)]
pub struct AppBody;

pub fn spawn_app_bar<'a>(commands: &'a mut Commands, theme: Res<Theme>) -> EntityCommands<'a> {
    let bar = commands
        .spawn((
            AppBar,
            NodeBundle {
                style: Style {
                    min_height: Val::Px(30.),
                    flex_grow: 0.0,
                    ..Default::default()
                },
                background_color: BackgroundColor(theme.ui_background()),
                focus_policy: FocusPolicy::Block,
                ..Default::default()
            },
            Outline {
                width: Val::Px(1.),
                color: theme.text(),
                offset: Val::Px(-1.),
            },
        ))
        .id();

    let body = commands
        .spawn((
            AppBody,
            NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    ..Default::default()
                },
                ..Default::default()
            },
            Interaction::default(),
        ))
        .id();

    let mut parent = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Stretch,
            ..default()
        },
        ..default()
    });

    parent.add_child(bar).add_child(body);

    parent
}
