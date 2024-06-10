mod cli;
use clap::Parser;
use cli::Cli;
use cli::Codec;
mod codecs;
use codecs::c1;

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
        _ => {
            todo!("C2, CIMap, CIMap2 codecs are not implemented yet");
        }
    }
}
