use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

#[derive(Event, Debug, Clone)]
pub struct ToggleMenuCommand;

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct CameraCommand {
    move_up: bool,
    move_left: bool,
    move_right: bool,
    move_down: bool,
    zoom_in: bool,
    zoom_out: bool,
}

pub fn update(
    mut keyboard_e: EventReader<KeyboardInput>,
    mut toggle_menu_cmd: EventWriter<ToggleMenuCommand>,
    _camera_cmd: ResMut<CameraCommand>,
) {
    for event in keyboard_e.read() {
        match event.key_code {
            KeyCode::Escape if event.state == ButtonState::Released => {
                toggle_menu_cmd.send(ToggleMenuCommand);
            }
            _ => (),
        }
    }
}
