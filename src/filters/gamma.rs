//! Gamma correction utilities for RGB images.
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
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    fn create_test_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        // Generate 3x3 test images
        ImageBuffer::from_fn(3, 3, |x, y| Rgb([x as u8 * 50, y as u8 * 50, 150]))
    }

    #[test]
    fn test_gamma_correct() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let gamma_value = 2.2;
        let gamma_img: ImageBuffer<Rgb<u8>, Vec<u8>> = gamma_correct(&img, gamma_value);

        assert_eq!(gamma_img.dimensions(), (3, 3));

        // Hand-calculate gamma-corrected values and compare with expected values
        for (x, y, pixel) in gamma_img.enumerate_pixels() {
            let original = img.get_pixel(x, y);
            for i in 0..3 {
                let normalized = original[i] as f64 / 255.0;
                let expected = (255.0 * normalized.powf(1.0 / gamma_value)).round() as u8;
                assert_eq!(
                    pixel[i], expected,
                    "Mismatch at pixel ({}, {}), channel {}",
                    x, y, i
                );
            }
        }
    }
}
