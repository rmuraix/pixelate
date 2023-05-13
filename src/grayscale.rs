use std::path::PathBuf;

use image::{self, ImageBuffer, Luma};

pub fn grayscale(path: PathBuf) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let img = image::open(path).unwrap();
    let img = img.to_rgb8();
    let size_x = img.width();
    let size_y = img.height();

    // create data for grayscale
    let mut gray_img = image::GrayImage::new(size_x, size_y);
    let _pix = img.get_pixel(0, 0);

    for y in 0..size_y {
        for x in 0..size_x {
            // Get pixel data
            let pix = img.get_pixel(x, y);
            // GrayScale BT.709
            // V = 0.2126*R + 0.7152*G + 0.0722*B
            let val = [((pix[0] as f32 * 0.2126) as u32
                + (pix[1] as f32 * 0.7152) as u32
                + (pix[2] as f32 * 0.0722) as u32) as u8; 1];
            // Writing pixel data to grayscale data
            gray_img.put_pixel(x, y, image::Luma(val));
        }
    }

    return gray_img;
}
