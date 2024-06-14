use crate::codecs::dithering::{dithering, Ditherable, DitheringMask};
use image::{DynamicImage, ImageBuffer, Luma, Rgb, Rgba};

fn binarize(pixel_value: u8) -> u8 {
    (pixel_value as f64 / 255.0).round() as u8
}

fn to_rgb(pixel: u8) -> Rgb<u8> {
    if pixel == 1 {
        Rgb([255, 255, 255])
    } else {
        Rgb([0, 0, 0])
    }
}

impl Ditherable<Luma<u8>, Rgb<u8>> for Luma<u8> {
    fn clip(&self) -> Luma<u8> {
        Luma::from([if self.0[0] > 127 { 255 } else { 0 }; 1])
    }
    fn unclip(&self) -> Rgb<u8> {
        let value = self.0[0];
        to_rgb(binarize(value))
    }
    fn quant_error(&self, other: Luma<u8>) -> f64 {
        self.0[0] as f64 - other.0[0] as f64
    }
    fn propagate_error(&self, error: f64, propagation: f64) -> Luma<u8> {
        Luma::from([(self.0[0] as f64 + error * propagation) as u8; 1])
    }
}

pub fn make_image_with_dithering(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    optional_mask: Option<DitheringMask>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let gray_img = DynamicImage::ImageRgba8(img).into_luma8();
    dithering(gray_img, optional_mask)
}
