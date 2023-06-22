use image::{ImageBuffer, Rgb, RgbImage};

pub fn halftoning(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    // dither method
    // Bayar pattern
    let pattern: [[u8; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];
    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let threshold: u8 = (pattern[(x % 4) as usize][(y % 4) as usize]) * 16 + 8;
        let img_pixel = img.get_pixel(x, y);
        for n in 0..3 {
            if img_pixel[n] >= threshold {
                pixel[n] = 255;
            } else {
                pixel[n] = 0;
            }
        }
    }
    return imgbuf;
}
