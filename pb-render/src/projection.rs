use bevy::render::camera::{OrthographicProjection, ScalingMode};

pub const PIXELS_PER_METER: f32 = 64.;

pub fn projection() -> OrthographicProjection {
    OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        scale: PIXELS_PER_METER.recip(),
        ..OrthographicProjection::default_2d()
    }
}
