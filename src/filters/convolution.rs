//! Reusable 2D convolution utilities for grayscale images.
use image::{ImageBuffer, Luma};

/// Convolve a grayscale image with an odd-sized square kernel, returning f32 values.
///
/// - `K` must be odd (3, 5, ...). Zero padding is used at the borders.
pub fn convolve_gray_f32<const K: usize>(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
    kernel: &[[f32; K]; K],
) -> Vec<f32> {
    assert!(K % 2 == 1, "Kernel size must be odd");
    let (w, h) = img.dimensions();
    let mut out = vec![0.0f32; (w * h) as usize];
    let r = (K / 2) as i32;

    for y in 0..h as i32 {
        for x in 0..w as i32 {
            let mut acc = 0.0f32;
            for ky in 0..K as i32 {
                for kx in 0..K as i32 {
                    let ix = x + kx - r;
                    let iy = y + ky - r;
                    // Retrieves the pixel value at the specified coordinates (`ix`, `iy`) from the image.
                    // If the coordinates are within the image bounds, returns the first channel value as `f32`.
                    // If the coordinates are out of bounds, returns `0.0`.
                    let v = if ix >= 0 && ix < w as i32 && iy >= 0 && iy < h as i32 {
                        img.get_pixel(ix as u32, iy as u32)[0] as f32
                    } else {
                        0.0
                    };
                    acc += v * kernel[ky as usize][kx as usize];
                }
            }
            out[(y as u32 * w + x as u32) as usize] = acc;
        }
    }
    out
}

/// Compute gradient magnitude and map to `Luma<u8>` with adjustable intensity.
///
/// The result is first normalized by the maximum magnitude, then multiplied by
/// `intensity` before clamping to 0..=255. Values >1 brighten edges; <1 dims them.
pub fn magnitude_to_luma_u8_scaled(
    gx: &[f32],
    gy: &[f32],
    width: u32,
    height: u32,
    intensity: f32,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    assert_eq!(gx.len(), gy.len());
    let mut mag = vec![0.0f32; gx.len()];
    let mut max_mag = 0.0f32;
    for i in 0..gx.len() {
        let m = (gx[i] * gx[i] + gy[i] * gy[i]).sqrt();
        mag[i] = m;
        if m > max_mag {
            max_mag = m;
        }
    }
    let base_scale = if max_mag > 0.0 { 255.0 / max_mag } else { 0.0 };
    let scale = base_scale * intensity.max(0.0);
    let mut out: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let m = mag[(y * width + x) as usize];
            let n = (m * scale).round().clamp(0.0, 255.0) as u8;
            out.put_pixel(x, y, Luma([n]));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Luma};

    #[test]
    fn test_convolve_identity() {
        let img: ImageBuffer<Luma<u8>, Vec<u8>> =
            ImageBuffer::from_fn(3, 3, |x, y| Luma([x as u8 + y as u8]));
        const K: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]];
        let out = convolve_gray_f32::<3>(&img, &K);
        for y in 0..3 {
            for x in 0..3 {
                assert_eq!(out[(y * 3 + x) as usize].round() as u8, x as u8 + y as u8);
            }
        }
    }
}
