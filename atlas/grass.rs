use image::Rgba;
use noise::NoiseFn;
use rand::{rngs::SmallRng, RngCore, SeedableRng};

use super::Noise;

pub struct GrassNoise {
    layers: Vec<(f64, f64, noise::OpenSimplex)>,
}

impl Default for GrassNoise {
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

        GrassNoise { layers }
    }
}

impl Noise for GrassNoise {
    fn get_color(&self, x: f64, y: f64, z: f64, w: f64) -> Rgba<u8> {
        let mut result = 0.0;
        let mut scale = 1.0;
        for (scale_x, scale_y, noise) in &self.layers {
            result += noise.get([x * scale_x, y * scale_x, z * scale_y, w * scale_y]) * scale;
            scale /= 2.0;
        }

        if result < -0.6 {
            Rgba([0x7e, 0x7a, 0x56, u8::MAX])
        } else if result < -0.3 {
            Rgba([0x70, 0x74, 0x52, u8::MAX])
        } else if result < 0.3 {
            Rgba([0x63, 0x6d, 0x4d, u8::MAX])
        } else if result < 0.6 {
            Rgba([0x55, 0x66, 0x48, u8::MAX])
        } else {
            Rgba([0x48, 0x60, 0x44, u8::MAX])
        }
    }
}
