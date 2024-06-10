use image::io::Reader as ImageReader;
mod cli;
use clap::Parser;
use cli::Cli;
use cli::Codec;

fn main() {
    let cli = Cli::parse();
    match &cli.codec {
	Codec::C1 { img_filepath } => {
	    println!("Loading image...");
	    let img = ImageReader::open("./assets/lena_colour.bmp").unwrap();
	    println!("Image loaded!");
	}
	_ => {
	    todo!("C2, CIMap, CIMap2 codecs are not implemented yet");
	}
	    
    }

}
