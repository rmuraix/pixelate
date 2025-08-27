//! Sobel edge detection built on top of reusable convolution.
use image::{ImageBuffer, Luma, Rgb};

use super::convolution::{convolve_gray_f32, magnitude_to_luma_u8_scaled};
use super::grayscale;

const SOBEL_X: [[f32; 3]; 3] = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];

const SOBEL_Y: [[f32; 3]; 3] = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

/// Apply Sobel edge detection to an RGB image.
/// Internally converts to grayscale, computes Gx and Gy, and outputs magnitude.
pub fn sobel_edges(
    img: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    intensity: f32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let gray: ImageBuffer<Luma<u8>, Vec<u8>> = grayscale::grayscale(
        img,
        crate::color::SRGB_LUMA_R,
        crate::color::SRGB_LUMA_G,
        crate::color::SRGB_LUMA_B,
    );

    let (w, h) = gray.dimensions();
    let gx: Vec<f32> = convolve_gray_f32::<3>(&gray, &SOBEL_X);
    let gy: Vec<f32> = convolve_gray_f32::<3>(&gray, &SOBEL_Y);
    magnitude_to_luma_u8_scaled(&gx, &gy, w, h, intensity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    #[test]
    fn detects_vertical_edge() {
        // 6x3 image: left half dark, right half bright -> vertical edge around x=2/3
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(6, 3, |x, _y| {
            if x < 3 {
                Rgb([10, 10, 10])
            } else {
                Rgb([240, 240, 240])
            }
        });
        let edges = sobel_edges(&img, 1.0);
        assert_eq!(edges.dimensions(), (6, 3));
        // Edge strength around the middle columns should be higher than corners
        let left_val = edges.get_pixel(0, 1)[0];
        let edge_val = edges.get_pixel(3, 1)[0];
        assert!(edge_val > left_val);
    }
}
