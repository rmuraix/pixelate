use image::{ImageBuffer, Luma, Rgb};

/// Convert an RGB image to grayscale with the specified channel weights.
///
/// # Arguments
/// * `img` - The input image
/// * `red` - red channel weights
/// * `green` - weights for the green channel
/// * `blue` - weights for the blue channel
///
/// # Returns
/// Grayscale image
pub fn grayscale(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    red: f64,
    green: f64,
    blue: f64,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width: u32 = img.width();
    let height: u32 = img.height();
    let mut gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = image::GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pix: &Rgb<u8> = img.get_pixel(x, y);
            let gray_value: u8 =
                (pix[0] as f64 * red + pix[1] as f64 * green + pix[2] as f64 * blue)
                    .round()
                    .min(255.0) as u8;
            gray_img.put_pixel(x, y, image::Luma([gray_value]));
        }
    }
    gray_img
}
