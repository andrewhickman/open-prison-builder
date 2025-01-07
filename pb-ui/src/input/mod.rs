pub mod settings;

pub use self::settings::Settings;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use pb_engine::EngineState;
use pb_util::run_oneshot_system;

use crate::{camera::CameraInput, input::settings::Action, widget::panel::PanelStack, UiState};

pub fn read(
    mut commands: Commands,
    settings: Res<Settings>,
    mut keyboard_e: EventReader<KeyboardInput>,
    keyboard_state: Res<ButtonInput<KeyCode>>,
    mut camera: ResMut<CameraInput>,
) {
    for event in keyboard_e.read() {
        if let Some(action) = settings.binds.get(&event.key_code) {
            match event.state {
                ButtonState::Pressed if !keyboard_state.just_pressed(event.key_code) => continue,
                ButtonState::Released if !keyboard_state.just_released(event.key_code) => continue,
                _ => (),
            }

            match action {
                Action::Cancel if event.state == ButtonState::Released => {
                    commands.queue(run_oneshot_system(cancel_command))
                }
                Action::PanLeft => camera.pan_left(event.state),
                Action::PanUp => camera.pan_up(event.state),
                Action::PanRight => camera.pan_right(event.state),
                Action::PanDown => camera.pan_down(event.state),
                Action::ZoomIn => camera.zoom_in(event.state),
                Action::ZoomOut => camera.zoom_out(event.state),
                _ => (),
            }
        }
    }
}

pub fn cancel_command(
    mut commands: Commands,
    mut panels: ResMut<PanelStack>,
    engine_state: Res<State<EngineState>>,
    ui_state: Res<State<UiState>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
) {
    if matches!(
        ui_state.get(),
        UiState::LoadingAssets | UiState::LoadingSave
    ) {
        return;
    }

    if let Some(entity) = panels.pop() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
            return;
        }
    }

    if matches!(engine_state.get(), EngineState::Running(_)) {
        match ui_state.get() {
            UiState::Startup | UiState::LoadingAssets | UiState::LoadingSave => (),
            UiState::Game => next_ui_state.set(UiState::Menu),
            UiState::Menu => next_ui_state.set(UiState::Game),
        }
    }
}
