//! This is a slight modification of [mblode](https://github.com/mblode)'s
//! [blue-noise code](https://github.com/mblode/blue-noise/blob/main/src/main.rs)
//! with some small performance improvements and other adjustments.

use image::{ImageBuffer, Luma, Rgb, RgbImage, Rgba, RgbaImage};

lazy_static::lazy_static! {
    static ref NOISE_IMG: ImageBuffer<Luma<u8>, Vec<u8>> =
        image::load_from_memory(include_bytes!("../img/noise.png")).expect("Load noise.png").grayscale().into_luma8();
    static ref NOISE_WIDTH: u32 = NOISE_IMG.dimensions().0;
    static ref NOISE_HEIGHT: u32 = NOISE_IMG.dimensions().1;
}

#[inline]
fn is_bright(noise_color: &Luma<u8>, picture_color: &Luma<u8>) -> bool {
    let noise_luma = noise_color.0;
    let picture_luma = picture_color.0;
    if picture_luma[0] > noise_luma[0] {
        true
    } else {
        false
    }
}

#[inline]
fn wrap(m: u32, n: u32) -> u32 {
    return n % m;
}

pub fn dither_image(
    input_buffer: Vec<u8>,
    input_width: u32,
    input_height: u32,
    scale_factor: u32,
) -> RgbaImage {
    // RgbImage == ImageBuffer<Rgb<u8>, Vec<u8>>
    let old_img: ImageBuffer<Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::from_vec(input_width, input_height, input_buffer).unwrap();
    let old_img = image::DynamicImage::ImageRgb8(old_img);
    let mut old_img = old_img.grayscale();
    let old_img = old_img.as_mut_luma8().unwrap();
    let (old_width, old_height) = old_img.dimensions();

    let mut new_img = RgbaImage::new(old_width * scale_factor, old_height * scale_factor);

    for x in 0..old_width {
        for y in 0..old_height {
            let wrap_x = wrap(*NOISE_WIDTH, x);
            let wrap_y = wrap(*NOISE_HEIGHT, y);

            let noise_pixel = NOISE_IMG.get_pixel(wrap_x, wrap_y);
            let old_pixel = old_img.get_pixel_mut(x, y);

            if is_bright(noise_pixel, old_pixel) {
                new_img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            } else {
                new_img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }
    }

    //new_img.save("/tmp/frame.bmp").unwrap();
    new_img
}
