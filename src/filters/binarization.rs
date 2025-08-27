use image::{ImageBuffer, Luma};

const PATTERN_SIZE: u32 = 4;
const THRESHOLD_MULTIPLIER: u8 = 16;
const THRESHOLD_OFFSET: u8 = 8;

/// Perform halftoning using the dither method on a grayscale image.
pub fn halftoning(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let pattern: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

    let mut imgbuf: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let threshold: u8 = pattern[(x % PATTERN_SIZE) as usize][(y % PATTERN_SIZE) as usize]
            * THRESHOLD_MULTIPLIER
            + THRESHOLD_OFFSET;
        let img_pixel: &Luma<u8> = img.get_pixel(x, y);
        let val = img_pixel[0];
        pixel[0] = if val >= threshold { 255 } else { 0 };
    }
    imgbuf
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma};

    fn create_test_image() -> ImageBuffer<Luma<u8>, Vec<u8>> {
        // Generate 4x4 grayscale gradient image
        ImageBuffer::from_fn(4, 4, |x, y| Luma([((x + y) * 32) as u8]))
    }

    #[test]
    fn test_halftoning() {
        let img: ImageBuffer<Luma<u8>, Vec<u8>> = create_test_image();
        let ht: ImageBuffer<Luma<u8>, Vec<u8>> = halftoning(img);
        assert_eq!(ht.dimensions(), (4, 4));
        // Ensure that each pixel has a value of 0 or 255
        for pixel in ht.pixels() {
            let v = pixel[0];
            assert!(v == 0 || v == 255);
        }
    }
}
