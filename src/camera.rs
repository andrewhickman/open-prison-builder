use bevy::{prelude::*, window::PrimaryWindow};

use crate::{theme::Theme, GameState};

pub struct InputPlugin;

#[derive(Default, Component)]
pub struct CursorPos(pub Option<Vec2>);

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), setup_menu);
        app.add_systems(
            Update,
            (camera_movement, update_cursor_pos)
                .chain()
                .run_if(in_state(GameState::Running)),
        );
    }
}

fn setup_menu(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: theme.background().into(),
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

pub fn update_cursor_pos(
    mut q_cameras: Query<(Ref<GlobalTransform>, Ref<Camera>, &mut CursorPos)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_left_events: EventReader<CursorLeft>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let is_changed = !cursor_left_events.is_empty()
        || !cursor_moved_events.is_empty()
        || q_cameras
            .iter()
            .any(|(transform, camera, _)| transform.is_changed() || camera.is_changed());
    if !is_changed {
        return;
    }

    cursor_left_events.clear();
    cursor_moved_events.clear();

    if let Some(position) = q_windows.single().cursor_position() {
        for (cam_t, cam, mut cursor_pos) in q_cameras.iter_mut() {
            cursor_pos.0 = cam.viewport_to_world_2d(&cam_t, position);
        }
    } else {
        for (_, _, mut cursor_pos) in q_cameras.iter_mut() {
            cursor_pos.0 = None;
        }
    }
}
