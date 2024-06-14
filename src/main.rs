mod cli;
use clap::Parser;
use cli::Cli;
use cli::Codec;
mod codecs;
use codecs::c1;
use codecs::c2;

fn main() {
    let cli = Cli::parse();
    match &cli.codec {
        Codec::C1 { img_filepath } => {
            println!("Loading image...");
            let img = image::open(img_filepath).unwrap().into_rgba8();
            println!("Image loaded!");
            println!("Applying C1 codec...");
            let final_img = c1::make_image_discrete(img);
            println!("Saving image...");
            let mut final_img_path: String = img_filepath[0..img_filepath.len() - 4].to_string();
            final_img_path.push_str("_c1.bmp");
            final_img.save(final_img_path).unwrap()
        }
        Codec::C2 { img_filepath } => {
            println!("Loading image...");
            let img = image::open(img_filepath).unwrap().into_rgba8();
            println!("Image loaded!");
            println!("Applying C2 codec...");
            let final_img = c2::make_image_with_dithering(img, None);
            println!("Saving image...");
            let mut final_img_path: String = img_filepath[0..img_filepath.len() - 4].to_string();
            final_img_path.push_str("_c2.bmp");
            final_img.save(final_img_path).unwrap()
        }
        _ => {
            todo!("CIMap, CIMap2 codecs are not implemented yet");
        }
    }
}
