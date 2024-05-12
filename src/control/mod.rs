mod camera;

use approx::abs_diff_eq;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{ui::UiMarkers, GameState};

use self::camera::{camera_movement, spawn_game_camera};

pub struct ControlPlugin;

#[derive(Clone, Debug, SystemSet, Hash, PartialEq, Eq)]
pub struct ControlSystem;

/// The position of the cursor, in world space.
///
/// `None` if the cursor is hovering over a UI element or outside the window.
#[derive(Default, Component)]
pub struct CursorPos(pub Option<Vec2>);

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), spawn_game_camera);
        app.add_systems(
            Update,
            (camera_movement, update_cursor_pos)
                .chain()
                .in_set(ControlSystem)
                .run_if(in_state(GameState::Running)),
        );
    }
}

pub fn update_cursor_pos(
    ui_markers: Res<UiMarkers>,
    body_q: Query<&Interaction>,
    mut q_cameras: Query<(Ref<GlobalTransform>, Ref<Camera>, &mut CursorPos)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Interaction::None = body_q.get(ui_markers.content).unwrap() {
        for (_, _, mut cursor_pos) in q_cameras.iter_mut() {
            set_cursor_if_changed(&mut cursor_pos, CursorPos(None));
        }
        return;
    }

    if let Some(position) = q_windows.single().cursor_position() {
        for (cam_t, cam, mut cursor_pos) in q_cameras.iter_mut() {
            let pos = cam.viewport_to_world_2d(&cam_t, position);
            set_cursor_if_changed(&mut cursor_pos, CursorPos(pos));
        }
    } else {
        for (_, _, mut cursor_pos) in q_cameras.iter_mut() {
            set_cursor_if_changed(&mut cursor_pos, CursorPos(None));
        }
    }
}

fn set_cursor_if_changed(cursor: &mut Mut<CursorPos>, new: CursorPos) {
    match (cursor.as_ref(), &new) {
        (&CursorPos(None), CursorPos(None)) => (),
        (&CursorPos(Some(old)), CursorPos(Some(new))) if abs_diff_eq!(old, new) => (),
        _ => {
            **cursor = new;
        }
    }
}
