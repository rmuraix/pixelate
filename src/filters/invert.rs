use image::{ImageBuffer, Rgb, RgbImage};

/// Invert the colors of an image (negative-positive inversion).
///
/// # Arguments
/// * `img` - the input RGB image
///
/// # Returns
/// RGB image after color inversion
pub fn invert_colors(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let img_pixel: &Rgb<u8> = img.get_pixel(x, y);
        for n in 0..3 {
            pixel[n] = 255 - img_pixel[n];
        }
    }
    imgbuf
}
