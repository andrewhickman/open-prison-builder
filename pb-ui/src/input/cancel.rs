use bevy::prelude::*;
use pb_engine::EngineState;

use crate::{UiState, input::CancelInput, widget::UiBuilder};

#[derive(Debug, Default, Resource)]
pub struct CancelStack {
    entities: Vec<Entity>,
}

#[derive(Default, Copy, Clone, Component)]
pub struct Cancellable;

pub fn cancellable_added(trigger: Trigger<OnAdd, Cancellable>, mut stack: ResMut<CancelStack>) {
    stack.entities.push(trigger.target());
}

pub fn cancellable_removed(
    trigger: Trigger<OnRemove, Cancellable>,
    mut stack: ResMut<CancelStack>,
) {
    if let Some(pos) = stack.entities.iter().position(|&e| e == trigger.target()) {
        stack.entities.remove(pos);
    }
}

impl CancelStack {
    pub fn pop(&mut self) -> Option<Entity> {
        self.entities.pop()
    }
}

impl<'w> UiBuilder<'w, '_> {
    pub fn cancellable(&mut self) -> UiBuilder<'w, '_> {
        self.insert(Cancellable)
    }
}

pub fn input(
    _: Trigger<CancelInput>,
    mut commands: Commands,
    mut stack: ResMut<CancelStack>,
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

    if let Some(entity) = stack.pop() {
        if let Ok(mut entity) = commands.get_entity(entity) {
            entity.despawn();
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
