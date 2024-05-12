use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::theme::{ButtonStyle, Theme};

pub type ButtonCallback = Box<dyn Fn(&mut ChildBuilder) + Send + Sync>;

#[derive(Component)]
pub struct MenuBar {
    child: Option<(Entity, Entity)>,
}

#[derive(Component)]
pub struct MenuBarButton {
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
                            ButtonStyle::Normal,
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

pub fn on_play_button_clicked(
    mut commands: Commands,
    mut menu_q: Query<(Entity, &mut MenuBar)>,
    interaction_q: Query<(Entity, &Interaction, &MenuBarButton), Changed<Interaction>>,
) {
    for (button_entity, interaction, button) in &interaction_q {
        if let Interaction::Pressed = *interaction {
            if let Ok((menu_entity, mut menu_bar)) = menu_q.get_mut(button.parent) {
                if let Some((existing_button, child)) = menu_bar.child.take() {
                    commands.entity(child).despawn_recursive();

                    if existing_button == button_entity {
                        continue;
                    }
                }

                let child = commands
                    .spawn(NodeBundle {
                        ..Default::default()
                    })
                    .with_children(&button.callback)
                    .set_parent(menu_entity)
                    .id();
                menu_bar.child = Some((button_entity, child));
            }
        }
    }
}
