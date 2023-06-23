use image::{self, ImageBuffer, Luma, Rgb};

pub fn grayscale(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    red: f64,
    green: f64,
    blue: f64,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width = img.width();
    let height = img.height();

    // create data for grayscale
    let mut gray_img = image::GrayImage::new(width, height);
    let _pix = img.get_pixel(0, 0);

    for y in 0..height {
        for x in 0..width {
            // Get pixel data
            let pix = img.get_pixel(x, y);
            let val = [((pix[0] as f32 * red as f32) as u32
                + (pix[1] as f32 * green as f32) as u32
                + (pix[2] as f32 * blue as f32) as u32) as u8; 1];
            // Writing pixel data to grayscale data
            gray_img.put_pixel(x, y, image::Luma(val));
        }
    }

    return gray_img;
}
