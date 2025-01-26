pub mod camera;
pub mod cancel;
pub mod picking;
pub mod settings;

pub use self::settings::Settings;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::input::settings::Action;

#[derive(Event, Debug, Clone, Copy)]
pub struct CancelInput;

#[derive(Event, Debug, Clone, Copy)]
pub struct CameraInput {
    pub kind: CameraInputKind,
    pub state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraInputKind {
    PanLeft,
    PanUp,
    PanRight,
    PanDown,
    ZoomIn,
    ZoomOut,
}

pub fn read(
    mut commands: Commands,
    settings: Res<Settings>,
    mut keyboard_e: EventReader<KeyboardInput>,
    keyboard_state: Res<ButtonInput<KeyCode>>,
) {
    for event in keyboard_e.read() {
        match event.state {
            ButtonState::Pressed if !keyboard_state.just_pressed(event.key_code) => continue,
            ButtonState::Released if !keyboard_state.just_released(event.key_code) => continue,
            _ => (),
        }

        for binding in settings.get_bind(event.key_code) {
            if !binding.modifiers.iter().all(|&m| keyboard_state.pressed(m)) {
                continue;
            }

            match binding.action {
                Action::Cancel if event.state == ButtonState::Released => {
                    commands.trigger(CancelInput);
                }
                Action::PanLeft => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanLeft,
                    state: event.state,
                }),
                Action::PanUp => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanUp,
                    state: event.state,
                }),
                Action::PanRight => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanRight,
                    state: event.state,
                }),
                Action::PanDown => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanDown,
                    state: event.state,
                }),
                Action::ZoomIn => commands.trigger(CameraInput {
                    kind: CameraInputKind::ZoomIn,
                    state: event.state,
                }),
                Action::ZoomOut => commands.trigger(CameraInput {
                    kind: CameraInputKind::ZoomOut,
                    state: event.state,
                }),
                _ => (),
            }
        }
    }
}
