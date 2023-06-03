use image::{ImageBuffer, Rgb, RgbImage};
use std::path::PathBuf;

pub fn main(path: PathBuf, gamma: f64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img: image::DynamicImage = image::open(path).unwrap();
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();

    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let img_pixel = img.get_pixel(x, y);
        for n in 0..3 {
            pixel[n] = (255.0 * ((img_pixel[n] as f64 / 255.0).powf(1.0 / gamma))) as u8;
        }
    }
    return imgbuf;
}
