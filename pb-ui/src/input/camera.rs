use bevy::{input::ButtonState, prelude::*};
use pb_render::projection::{projection, PIXELS_PER_METER};

use crate::{
    input::{CameraInput, CameraInputKind},
    theme::Theme,
    UiState,
};

/// Camera speed, in metres per second
pub const CAMERA_PAN_SPEED: f32 = 0.5;
pub const CAMERA_ZOOM_SPEED: f32 = 1.;

#[derive(Resource, Default, Debug, Clone, PartialEq)]
pub struct CameraState {
    pan: Vec2,
    zoom: f32,
}

pub fn init(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: theme.background.into(),
            ..Default::default()
        },
        projection(),
        Msaa::Sample4,
    ));
}

pub fn input(
    trigger: Trigger<CameraInput>,
    ui_state: Res<State<UiState>>,
    mut input: ResMut<CameraState>,
) {
    if *ui_state.get() != UiState::Game {
        return;
    }

    match trigger.kind {
        CameraInputKind::PanLeft => input.pan_left(trigger.state),
        CameraInputKind::PanUp => input.pan_up(trigger.state),
        CameraInputKind::PanRight => input.pan_right(trigger.state),
        CameraInputKind::PanDown => input.pan_down(trigger.state),
        CameraInputKind::ZoomIn => input.zoom_in(trigger.state),
        CameraInputKind::ZoomOut => input.zoom_out(trigger.state),
    }
}

pub fn update_condition(input: Res<CameraState>) -> bool {
    *input != CameraState::default()
}

pub fn update(
    input: Res<CameraState>,
    time: Res<Time<Real>>,
    mut camera_transform_q: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut camera_projection) in &mut camera_transform_q {
        input.apply(&mut transform, &mut camera_projection, time.delta_secs());
    }
}

impl CameraState {
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
            * (CAMERA_PAN_SPEED * PIXELS_PER_METER * projection.scale * PIXELS_PER_METER * delta);
        projection.scale = (projection.scale
            + self.zoom * CAMERA_ZOOM_SPEED * delta / PIXELS_PER_METER)
            .clamp(0.1 / PIXELS_PER_METER, 5. / PIXELS_PER_METER);
    }
}

fn delta(state: ButtonState) -> f32 {
    match state {
        ButtonState::Pressed => 1.0,
        ButtonState::Released => -1.0,
    }
}
