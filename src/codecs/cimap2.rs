use crate::codecs::cimap::quantize_image;
use crate::codecs::dithering::{dithering, Ditherable, DitheringMask};
use image::{ImageBuffer, Pixel, Rgb, Rgba};

fn clip(pixel_value: u8) -> u8 {
    if pixel_value > 127 {
        255
    } else {
        0
    }
}

impl Ditherable<Rgb<u8>, Rgb<u8>> for Rgb<u8> {
    fn clip(self: &Rgb<u8>) -> Rgb<u8> {
        let colors = Rgb::channels(&self);
        let mut final_colors: [u8; 3] = [0, 0, 0];
        final_colors[0] = clip(colors[0]);
        final_colors[1] = clip(colors[1]);
        final_colors[2] = clip(colors[2]);

        Rgb::from_slice(&final_colors).to_owned()
    }
    fn unclip(&self) -> Rgb<u8> {
        *self
    }
    fn quant_error(&self, other: Rgb<u8>) -> f64 {
        let colors_1 = Rgb::channels(self);
        let colors_2 = Rgb::channels(&other);
        let diff_r = (colors_1[0] as i16 - colors_2[0] as i16) as f64;
        let diff_g = (colors_1[1] as i16 - colors_2[1] as i16) as f64;
        let diff_b = (colors_1[2] as i16 - colors_2[2] as i16) as f64;
        (diff_r + diff_g + diff_b) as f64 / 3.0
    }
    fn propagate_error(&self, error: f64, propagation: f64) -> Rgb<u8> {
        let colors = Rgb::channels(&self);
        let mut final_colors: [u8; 3] = [0, 0, 0];
        final_colors[0] = (colors[0] as f64 + error * propagation) as u8;
        final_colors[1] = (colors[1] as f64 + error * propagation) as u8;
        final_colors[2] = (colors[2] as f64 + error * propagation) as u8;

        Rgb::from_slice(&final_colors).to_owned()
    }
}

pub fn make_image_with_dithering(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    n_colors: usize,
    optional_mask: Option<DitheringMask>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let quantized_img_rgb = quantize_image(img, n_colors);
    dithering(quantized_img_rgb, optional_mask)
}
