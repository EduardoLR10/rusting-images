mod cli;
use clap::Parser;
use cli::Cli;
use cli::Codec;
mod codecs;
use codecs::c1;
use codecs::c2;
use codecs::cimap;
use codecs::cimap2;
use image::{EncodableLayout, ImageBuffer, Pixel, PixelWithColorType, Rgba};
use std::ops::Deref;

fn load_image(img_filepath: &String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    println!("Loading image...");
    let img = image::open(img_filepath).unwrap().into_rgba8();
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

fn main() {
    let cli = Cli::parse();
    match &cli.codec {
        Codec::C1 { img_filepath } => {
            let img = load_image(img_filepath);
            println!("Applying C1 codec...");
            let final_img = c1::make_image_discrete(img);
            save_image(final_img, img_filepath, "_c1.bmp".to_string())
        }
        Codec::C2 { img_filepath } => {
            let img = load_image(img_filepath);
            println!("Applying C2 codec...");
            let final_img = c2::make_image_with_dithering(img, None);
            save_image(final_img, img_filepath, "_c2.bmp".to_string())
        }
        Codec::CIMap {
            img_filepath,
            n_colors,
        } => {
            let img = load_image(img_filepath);
            println!("Applying CIMap codec...");
            let final_img = cimap::quantize_image(img, n_colors.to_owned());
            save_image(final_img, img_filepath, "_cimap.bmp".to_string())
        }
        Codec::CIMap2 {
            img_filepath,
            n_colors,
        } => {
            let img = load_image(img_filepath);
            println!("Applying CIMap2 codec...");
            let final_img = cimap2::make_image_with_dithering(img, n_colors.to_owned(), None);
            save_image(final_img, img_filepath, "_cimap2.bmp".to_string())
        }
    }
}
