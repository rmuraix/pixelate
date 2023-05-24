use std::path::PathBuf;

use image::{ImageBuffer, Luma};

use super::grayscale;

pub fn halftoning(path: PathBuf) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = grayscale(path, 0.2126, 0.7152, 0.0722);
    let size_x: u32 = gray_img.width();
    let size_y: u32 = gray_img.height();
    // dither method
    // Bayar pattern
    let pattern: [[u32; 4]; 4] = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];
    for y in 0..size_y {
        for x in 0..size_x {
            let pixel: &Luma<u8> = gray_img.get_pixel(x, y);
            let threshold: u32 = (pattern[(x % 4) as usize][(y % 4) as usize]) * 16 + 8;
            if pixel[0] as u32 >= threshold {
                let val = [255 as u8; 1];
                gray_img.put_pixel(x, y, image::Luma(val));
            } else {
                let val = [0 as u8; 1];
                // Writing pixel data
                gray_img.put_pixel(x, y, image::Luma(val));
            }
        }
    }
    return gray_img;
}
