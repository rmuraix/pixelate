//! Per-channel inversion utilities for RGB images.
use image::{ImageBuffer, Rgb, RgbImage};

/// Invert the colors of an image (negative-positive inversion).
///
/// # Arguments
/// * `img` - The input RGB image
///
/// # Returns
/// RGB image after color inversion
pub fn invert_colors(img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let img_pixel: &Rgb<u8> = img.get_pixel(x, y);
        for n in 0..3 {
            pixel[n] = 255 - img_pixel[n];
        }
    }
    imgbuf
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    fn create_test_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::from_fn(2, 2, |x, y| {
            let r: u8 = (x * 120) as u8;
            let g: u8 = (y * 120) as u8;
            let b: u8 = 60;
            Rgb([r, g, b])
        })
    }

    #[test]
    fn test_invert_colors() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let inv: ImageBuffer<Rgb<u8>, Vec<u8>> = invert_colors(&img);
        assert_eq!(inv.dimensions(), (2, 2));
        for (x, y, pixel) in inv.enumerate_pixels() {
            let orig: &Rgb<u8> = img.get_pixel(x, y);
            for i in 0..3 {
                assert_eq!(pixel[i], 255 - orig[i]);
            }
        }
    }
}
