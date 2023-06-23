use image::{ImageBuffer, Rgb, RgbImage};

pub fn main(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();

    let mut imgbuf: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let img_pixel = img.get_pixel(x, y);
        for n in 0..3 {
            pixel[n] = 255 - img_pixel[n];
        }
    }
    return imgbuf;
}
