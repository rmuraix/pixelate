use image::{ImageBuffer, Rgb, RgbImage};

const MAX_PIXEL: f64 = 255.0;

/// Apply gamma correction.
///
/// # Arguments
/// * `img` - The input RGB image
/// * `gamma` - Gamma value to use for correction
///
/// # Returns
/// RGB image after gamma correction
pub fn gamma_correct(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    gamma: f64,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let img_pixel: &Rgb<u8> = img.get_pixel(x, y);
        for n in 0..3 {
            let normalized: f64 = img_pixel[n] as f64 / MAX_PIXEL;
            pixel[n] = (MAX_PIXEL * normalized.powf(1.0 / gamma)).round() as u8;
        }
    }
    imgbuf
}
