mod menu_bar;
mod spinner;

use bevy::prelude::*;

use crate::{theme::Theme, GameState};

pub use self::menu_bar::spawn_menu_bar;
pub use self::spinner::{spawn_spinner, Spinner, SpinnerBundle};

pub struct UiPlugin;

#[derive(Component)]
pub struct GameUi;

#[derive(Component)]
pub struct GameContent;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spinner::update_spinners, menu_bar::on_play_button_clicked),
        );
        app.add_systems(OnEnter(GameState::Running), spawn_game_ui);
        app.add_systems(OnExit(GameState::Running), despawn_game_ui);
    }
}

fn spawn_game_ui(mut commands: Commands, theme: Res<Theme>) {
    let ui_root = commands
        .spawn((
            GameUi,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            GameContent,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Interaction::default(),
        ))
        .set_parent(ui_root);

    spawn_menu_bar(
        &mut commands,
        theme,
        vec![
            ("Build".to_owned(), Box::new(spawn_build_menu)),
            ("View".to_owned(), Box::new(spawn_build_menu)),
            ("Manage".to_owned(), Box::new(spawn_build_menu)),
        ],
    )
    .set_parent(ui_root);
}

fn spawn_build_menu(commands: &mut ChildBuilder) {
    commands.spawn(TextBundle::from_section(
        "hello".to_owned(),
        TextStyle::default(),
    ));
}

fn despawn_game_ui(mut commands: Commands, ui_q: Query<Entity, With<GameUi>>) {
    for entity in ui_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
