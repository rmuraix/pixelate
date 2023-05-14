use std::path::PathBuf;

use image::{ImageBuffer, Luma};

mod binarization;
mod grayscale;

pub fn grayscale(path: PathBuf) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    return grayscale::grayscale(path);
}
pub fn halftoning(path: PathBuf) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    return binarization::halftoning(path);
}
