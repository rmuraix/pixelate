//! Grayscale conversion utilities.
use image::{ImageBuffer, Luma, Rgb};

/// Convert an RGB image to grayscale with the specified channel weights.
///
/// # Arguments
/// * `img` - The input image
/// * `red` - Red channel weight
/// * `green` - Green channel weight
/// * `blue` - Blue channel weight
///
/// # Returns
/// Grayscale image
pub fn grayscale(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    red: f64,
    green: f64,
    blue: f64,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width: u32 = img.width();
    let height: u32 = img.height();
    let mut gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = image::GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pix: &Rgb<u8> = img.get_pixel(x, y);
            let gray_value: u8 =
                (pix[0] as f64 * red + pix[1] as f64 * green + pix[2] as f64 * blue)
                    .round()
                    .min(255.0) as u8;
            gray_img.put_pixel(x, y, image::Luma([gray_value]));
        }
    }
    gray_img
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma, Rgb};

    fn create_test_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::from_fn(2, 2, |x, y| {
            let r: u8 = (x * 100) as u8;
            let g: u8 = (y * 100) as u8;
            let b: u8 = 50;
            Rgb([r, g, b])
        })
    }

    #[test]
    fn test_grayscale() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let gray: ImageBuffer<Luma<u8>, Vec<u8>> = grayscale(&img, 0.3, 0.59, 0.11);

        assert_eq!(gray.dimensions(), (2, 2));

        // Calculate and compare the expected grayscale value of each pixel
        for (x, y, pixel) in gray.enumerate_pixels() {
            let orig_pixel: &Rgb<u8> = img.get_pixel(x, y);
            let expected_value: u8 = (orig_pixel[0] as f64 * 0.3
                + orig_pixel[1] as f64 * 0.59
                + orig_pixel[2] as f64 * 0.11)
                .round() as u8;

            let Luma([val]) = *pixel;
            assert_eq!(val, expected_value, "Mismatch at pixel ({}, {})", x, y);
        }
    }
}
