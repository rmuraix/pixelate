//! Filter traits and built-in filters used by the CLI and library users.
//!
//! Each filter implements the generic [`crate::filters::Filter`] trait with concrete input and
//! output image types. Filters consume an input by reference and return a new
//! image buffer without mutating the original.
use crate::color::{SRGB_LUMA_B, SRGB_LUMA_G, SRGB_LUMA_R};
use image::{ImageBuffer, Luma, Rgb};

/// Generic trait for applying image filters.
///
/// `I` is the input type (typically an `ImageBuffer`), and `O` is the output
/// type. Implementations must be pure: they should not mutate the input and
/// should return a fresh buffer.
pub trait Filter<I, O> {
    fn apply(&self, input: &I) -> O;
}

mod convolution;
mod dither;
mod gamma;
mod grayscale;
mod invert;
mod sobel;

/// Convert an RGB image to grayscale using weighted channel luminance.
pub struct GrayscaleFilter {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl GrayscaleFilter {
    /// Create a new grayscale filter with provided channel weights.
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl Filter<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageBuffer<Luma<u8>, Vec<u8>>> for GrayscaleFilter {
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        grayscale::grayscale(img, self.red, self.green, self.blue)
    }
}

/// Ordered-dither (Bayer matrix) halftone on luminance; outputs black and white.
pub struct HalftoneFilter;

impl Filter<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageBuffer<Luma<u8>, Vec<u8>>> for HalftoneFilter {
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        // Convert to grayscale before applying halftoning
        let gray: ImageBuffer<Luma<u8>, Vec<u8>> =
            grayscale::grayscale(img, SRGB_LUMA_R, SRGB_LUMA_G, SRGB_LUMA_B);
        dither::halftoning(&gray)
    }
}

/// Gamma-correction filter for RGB images.
pub struct GammaFilter {
    pub gamma: f64,
}

impl GammaFilter {
    /// Create a new gamma-correction filter with the given `gamma` value.
    pub fn new(gamma: f64) -> Self {
        Self { gamma }
    }
}

impl Filter<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageBuffer<Rgb<u8>, Vec<u8>>> for GammaFilter {
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        gamma::gamma_correct(img, self.gamma)
    }
}

/// Per-channel inversion (negative) for RGB images.
pub struct InvertFilter;

impl Filter<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageBuffer<Rgb<u8>, Vec<u8>>> for InvertFilter {
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        invert::invert_colors(img)
    }
}

/// Sobel edge detection (magnitude of gradient) for RGB images.
pub struct SobelFilter {
    /// Intensity multiplier applied after normalization (>= 0.0).
    pub intensity: f64,
}

impl SobelFilter {
    /// Create a new Sobel filter with the given intensity multiplier.
    pub fn new(intensity: f64) -> Self {
        Self { intensity }
    }
}

impl Filter<ImageBuffer<Rgb<u8>, Vec<u8>>, ImageBuffer<Luma<u8>, Vec<u8>>> for SobelFilter {
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        sobel::sobel_edges(img, self.intensity as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma, Rgb};

    /// Generate a 3x3 RGB image for testing.
    fn create_test_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::from_fn(3, 3, |x, y| {
            // R = x * 50, G = y * 50, B = 100
            Rgb([(x * 50) as u8, (y * 50) as u8, 100])
        })
    }

    #[test]
    fn test_grayscale_filter() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let filter: GrayscaleFilter = GrayscaleFilter::new(0.3, 0.59, 0.11);
        let gray_img: ImageBuffer<Luma<u8>, Vec<u8>> =
            <GrayscaleFilter as Filter<_, _>>::apply(&filter, &img);

        assert_eq!(gray_img.dimensions(), (3, 3));

        // Calculate and compare the expected grayscale value of each pixel
        for (x, y, pixel) in gray_img.enumerate_pixels() {
            let orig_pixel = img.get_pixel(x, y);
            let expected_value = (orig_pixel[0] as f64 * 0.3
                + orig_pixel[1] as f64 * 0.59
                + orig_pixel[2] as f64 * 0.11)
                .round() as u8;

            let Luma([val]) = *pixel;
            assert_eq!(val, expected_value, "Mismatch at pixel ({}, {})", x, y);
        }
    }

    #[test]
    fn test_halftone_filter() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let filter: HalftoneFilter = HalftoneFilter;
        let ht_img: ImageBuffer<Luma<u8>, Vec<u8>> =
            <HalftoneFilter as Filter<_, _>>::apply(&filter, &img);
        assert_eq!(ht_img.dimensions(), (3, 3));
    }

    #[test]
    fn test_gamma_filter() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let filter: GammaFilter = GammaFilter::new(2.2);
        let gamma_img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            <GammaFilter as Filter<_, _>>::apply(&filter, &img);
        assert_eq!(gamma_img.dimensions(), (3, 3));
    }

    #[test]
    fn test_invert_filter() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let filter: InvertFilter = InvertFilter;
        let inv_img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            <InvertFilter as Filter<_, _>>::apply(&filter, &img);
        assert_eq!(inv_img.dimensions(), (3, 3));
        // Check that each pixel is correctly inverted
        for (x, y, pixel) in inv_img.enumerate_pixels() {
            let orig = img.get_pixel(x, y);
            for i in 0..3 {
                assert_eq!(pixel[i], 255 - orig[i]);
            }
        }
    }
}
