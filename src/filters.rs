use std::path::PathBuf;

use image::{ImageBuffer, Luma};

mod binarization;
mod grayscale;

pub fn grayscale(path: PathBuf, red: f64, green: f64, blue: f64) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    return grayscale::grayscale(path, red, green, blue);
}
pub fn halftoning(path: PathBuf) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    return binarization::halftoning(path);
}
