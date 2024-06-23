use image::{EncodableLayout, ImageBuffer, Pixel, PixelWithColorType};
use std::ops::{Deref, DerefMut};

pub fn psnr<T, U>(
    ref_img: &ImageBuffer<T, U>,
    tes_img: &ImageBuffer<T, U>
) -> f64
where
    T: Pixel + PixelWithColorType,
    [T::Subpixel]: EncodableLayout,
    U: Deref<Target = [T::Subpixel]> + DerefMut, f64: From<<T as Pixel>::Subpixel>
{
    fn mse<T, U>(
	ref_img: &ImageBuffer<T, U>,
	tes_img: &ImageBuffer<T, U>
    ) -> (Vec<f64>, usize)
    where
	T: Pixel + PixelWithColorType,
        [T::Subpixel]: EncodableLayout,
        U: Deref<Target = [T::Subpixel]> + DerefMut, f64: From<<T as Pixel>::Subpixel>
    {
	let (width_1, height_1) = ref_img.dimensions();
	let (width_2, height_2) = tes_img.dimensions();
	std::assert!(width_1 == width_2 && height_1 == height_2, "Images have different size for MSE calculation");
	let channels_len = ref_img.get_pixel(0, 0).channels().len();
	let mut mses = vec![0.0; channels_len];
	for c in 0..channels_len {
	    for x in 0..width_1 {
		for y in 0..height_1 {
		    let p_1 = ref_img.get_pixel(x, y).channels();
		    let p_2 = tes_img.get_pixel(x, y).channels();		
		    let difference = f64::from(p_1[c]) - f64::from(p_2[c]);
		    mses[c] += difference * difference;
		}
	    }
	}

	for c in 0..channels_len {
	    mses[c] /= (width_1 * height_1) as f64;	   
	}

	(mses, channels_len)
    }
    let (mses, channels_len) = mse(ref_img, tes_img);
    let mut mse_avg: f64 = 0.0;
    for index in 0..mses.len() {
	mse_avg += mses[index];
    }
    mse_avg /= channels_len as f64;
    10.0 * (255.0 * 255.0 / mse_avg).log10()
}
