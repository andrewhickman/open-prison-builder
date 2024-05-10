mod spinner;

use bevy::app::{App, Plugin, Update};

pub use self::spinner::{spawn_spinner, Spinner, SpinnerBundle};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spinner::update_spinners);
    }
}
