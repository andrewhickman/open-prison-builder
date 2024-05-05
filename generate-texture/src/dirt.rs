use bevy_render::color::Color;
use noise::NoiseFn;
use noise::SuperSimplex;
use rand::{rngs::SmallRng, RngCore, SeedableRng};

use crate::{Noise, HEIGHT, WIDTH};

pub struct DirtNoise {
    layers: Vec<(f64, f64, noise::OpenSimplex)>,
}

impl Default for DirtNoise {
    fn default() -> Self {
        let mut rand = SmallRng::seed_from_u64(42);

        let layers = (0..4)
            .map(|n| {
                let scale = f64::powi(2.0, 4 + n);
                (
                    scale * 2.0,
                    scale * 2.0,
                    noise::OpenSimplex::new(rand.next_u32()),
                )
            })
            .collect();

        DirtNoise { layers }
    }
}

impl Noise for DirtNoise {
    fn get_color(&self, x: f64, y: f64, z: f64, w: f64) -> Color {
        let mut result = 0.0;
        let mut scale = 1.0;
        for (scale_x, scale_y, noise) in &self.layers {
            result += noise.get([x * scale_x, y * scale_x, z * scale_y, w * scale_y]) * scale;
            scale /= 2.0;
        }

        if result < -0.1 {
            Color::rgb(0.278, 0.243, 0.161)
        } else if result < 0.1 {
            Color::rgb(0.247, 0.200, 0.145)
        } else {
            Color::rgb(0.278, 0.243, 0.161)
        }
    }
}
