use std::{path::PathBuf, time};

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
    /// Convert to grayscale image
    Grayscale {
        /// Red weight
        #[arg(short, long, default_value = "0.2126")]
        red: f64,
        /// Green weight
        #[arg(short, long, default_value = "0.7152")]
        green: f64,
        /// Blue weight
        #[arg(short, long, default_value = "0.0722")]
        blue: f64,
    },
    /// halftoning using the dither method
    Halftone,
    /// gamma correction
    Gamma {
        /// gamma value
        #[arg(short, long)]
        gamma: f64,
    },
}

fn main() {
    let start = time::Instant::now();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Grayscale { red, green, blue } => {
            if (red + green + blue) > 1.0 {
                panic!("The sum of the RGB values must be less than 1.0")
            }
            let img = filters::grayscale(cli.target, *red, *green, *blue);
            img.save(cli.out).unwrap()
        }
        Commands::Halftone => {
            let img = filters::halftoning(cli.target);
            img.save(cli.out).unwrap()
        }
        Commands::Gamma { gamma } => {
            let img = filters::gamma(cli.target, *gamma);
            img.save(cli.out).unwrap()
        }
    }
    println!("Compute time:{:?}", start.elapsed());
}
