use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod filters;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the image file to be processed
    #[arg(short, long, value_name = "FILE")]
    target: PathBuf,

    /// Output path of the processed image file.
    #[arg(short, long, value_name = "FILE")]
    out: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Grayscale,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Grayscale => {
            let img = filters::grayscale(cli.target);
            img.save(cli.out).unwrap()
        }
    }
}
