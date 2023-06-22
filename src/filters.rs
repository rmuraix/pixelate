use image::{ImageBuffer, Luma, Rgb};

mod binarization;
mod gamma;
mod grayscale;
mod negaposi;

pub fn grayscale(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    red: f64,
    green: f64,
    blue: f64,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    grayscale::grayscale(img, red, green, blue)
}
pub fn halftoning(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    binarization::halftoning(img)
}
pub fn gamma(img: ImageBuffer<Rgb<u8>, Vec<u8>>, gamma: f64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    gamma::main(img, gamma)
}
pub fn negaposi(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    negaposi::main(img)
}
