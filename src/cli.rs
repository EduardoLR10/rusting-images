use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Use C1 codec
    C1 { img_filepath: String },
    /// Use C2 codec
    C2 { img_filepath: String },
    /// Use CIMap codec
    CIMap {
        img_filepath: String,
        n_colors: usize,
    },
    /// Use CIMap codec
    CIMap2 {
        img_filepath: String,
        n_colors: usize,
    },
    Psnr {
	reference_img_filepath: String,
	test_img_filepath: String
    }
}
