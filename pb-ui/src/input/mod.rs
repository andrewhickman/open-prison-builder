pub mod camera;
pub mod cancel;
pub mod movement;
pub mod pause;
pub mod picking;
pub mod settings;

pub use self::settings::Settings;

use bevy::{
    input::{ButtonState, keyboard::KeyboardInput},
    prelude::*,
};
use movement::{MovementDirection, MovementInput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Input {
    Cancel,
    PanLeft,
    PanUp,
    PanRight,
    PanDown,
    ZoomIn,
    ZoomOut,
    DecreaseGridSize,
    IncreaseGridSize,
    MoveLeft,
    MoveForward,
    MoveRight,
    MoveBackward,
    TogglePause,
}

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

#[derive(Event, Debug, Clone, Copy)]
pub enum GridInput {
    DecreaseSize,
    IncreaseSize,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct TogglePauseInput;

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
                Input::Cancel => {
                    if event.state == ButtonState::Released {
                        commands.trigger(CancelInput);
                    }
                }
                Input::PanLeft => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanLeft,
                    state: event.state,
                }),
                Input::PanUp => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanUp,
                    state: event.state,
                }),
                Input::PanRight => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanRight,
                    state: event.state,
                }),
                Input::PanDown => commands.trigger(CameraInput {
                    kind: CameraInputKind::PanDown,
                    state: event.state,
                }),
                Input::ZoomIn => commands.trigger(CameraInput {
                    kind: CameraInputKind::ZoomIn,
                    state: event.state,
                }),
                Input::ZoomOut => commands.trigger(CameraInput {
                    kind: CameraInputKind::ZoomOut,
                    state: event.state,
                }),
                Input::DecreaseGridSize => {
                    if event.state == ButtonState::Released {
                        commands.trigger(GridInput::DecreaseSize);
                    }
                }
                Input::IncreaseGridSize => {
                    if event.state == ButtonState::Released {
                        commands.trigger(GridInput::IncreaseSize);
                    }
                }
                Input::MoveLeft => commands.trigger(MovementInput {
                    dir: MovementDirection::Left,
                    state: event.state,
                }),
                Input::MoveForward => commands.trigger(MovementInput {
                    dir: MovementDirection::Forward,
                    state: event.state,
                }),
                Input::MoveRight => commands.trigger(MovementInput {
                    dir: MovementDirection::Right,
                    state: event.state,
                }),
                Input::MoveBackward => commands.trigger(MovementInput {
                    dir: MovementDirection::Backward,
                    state: event.state,
                }),
                Input::TogglePause => {
                    if event.state == ButtonState::Released {
                        commands.trigger(TogglePauseInput);
                    }
                }
            }
        }
    }
}
