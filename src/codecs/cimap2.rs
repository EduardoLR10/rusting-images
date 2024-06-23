use crate::codecs::cimap::quantize_image;
use crate::codecs::dithering::{dithering, DitheringMask};
use image::{ImageBuffer, Pixel, Luma, Rgb, GrayImage};

pub fn make_image_with_dithering(
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
    n_colors: usize,
    optional_mask: Option<DitheringMask>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let quantized_img_rgb = quantize_image(img, n_colors);
    let mut r_img = GrayImage::new(width, height);
    let mut g_img = GrayImage::new(width, height);
    let mut b_img = GrayImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
	    let p_channels = quantized_img_rgb.get_pixel(x, y).channels();
	    r_img.put_pixel(x, y, Luma::from([p_channels[0]; 1]));
	    g_img.put_pixel(x, y, Luma::from([p_channels[1]; 1]));
	    b_img.put_pixel(x, y, Luma::from([p_channels[2]; 1]));
        }
    }
    dithering(&mut r_img, optional_mask);
    dithering(&mut g_img, optional_mask);
    dithering(&mut b_img, optional_mask);
    ImageBuffer::from_fn(width, height, |x, y| {
	let r_value = r_img.get_pixel(x, y);
	let g_value = g_img.get_pixel(x, y);
	let b_value = b_img.get_pixel(x, y);
        Rgb([r_value.0[0], g_value.0[0], b_value.0[0]])
    })
}
