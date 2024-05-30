mod menu;
mod node;
mod theme;

mod widget;

use bevy::{
    app::{App, Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        schedule::OnEnter,
        system::{Commands, Res},
    },
    render::camera::Camera,
};
use menu::MenuState;

use crate::theme::Theme;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Theme>();

        app.add_systems(Startup, (spawn_camera, node::spawn));

        app.init_state::<MenuState>();
        app.add_systems(OnEnter(MenuState::Shown), menu::show);
        app.add_systems(OnEnter(MenuState::Hidden), menu::hide);
    }
}

fn spawn_camera(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: theme.background.into(),
            ..Default::default()
        },
        ..Default::default()
    });
}
