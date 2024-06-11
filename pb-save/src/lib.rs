pub mod save;
pub mod settings;
pub mod store;

use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (store::init, settings::init).chain());
    }
}
