use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use pb_engine::EngineState;

use crate::{menu::MenuState, widget::panel::PanelStack};

#[derive(Event, Debug, Clone)]
pub struct CancelCommand;

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
    mut toggle_menu_cmd: EventWriter<CancelCommand>,
    _camera_cmd: ResMut<CameraCommand>,
) {
    for event in keyboard_e.read() {
        match event.key_code {
            KeyCode::Escape if event.state == ButtonState::Released => {
                toggle_menu_cmd.send(CancelCommand);
            }
            _ => (),
        }
    }
}

pub fn cancel_command(
    mut commands: Commands,
    mut cancel_e: EventReader<CancelCommand>,
    mut panels: ResMut<PanelStack>,
    engine_state: Res<State<EngineState>>,
    menu_state: Res<State<MenuState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    for CancelCommand in cancel_e.read() {
        if matches!(engine_state.get(), EngineState::Loading) {
            continue;
        }

        let top_panel = match menu_state.get() {
            MenuState::Shown => panels.pop_child(),
            MenuState::Hidden => panels.pop(),
        };

        if let Some(entity) = top_panel {
            if let Some(entity) = commands.get_entity(entity) {
                entity.despawn_recursive();
            }
        } else {
            let toggled_menu = match menu_state.get() {
                MenuState::Shown => MenuState::Hidden,
                MenuState::Hidden => MenuState::Shown,
            };

            next_menu_state.set(toggled_menu);
        }
    }
}
