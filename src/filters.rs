use image::{ImageBuffer, Luma, Rgb};

/// Common trait for applying image filters.
pub trait Filter {
    type Output;
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self::Output;
}

mod binarization;
mod gamma;
mod grayscale;
mod invert;

pub struct GrayscaleFilter {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl GrayscaleFilter {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl Filter for GrayscaleFilter {
    type Output = ImageBuffer<Luma<u8>, Vec<u8>>;
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self::Output {
        grayscale::grayscale(img.clone(), self.red, self.green, self.blue)
    }
}

pub struct HalftoneFilter;

impl Filter for HalftoneFilter {
    type Output = ImageBuffer<Rgb<u8>, Vec<u8>>;
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self::Output {
        binarization::halftoning(img.clone())
    }
}

pub struct GammaFilter {
    pub gamma: f64,
}

impl GammaFilter {
    pub fn new(gamma: f64) -> Self {
        Self { gamma }
    }
}

impl Filter for GammaFilter {
    type Output = ImageBuffer<Rgb<u8>, Vec<u8>>;
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self::Output {
        gamma::gamma_correct(img.clone(), self.gamma)
    }
}

pub struct InvertFilter;

impl Filter for InvertFilter {
    type Output = ImageBuffer<Rgb<u8>, Vec<u8>>;
    fn apply(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Self::Output {
        invert::invert_colors(img.clone())
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
        let gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = filter.apply(&img);

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
        let ht_img: ImageBuffer<Rgb<u8>, Vec<u8>> = filter.apply(&img);
        assert_eq!(ht_img.dimensions(), (3, 3));
    }

    #[test]
    fn test_gamma_filter() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let filter: GammaFilter = GammaFilter::new(2.2);
        let gamma_img: ImageBuffer<Rgb<u8>, Vec<u8>> = filter.apply(&img);
        assert_eq!(gamma_img.dimensions(), (3, 3));
    }

    #[test]
    fn test_invert_filter() {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = create_test_image();
        let filter: InvertFilter = InvertFilter;
        let inv_img: ImageBuffer<Rgb<u8>, Vec<u8>> = filter.apply(&img);
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
