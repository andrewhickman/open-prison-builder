use bevy::{input::ButtonState, prelude::*, render::camera::ScalingMode};

use crate::theme::Theme;

/// Camera speed, in metres per second
pub const CAMERA_MOVE_SPEED: f32 = 1.;
pub const CAMERA_PIXELS_PER_METER: f32 = 64.;

#[derive(Resource, Default, Debug, Clone, PartialEq)]
pub struct CameraInput {
    pan: Vec2,
    zoom: f32,
}

pub fn init(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: theme.background.into(),
            ..Default::default()
        },
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: ScalingMode::WindowSize(CAMERA_PIXELS_PER_METER),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn update(
    input: Res<CameraInput>,
    time: Res<Time<Real>>,
    mut camera_transform_q: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    if *input == CameraInput::default() {
        return;
    }

    for (mut transform, mut camera_projection) in &mut camera_transform_q {
        input.apply(&mut transform, &mut camera_projection, time.delta_seconds());
    }
}

impl CameraInput {
    pub fn pan_left(&mut self, state: ButtonState) {
        self.pan.x -= delta(state);
    }

    pub fn pan_up(&mut self, state: ButtonState) {
        self.pan.y += delta(state);
    }

    pub fn pan_down(&mut self, state: ButtonState) {
        self.pan.y -= delta(state);
    }

    pub fn pan_right(&mut self, state: ButtonState) {
        self.pan.x += delta(state);
    }

    pub fn zoom_in(&mut self, state: ButtonState) {
        self.zoom -= delta(state);
    }

    pub fn zoom_out(&mut self, state: ButtonState) {
        self.zoom += delta(state);
    }

    pub fn apply(
        &self,
        transform: &mut Transform,
        projection: &mut OrthographicProjection,
        delta: f32,
    ) {
        transform.translation += self.pan.extend(0.)
            * (CAMERA_MOVE_SPEED * CAMERA_PIXELS_PER_METER * projection.scale * delta);
        projection.scale = (projection.scale + self.zoom * delta).clamp(0.1, 5.);
    }
}

fn delta(state: ButtonState) -> f32 {
    match state {
        ButtonState::Pressed => 1.0,
        ButtonState::Released => -1.0,
    }
}
