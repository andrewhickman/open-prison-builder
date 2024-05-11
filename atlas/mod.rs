use std::{f64, fs, io::Cursor, path::Path};

use image::{ImageBuffer, ImageFormat, Rgba};

mod dirt;
mod grass;

pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 1024;

pub fn write_atlas(path: impl AsRef<Path>) {
    let mut image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(WIDTH, HEIGHT * 2);

    write_noise::<dirt::DirtNoise>(&mut image, 0);
    write_noise::<grass::GrassNoise>(&mut image, 1);

    let mut buf = Vec::new();
    image.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png).unwrap();
    if fs::read(path.as_ref()).unwrap() != buf {
        fs::write(path, buf).unwrap();
    }
}

trait Noise: Default {
    fn get_color(&self, x: f64, y: f64, z: f64, w: f64) -> Rgba<u8>;
}

fn write_noise<N: Noise>(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, offset: u32) {
    let noise = N::default();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let x1 = 0.0;
            let x2 = 2.0;
            let y1 = 0.0;
            let y2 = 2.0;
            let dx = x2 - x1;
            let dy = y2 - y1;

            let s = x as f64 / WIDTH as f64;
            let t = y as f64 / HEIGHT as f64;

            let nx = x1 + f64::cos(s * f64::consts::TAU) * (dx / f64::consts::TAU);
            let ny = y1 + f64::cos(t * f64::consts::TAU) * (dy / f64::consts::TAU);
            let nz = x1 + f64::sin(s * f64::consts::TAU) * (dx / f64::consts::TAU);
            let nw = y1 + f64::sin(t * f64::consts::TAU) * (dy / f64::consts::TAU);

            *image.get_pixel_mut(x, offset * HEIGHT + y) = noise.get_color(nx, ny, nz, nw);
        }
    }
}
