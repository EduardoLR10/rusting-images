use image::{DynamicImage, ImageBuffer, Luma, Rgb, Rgba};

type DitheringMask = [[f64; 3]; 3];

const FLOYD_STEINBERG: DitheringMask = [
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 7.0 / 16.0],
    [3.0 / 16.0, 5.0 / 16.0, 1.0 / 16.0],
];

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

pub fn make_image_with_dithering(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    optional_mask: Option<DitheringMask>,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mask = optional_mask.unwrap_or(FLOYD_STEINBERG);
    let (width, height) = img.dimensions();
    let mut gray_img = DynamicImage::ImageRgba8(img).into_luma8();
    for y in 0..width {
        for x in 0..height {
            let old_pixel = gray_img.get_pixel(x, y);
            let new_pixel = Luma::from([if old_pixel.0[0] > 127 { 255 } else { 0 }; 1]);
            let quant_error = old_pixel.0[0] as f64 - new_pixel.0[0] as f64;
            gray_img.put_pixel(x, y, new_pixel);
            for i in -1..2 as i32 {
                for j in -1..2 as i32 {
                    let x_index = x as i32 + i;
                    let y_index = y as i32 + j;
                    if x_index < height as i32
                        && x_index >= 0
                        && y_index < width as i32
                        && y_index >= 0
                    {
                        let dithered_mask_value = mask[i as usize][j as usize];
                        let current_pixel = gray_img.get_pixel(x_index as u32, y_index as u32);
                        let pixel_with_error = Luma::from(
                            [(current_pixel.0[0] as f64 + quant_error * dithered_mask_value) as u8;
                                1],
                        );
                        gray_img.put_pixel(x_index as u32, y_index as u32, pixel_with_error);
                    }
                }
            }
        }
    }
    ImageBuffer::from_fn(width, height, |x, y| {
        let value = gray_img.get_pixel(x, y).0[0];
        to_rgb(binarize(value))
    })
}
