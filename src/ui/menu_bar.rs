use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};

use crate::theme::Theme;

pub type ButtonCallback = Box<dyn Fn(&mut ChildBuilder) -> Entity + Send + Sync>;

#[derive(Component)]
struct MenuBar {
    child: Option<Entity>,
}

#[derive(Component)]
struct MenuBarButton {
    parent: Entity,
    callback: ButtonCallback,
}

pub fn spawn_menu_bar<'a>(
    commands: &'a mut Commands,
    theme: Res<Theme>,
    buttons: Vec<(String, ButtonCallback)>,
) -> EntityCommands<'a> {
    let mut commands = commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::ColumnReverse,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        },
        MenuBar { child: None },
    ));

    let parent = commands.id();

    commands.with_children(|builder| {
        builder
            .spawn((
                NodeBundle {
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
                },
                Outline {
                    width: Val::Px(1.),
                    color: theme.text(),
                    offset: Val::Px(-1.),
                },
            ))
            .with_children(|builder| {
                for (text, callback) in buttons {
                    builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    flex_grow: 1.0,
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            MenuBarButton { parent, callback },
                            Outline {
                                width: Val::Px(1.),
                                color: theme.text(),
                                offset: Val::Px(-1.),
                            },
                        ))
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
    });

    commands
}
