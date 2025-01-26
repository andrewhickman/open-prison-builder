#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod assets;
mod autosave;
mod input;
mod layout;
mod loading;
mod menu;
mod message;
mod ribbon;
mod theme;
mod widget;

use bevy::{
    input::{keyboard::KeyboardInput, InputSystem},
    picking::PickSet,
    prelude::*,
};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

use input::{camera::CameraState, cancel::CancelStack, picking::PickingState};
use loading::LoadingState;
use pb_engine::EngineState;
use pb_util::set_state;
use ribbon::RibbonState;

use crate::{menu::MenuState, message::Message};

pub struct UiPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum UiState {
    #[default]
    Startup,
    LoadingAssets,
    Menu,
    LoadingSave,
    Game,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TextInputPlugin);

        app.init_state::<UiState>()
            .add_systems(PostStartup, set_state(UiState::LoadingAssets))
            .add_systems(
                PreUpdate,
                assets::update.run_if(in_state(UiState::LoadingAssets)),
            );

        app.add_systems(
            Startup,
            (
                theme::init.chain().after(pb_assets::load),
                layout::init.after(theme::init),
                input::camera::init.after(theme::init),
                input::settings::init.after(pb_store::init),
            ),
        );

        app.init_resource::<CancelStack>()
            .add_systems(
                Update,
                (
                    widget::disabled::update,
                    widget::spinner::update,
                    widget::input::update.after(TextInputSystem),
                ),
            )
            .add_observer(input::cancel::on_add)
            .add_observer(input::cancel::on_remove);

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
            .init_resource::<PickingState>()
            .add_systems(
                PreUpdate,
                (
                    input::read
                        .after(InputSystem)
                        .run_if(on_event::<KeyboardInput>),
                    input::picking::vertex::backend
                        .in_set(PickSet::Backend)
                        .run_if(in_state(UiState::Game)),
                ),
            )
            .add_systems(
                Update,
                input::camera::update.run_if(input::camera::update_condition),
            )
            .add_observer(input::cancel::cancel)
            .add_observer(input::camera::action)
            .add_observer(input::picking::vertex::root_added);
    }
}
