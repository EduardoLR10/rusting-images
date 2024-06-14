use image::{EncodableLayout, ImageBuffer, Pixel, PixelWithColorType};
use std::ops::{Deref, DerefMut};

pub type DitheringMask = [[f64; 3]; 3];

pub const FLOYD_STEINBERG: DitheringMask = [
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 7.0 / 16.0],
    [3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0],
];

pub trait Ditherable<T: Pixel + PixelWithColorType, U: Pixel + PixelWithColorType> {
    fn clip(&self) -> T;
    fn unclip(&self) -> U;
    fn quant_error(&self, other: T) -> f64;
    fn propagate_error(&self, error: f64, propagation: f64) -> T;
}

pub fn dithering<T, U, V>(
    mut img: ImageBuffer<T, U>,
    optional_mask: Option<DitheringMask>,
) -> ImageBuffer<V, Vec<<V as Pixel>::Subpixel>>
where
    V: Pixel + PixelWithColorType,
    T: Pixel + PixelWithColorType + Ditherable<T, V>,
    [T::Subpixel]: EncodableLayout,
    U: Deref<Target = [T::Subpixel]>,
    U: DerefMut,
{
    let mask = optional_mask.unwrap_or(FLOYD_STEINBERG);
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let old_pixel = img.get_pixel(x, y);
            let new_pixel = old_pixel.clip();
            let quant_error = Ditherable::quant_error(old_pixel, new_pixel);
            img.put_pixel(x, y, new_pixel);
            for j in -1..2 as i32 {
                for i in -1..2 as i32 {
                    let x_index = x as i32 + i;
                    let y_index = y as i32 + j;
                    if x_index < width as i32
                        && x_index >= 0
                        && y_index < height as i32
                        && y_index >= 0
                    {
                        let dithered_mask_value = mask[(i + 1) as usize][(j + 1) as usize];
                        let current_pixel = img.get_pixel(x_index as u32, y_index as u32);
                        let pixel_with_error = Ditherable::propagate_error(
                            current_pixel,
                            quant_error,
                            dithered_mask_value,
                        );
                        img.put_pixel(x_index as u32, y_index as u32, pixel_with_error);
                    }
                }
            }
        }
    }
    ImageBuffer::from_fn(width, height, |x, y| {
        let value = img.get_pixel(x, y);
        Ditherable::unclip(value)
    })
}
