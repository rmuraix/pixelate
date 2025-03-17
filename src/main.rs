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
    input: PathBuf,

    /// Output path for the processed image file
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert the image to grayscale
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
    /// Apply halftoning using the dithering method
    Halftone,
    /// Perform gamma correction
    Gamma {
        /// gamma value
        #[arg(short, long)]
        gamma: f64,
    },
    /// Apply negative-positive inversion
    Invert,
}

fn main() {
    let start = time::Instant::now();
    let cli = Cli::parse();
    let dynamic_img: image::DynamicImage = image::open(cli.input).unwrap();
    match &cli.command {
        Commands::Grayscale { red, green, blue } => {
            if (red + green + blue) > 1.0 {
                panic!("The sum of the RGB values must be less than 1.0")
            }
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::grayscale(rgb_img, *red, *green, *blue);
            img.save(cli.output).unwrap()
        }
        Commands::Halftone => {
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::halftoning(rgb_img);
            img.save(cli.output).unwrap()
        }
        Commands::Gamma { gamma } => {
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::gamma(rgb_img, *gamma);
            img.save(cli.output).unwrap()
        }
        Commands::Invert => {
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::negaposi(rgb_img);
            img.save(cli.output).unwrap()
        }
    }
    println!("Compute time:{:?}", start.elapsed());
}
