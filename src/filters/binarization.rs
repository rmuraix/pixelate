use image::{ImageBuffer, Rgb, RgbImage};

const PATTERN_SIZE: u32 = 4;
const THRESHOLD_MULTIPLIER: u8 = 16;
const THRESHOLD_OFFSET: u8 = 8;

/// Perform halftoning using the dither method.
pub fn halftoning(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let pattern: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let threshold: u8 = pattern[(x % PATTERN_SIZE) as usize][(y % PATTERN_SIZE) as usize]
            * THRESHOLD_MULTIPLIER
            + THRESHOLD_OFFSET;
        let img_pixel: &Rgb<u8> = img.get_pixel(x, y);
        for n in 0..3 {
            pixel[n] = if img_pixel[n] >= threshold { 255 } else { 0 };
        }
    }
    imgbuf
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    fn create_test_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        // Generate 4x4 gradient image
        ImageBuffer::from_fn(4, 4, |x, y| Rgb([x as u8 * 60, y as u8 * 60, 100]))
    }

    #[test]
    fn test_halftoning() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let ht: ImageBuffer<Rgb<u8>, Vec<u8>> = halftoning(img);
        assert_eq!(ht.dimensions(), (4, 4));
        // Ensure that each pixel has a value of 0 or 255
        for pixel in ht.pixels() {
            for &v in pixel.0.iter() {
                assert!(v == 0 || v == 255);
            }
        }
    }
}
