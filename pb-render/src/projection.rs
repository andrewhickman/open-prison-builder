use bevy::render::camera::{OrthographicProjection, Projection, ScalingMode};

pub const PIXELS_PER_METER: f32 = 64.;

pub fn projection() -> Projection {
    Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::WindowSize,
        scale: PIXELS_PER_METER.recip(),
        ..OrthographicProjection::default_2d()
    })
}

pub trait ProjectionExt {
    fn scale(&self) -> f32;

    fn set_scale(&mut self, scale: f32);
}

impl ProjectionExt for Projection {
    fn scale(&self) -> f32 {
        match self {
            Projection::Orthographic(projection) => projection.scale,
            _ => panic!("unexpected projection"),
        }
    }

    fn set_scale(&mut self, scale: f32) {
        match self {
            Projection::Orthographic(projection) => projection.scale = scale,
            _ => panic!("unexpected projection"),
        }
    }
}
