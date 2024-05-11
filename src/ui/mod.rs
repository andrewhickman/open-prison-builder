mod app_bar;
mod spinner;

use bevy::prelude::*;

use crate::{theme::Theme, GameState};

pub use self::app_bar::{spawn_app_bar, AppBody};
pub use self::spinner::{spawn_spinner, Spinner, SpinnerBundle};

pub struct UiPlugin;

#[derive(Component)]
struct GameUi;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spinner::update_spinners);
        app.add_systems(OnEnter(GameState::Running), spawn_game_ui);
        app.add_systems(OnExit(GameState::Running), despawn_game_ui);
    }
}

fn spawn_game_ui(mut commands: Commands, theme: Res<Theme>) {
    spawn_app_bar(&mut commands, theme).insert(GameUi);
}

fn despawn_game_ui(mut commands: Commands, ui_q: Query<Entity, With<GameUi>>) {
    for entity in ui_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
