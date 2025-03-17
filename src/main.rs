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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start: time::Instant = time::Instant::now();
    let cli: Cli = Cli::parse();

    // Read the image file
    let dynamic_img: image::DynamicImage = image::open(&cli.input)?;
    let rgb_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = dynamic_img.to_rgb8();

    match &cli.command {
        Commands::Grayscale { red, green, blue } => {
            // Check that the sum of RGB weights does not exceed 1.0
            if red + green + blue > 1.0 {
                return Err("The sum of the RGB weights must be less than or equal to 1.0".into());
            }
            let img: image::ImageBuffer<image::Luma<u8>, Vec<u8>> =
                filters::grayscale(rgb_img, *red, *green, *blue);
            img.save(&cli.output)?;
        }
        Commands::Halftone => {
            let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = filters::halftoning(rgb_img);
            img.save(&cli.output)?;
        }
        Commands::Gamma { gamma } => {
            if *gamma <= 0.0 {
                return Err("Gamma value must be greater than 0.0".into());
            }
            let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
                filters::gamma_correct(rgb_img, *gamma);
            img.save(&cli.output)?;
        }
        Commands::Invert => {
            let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = filters::invert_colors(rgb_img);
            img.save(&cli.output)?;
        }
    }
    println!("Compute time: {:?}", start.elapsed());
    Ok(())
}
