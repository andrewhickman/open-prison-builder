use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub fn diff_image(expected: &DynamicImage, actual: &DynamicImage) -> (u64, DynamicImage) {
    assert_eq!(expected.dimensions(), actual.dimensions());

    let width = actual.width();
    let height = actual.height();
    let mut result = DynamicImage::new_rgba8(width, height);

    let mut error = 0;

    for y in 0..height {
        for x in 0..width {
            let expected_pixel: Rgba<u8> = expected.get_pixel(x, y);
            let actual_pixel: Rgba<u8> = actual.get_pixel(x, y);

            if expected_pixel == actual_pixel {
                result.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            } else {
                result.put_pixel(x, y, Rgba([u8::MAX, 0, 0, u8::MAX]));
            }

            error += squared_error(expected_pixel[0], actual_pixel[0])
                + squared_error(expected_pixel[1], actual_pixel[1])
                + squared_error(expected_pixel[2], actual_pixel[2])
                + squared_error(expected_pixel[3], actual_pixel[3]);
        }
    }

    (error, result)
}

fn squared_error(expected: u8, actual: u8) -> u64 {
    let err = actual.abs_diff(expected) as u64;
    err * err
}
