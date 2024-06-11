use bevy::prelude::*;

use crate::theme::Theme;

pub fn init(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: theme.background.into(),
            ..Default::default()
        },
        ..Default::default()
    });
}
