mod cli;
use clap::Parser;
use cli::Cli;
use cli::Codec;
mod codecs;
use codecs::c1;
use codecs::c2;
use codecs::cimap;
use codecs::cimap2;
mod util;
use image::{EncodableLayout, ImageBuffer, Pixel, PixelWithColorType, Rgb};
use std::ops::{Deref, DerefMut};

fn load_image(img_filepath: &String) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    println!("Loading image...");
    let img = image::open(img_filepath).unwrap().into_rgb8();
    println!("Image loaded!");
    img
}

fn save_image<T, U>(img: ImageBuffer<T, U>, img_filepath: &String, suffix_filepath: String)
where
    T: Pixel + PixelWithColorType,
    [T::Subpixel]: EncodableLayout,
    U: Deref<Target = [T::Subpixel]>,
{
    println!("Saving image...");
    let mut final_img_path: String = img_filepath[0..img_filepath.len() - 4].to_string();
    final_img_path.push_str(&suffix_filepath);
    img.save(final_img_path).unwrap()
}

fn show_psnr<T, U>(ref_img: &ImageBuffer<T, U>, tes_img: &ImageBuffer<T, U>)
where
    T: Pixel + PixelWithColorType,
    [T::Subpixel]: EncodableLayout,
    U: Deref<Target = [T::Subpixel]> + DerefMut, f64: From<<T as Pixel>::Subpixel>
{
    println!("Calculated Average PSNR: {:.2}", util::psnr(ref_img, tes_img))
}


fn main() {
    let cli = Cli::parse();
    match &cli.codec {
        Codec::C1 { img_filepath } => {
            let img = load_image(img_filepath);
            println!("Applying C1 codec...");
            let final_img = c1::make_image_discrete(img.clone());
	    show_psnr(&img, &final_img);
            save_image(final_img, img_filepath, "_c1.bmp".to_string())
        }
        Codec::C2 { img_filepath } => {
            let img = load_image(img_filepath);
            println!("Applying C2 codec...");
            let final_img = c2::make_image_with_dithering(img.clone(), None);
	    show_psnr(&img, &final_img);
            save_image(final_img, img_filepath, "_c2.bmp".to_string())
        }
        Codec::CIMap {
            img_filepath,
            n_colors,
        } => {
            let img = load_image(img_filepath);
            println!("Applying CIMap codec...");
            let final_img = cimap::quantize_image(img.clone(), n_colors.to_owned());
	    show_psnr(&img, &final_img);
            save_image(final_img, img_filepath, "_cimap.bmp".to_string())
        }
        Codec::CIMap2 {
            img_filepath,
            n_colors,
        } => {
            let img = load_image(img_filepath);
            println!("Applying CIMap2 codec...");
            let final_img = cimap2::make_image_with_dithering(img.clone(), n_colors.to_owned(), None);
	    show_psnr(&img, &final_img);
            save_image(final_img, img_filepath, "_cimap2.bmp".to_string())
        }
    }
}
