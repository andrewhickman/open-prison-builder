#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod action;
mod autosave;
mod input;
mod layout;
mod loading;
mod menu;
mod message;
mod ribbon;
mod startup;
mod theme;
mod widget;

use bevy::{
    input::{InputSystem, keyboard::KeyboardInput},
    picking::PickSet,
    prelude::*,
};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

use input::{camera::CameraState, cancel::CancelStack};
use loading::LoadingState;
use ribbon::RibbonState;

use crate::{menu::MenuState, message::Message};

pub struct PbUiPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum UiState {
    #[default]
    Startup,
    LoadingAssets,
    Menu,
    LoadingSave,
    Game,
}

impl Plugin for PbUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin);

        input::picking::physics::register(app);

        app.init_state::<UiState>()
            .add_systems(OnEnter(UiState::Game), action::enter_game)
            .add_systems(OnExit(UiState::Game), action::exit_game);

        app.add_systems(
            Startup,
            (
                theme::init.after(pb_assets::load),
                layout::init.after(theme::init),
                input::camera::init.after(theme::init),
            ),
        )
        .add_systems(PostStartup, startup::init);

        app.init_resource::<CancelStack>()
            .add_systems(
                Update,
                (
                    widget::disabled::update,
                    widget::spinner::update,
                    widget::input::update.after(TextInputSystem),
                ),
            )
            .add_observer(input::cancel::cancellable_added)
            .add_observer(input::cancel::cancellable_removed);

        app.add_computed_state::<LoadingState>()
            .add_systems(OnEnter(LoadingState::Shown), loading::show)
            .add_systems(OnEnter(LoadingState::Hidden), loading::hide);

        app.add_computed_state::<MenuState>()
            .add_systems(
                OnEnter(MenuState::Shown),
                (menu::show, menu::update).chain(),
            )
            .add_systems(OnEnter(MenuState::Hidden), menu::hide);

        app.add_computed_state::<RibbonState>()
            .add_systems(OnEnter(RibbonState::Shown), ribbon::show)
            .add_systems(OnEnter(RibbonState::Hidden), ribbon::hide);

        app.add_event::<Message>()
            .add_systems(Update, (message::spawn_messages, message::despawn_messages));

        app.add_systems(PostUpdate, autosave::run.run_if(autosave::run_condition));

        app.init_resource::<CameraState>()
            .add_systems(
                PreUpdate,
                (
                    input::read
                        .after(InputSystem)
                        .run_if(on_event::<KeyboardInput>),
                    input::picking::point::update_hits
                        .in_set(PickSet::Backend)
                        .run_if(in_state(UiState::Game)),
                    input::picking::physics::update_hits
                        .in_set(PickSet::Backend)
                        .run_if(in_state(UiState::Game)),
                ),
            )
            .add_systems(
                Update,
                input::camera::update.run_if(input::camera::update_condition),
            )
            .add_observer(input::cancel::input)
            .add_observer(input::camera::input)
            .add_observer(input::movement::input)
            .add_observer(input::pause::input)
            .add_observer(input::picking::point::grid::input)
            .add_observer(input::picking::point::root_added)
            .add_observer(input::picking::point::grid::grid_added)
            .add_observer(input::picking::physics::pawn::pawn_added)
            .add_observer(input::picking::physics::corner::corner_added)
            .add_observer(input::picking::physics::wall::wall_added)
            .add_observer(action::action_added)
            .add_observer(action::action_removed)
            .add_observer(action::default::cancel);
    }
}
