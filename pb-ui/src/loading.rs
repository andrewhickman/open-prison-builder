use bevy::prelude::*;

use crate::{layout::Layout, theme::Theme, widget::UiBuilder, UiState};

#[derive(Default, Copy, Clone, Component)]
pub struct Loading;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LoadingState {
    Shown,
    Hidden,
}

impl ComputedStates for LoadingState {
    type SourceStates = UiState;

    fn compute(source: UiState) -> Option<Self> {
        match source {
            UiState::Startup => None,
            UiState::LoadingAssets | UiState::LoadingSave => Some(LoadingState::Shown),
            UiState::Menu | UiState::Game => Some(LoadingState::Hidden),
        }
    }
}

pub fn show(commands: Commands, layout: Res<Layout>, theme: Res<Theme>) {
    UiBuilder::new(commands, layout.root)
        .spinner(&theme, 60.)
        .insert(Loading);
}

pub fn hide(mut commands: Commands, query: Query<Entity, With<Loading>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
