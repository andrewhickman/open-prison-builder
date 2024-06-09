#![allow(clippy::type_complexity)]

mod loading;
mod menu;
mod node;
mod theme;
mod widget;

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_simple_text_input::TextInputPlugin;
use pb_assets::LoadState;

use crate::menu::MenuState;
use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum EngineState {
    #[default]
    Disabled,
    Loading,
    Running,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);
        app.add_plugins(TextInputPlugin);

        app.add_systems(
            Startup,
            (
                theme::init.after(pb_assets::load),
                (init_camera, node::init).after(theme::init),
            ),
        );

        app.add_systems(Update, (widget::button::update, widget::spinner::update));

        app.init_state::<MenuState>();
        app.add_systems(OnEnter(MenuState::Shown), menu::show);
        app.add_systems(OnEnter(MenuState::Hidden), menu::hide);

        app.add_systems(OnEnter(LoadState::Pending), loading::enter);
        app.add_systems(OnExit(LoadState::Pending), loading::exit);

        app.init_state::<EngineState>();
        app.add_systems(OnEnter(EngineState::Loading), loading::enter);
        app.add_systems(OnExit(EngineState::Loading), loading::exit);
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
