use bevy::prelude::*;

use crate::assets::TextureAssets;
use crate::theme::{ButtonTheme, Theme};
use crate::ui::button::{register_button_command, ButtonBundle};
use crate::{commands, GameState};

use super::button::ButtonCommandInput;

pub struct MainMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), cleanup_menu);

        register_button_command(app, commands::PLAY, on_play_button_clicked);
        register_button_command(app, commands::OPEN_BEVY, on_open_bevy_button_clicked);
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands, theme: Res<Theme>, _: Res<TextureAssets>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: theme.ui_background().into(),
                ..Default::default()
            },
            ..Default::default()
        },
        Menu,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((ButtonBundle {
                    theme: ButtonTheme::Bold,
                    command: commands::PLAY,
                    style: Style {
                        width: Val::Px(140.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: theme.text(),
                            ..default()
                        },
                    ));
                });
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((ButtonBundle {
                    theme: ButtonTheme::Normal,
                    command: commands::OPEN_BEVY,
                    style: Style {
                        width: Val::Px(170.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 15.0,
                            color: theme.text(),
                            ..default()
                        },
                    ));
                });
        });
}

fn on_play_button_clicked(_: In<ButtonCommandInput>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Running);
}

fn on_open_bevy_button_clicked(_: In<ButtonCommandInput>) {
    if let Err(error) = webbrowser::open("https://bevyengine.org") {
        warn!("failed to open link {error:?}");
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
