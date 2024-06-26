use image::{DynamicImage, ImageBuffer, Luma, Rgb};

pub fn make_image_discrete(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    let gray_img = DynamicImage::ImageRgb8(img).into_luma8();
    ImageBuffer::from_fn(width, height, |x, y| {
        let value = gray_img.get_pixel(x, y);
        let threshold: Luma<u8> = Luma::from([127; 1]);
        if value.0 > threshold.0 {
            Rgb([255, 255, 255])
        } else {
            Rgb([0, 0, 0])
        }
    })
}
