#![allow(clippy::type_complexity, clippy::too_many_arguments)]

mod input;
mod layout;
mod loading;
mod menu;
mod message;
mod theme;
mod widget;

use bevy::input::InputSystem;
use bevy::prelude::*;
use bevy_mod_picking::prelude::EventListenerPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use input::{CameraCommand, ToggleMenuCommand};
use message::Message;
use pb_assets::LoadState;
use pb_engine::EngineState;
use pb_util::set_state;
use widget::form::{FormSubmit, FormUpdate};

use crate::menu::MenuState;
use crate::theme::Theme;

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
                (init_camera, layout::init).after(theme::init),
            ),
        );

        app.add_event::<ToggleMenuCommand>();
        app.init_resource::<CameraCommand>();
        app.add_systems(PreUpdate, input::update.after(InputSystem));

        app.add_systems(Update, (widget::button::update, widget::spinner::update));
        app.add_systems(Update, widget::input::update.after(TextInputSystem));

        app.init_state::<MenuState>();
        app.add_systems(OnEnter(MenuState::Shown), (menu::show, menu::update));
        app.add_systems(OnEnter(MenuState::Hidden), menu::hide);
        app.add_systems(Update, menu::toggle);

        app.add_systems(OnEnter(LoadState::Pending), loading::enter);
        app.add_systems(
            OnExit(LoadState::Pending),
            (loading::exit, set_state(MenuState::Shown)),
        );

        app.add_systems(OnEnter(EngineState::Loading), loading::enter);
        app.add_systems(OnExit(EngineState::Loading), loading::exit);

        app.add_event::<Message>();
        app.add_systems(Update, (message::spawn_messages, message::despawn_messages));
    }
}

fn init_camera(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: theme.background.into(),
            ..Default::default()
        },
        ..Default::default()
    });
}
