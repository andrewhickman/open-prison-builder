#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod autosave;
mod camera;
mod input;
mod layout;
mod loading;
mod menu;
mod message;
mod startup;
mod theme;
mod widget;

use bevy::{
    input::{keyboard::KeyboardInput, InputSystem},
    prelude::*,
};
use bevy_mod_picking::prelude::*;
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

use camera::CameraInput;
use pb_engine::EngineState;
use pb_util::set_state;
use widget::panel::PanelStack;

use crate::{
    menu::MenuState,
    message::Message,
    startup::StartupState,
    widget::form::{FormSubmit, FormUpdate},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPickingPlugins,
            TextInputPlugin,
            EventListenerPlugin::<FormUpdate>::default(),
            EventListenerPlugin::<FormSubmit>::default(),
        ));

        app.add_systems(
            Startup,
            (
                theme::init.after(pb_assets::load),
                (camera::init, layout::init).after(theme::init),
            ),
        );

        app.init_state::<StartupState>().add_systems(
            PreUpdate,
            startup::update.run_if(in_state(StartupState::Pending)),
        );

        app.init_resource::<CameraInput>()
            .add_systems(
                PreUpdate,
                input::read
                    .after(InputSystem)
                    .run_if(in_state(StartupState::Ready).and_then(on_event::<KeyboardInput>())),
            )
            .add_systems(Update, camera::update);

        app.add_systems(Update, (widget::button::update, widget::spinner::update));

        app.add_systems(Update, widget::input::update.after(TextInputSystem));

        app.init_resource::<PanelStack>()
            .add_systems(Update, widget::panel::update);

        app.init_state::<MenuState>()
            .add_systems(
                OnEnter(MenuState::Shown),
                (menu::show, menu::update).chain(),
            )
            .add_systems(OnEnter(MenuState::Hidden), menu::hide);

        app.add_systems(OnEnter(StartupState::Pending), loading::enter)
            .add_systems(
                OnExit(StartupState::Pending),
                (loading::exit, set_state(MenuState::Shown)),
            );

        app.add_systems(OnEnter(EngineState::Loading), loading::enter)
            .add_systems(OnExit(EngineState::Loading), loading::exit);

        app.add_event::<Message>()
            .add_systems(Update, (message::spawn_messages, message::despawn_messages));

        app.add_systems(PostUpdate, autosave::run.run_if(autosave::run_condition));
    }
}
