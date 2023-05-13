use std::path::PathBuf;

use image::{ImageBuffer, Luma};

mod grayscale;

pub fn grayscale(path: PathBuf) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    return grayscale::grayscale(path);
}
