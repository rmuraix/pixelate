use std::path::PathBuf;

use image::{ImageBuffer, Luma, Rgb};

mod binarization;
mod grayscale;

pub fn grayscale(path: PathBuf, red: f64, green: f64, blue: f64) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    return grayscale::grayscale(path, red, green, blue);
}
pub fn halftoning(path: PathBuf) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    return binarization::halftoning(path);
}
