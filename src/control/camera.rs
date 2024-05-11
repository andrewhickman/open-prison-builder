use bevy::prelude::*;

use crate::theme::Theme;

use super::CursorPos;

pub fn spawn_game_camera(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: theme.game_background().into(),
                ..Default::default()
            },
            ..Default::default()
        },
        CursorPos::default(),
    ));
}

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyE) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyQ) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.1 {
            ortho.scale = 0.1;
        }

        if direction != Vec3::ZERO {
            let z = transform.translation.z;
            transform.translation += time.delta_seconds() * direction * 500.;
            transform.translation.z = z;
        }
    }
}
