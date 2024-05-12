use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

use crate::theme::{ButtonTheme, Theme};

use super::button::{ButtonBundle, ButtonCommand};

pub fn spawn_menu_bar<'a>(
    commands: &'a mut Commands,
    theme: Res<Theme>,
    buttons: Vec<(String, ButtonCommand)>,
) -> EntityCommands<'a> {
    let mut commands = commands.spawn(NodeBundle {
        style: Style {
            align_self: AlignSelf::Stretch,
            width: Val::Percent(100.),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        background_color: BackgroundColor(theme.ui_background()),
        focus_policy: FocusPolicy::Block,
        ..Default::default()
    });

    commands.with_children(|builder| {
        for (text, callback) in buttons {
            builder
                .spawn((ButtonBundle {
                    theme: ButtonTheme::Normal,
                    command: callback,
                    style: Style {
                        flex_grow: 1.0,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(1.)),
                        ..Default::default()
                    },
                    border_color: BorderColor(theme.text()),
                    ..Default::default()
                },))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        text,
                        TextStyle {
                            font_size: 24.0,
                            color: theme.text(),
                            ..Default::default()
                        },
                    ));
                });
        }
    });

    commands
}
