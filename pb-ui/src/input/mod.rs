pub mod settings;

pub use self::settings::Settings;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use pb_engine::EngineState;

use crate::{input::settings::Action, widget::panel::PanelStack, UiState};

#[derive(Event, Debug, Clone, Copy)]
pub struct CancelAction;

#[derive(Event, Debug, Clone, Copy)]
pub struct CameraAction {
    pub kind: CameraActionKind,
    pub state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraActionKind {
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
                    commands.trigger(CancelAction);
                }
                Action::PanLeft => commands.trigger(CameraAction {
                    kind: CameraActionKind::PanLeft,
                    state: event.state,
                }),
                Action::PanUp => commands.trigger(CameraAction {
                    kind: CameraActionKind::PanUp,
                    state: event.state,
                }),
                Action::PanRight => commands.trigger(CameraAction {
                    kind: CameraActionKind::PanRight,
                    state: event.state,
                }),
                Action::PanDown => commands.trigger(CameraAction {
                    kind: CameraActionKind::PanDown,
                    state: event.state,
                }),
                Action::ZoomIn => commands.trigger(CameraAction {
                    kind: CameraActionKind::ZoomIn,
                    state: event.state,
                }),
                Action::ZoomOut => commands.trigger(CameraAction {
                    kind: CameraActionKind::ZoomOut,
                    state: event.state,
                }),
                _ => (),
            }
        }
    }
}

pub fn cancel(
    _: Trigger<CancelAction>,
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
