use bevy::prelude::*;

use crate::input::TogglePauseInput;

pub fn input(_: Trigger<TogglePauseInput>, mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}
