mod menu;
mod node;
mod theme;

mod widget;

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

use crate::menu::MenuState;
use crate::theme::Theme;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins);

        app.add_systems(
            Startup,
            (
                theme::init.after(pb_assets::load),
                (init_camera, node::init).after(theme::init),
            ),
        );

        app.init_state::<MenuState>();
        app.add_systems(OnEnter(MenuState::Shown), menu::show);
        app.add_systems(OnEnter(MenuState::Hidden), menu::hide);
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
