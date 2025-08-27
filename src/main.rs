use clap::{Parser, Subcommand, ValueEnum};
use std::{path::PathBuf, time};

use pixelate::filters::{
    Filter, GammaFilter, GrayscaleFilter, HalftoneFilter, InvertFilter, SobelFilter,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the image file to be processed
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
    /// Output path for the processed image file
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum EdgeMethod {
    /// Sobel operator (gradient magnitude)
    Sobel,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert the image to grayscale
    Grayscale {
        /// Red channel weight
        #[arg(short, long, default_value = "0.2126")]
        red: f64,
        /// Green channel weight
        #[arg(short, long, default_value = "0.7152")]
        green: f64,
        /// Blue channel weight
        #[arg(short, long, default_value = "0.0722")]
        blue: f64,
    },
    /// Apply halftoning using the dithering method
    Halftone,
    /// Perform gamma correction
    Gamma {
        /// Gamma value
        #[arg(short, long)]
        gamma: f64,
    },
    /// Apply negative-positive inversion
    Invert,
    /// Detect edges (default: Sobel operator)
    Edge {
        /// Edge detection method
        #[arg(long, value_enum, default_value_t = EdgeMethod::Sobel)]
        method: EdgeMethod,
        /// Intensity multiplier applied after normalization (>= 0.0)
        #[arg(long, default_value = "1.0")]
        intensity: f64,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = time::Instant::now();
    let cli = Cli::parse();

    // Read the image file
    let dynamic_img: image::DynamicImage = image::open(&cli.input)?;
    let rgb_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = dynamic_img.to_rgb8();

    match &cli.command {
        Commands::Grayscale { red, green, blue } => {
            if red + green + blue > 1.0 {
                return Err("The sum of the RGB weights must be less than or equal to 1.0".into());
            }
            let filter: GrayscaleFilter = GrayscaleFilter::new(*red, *green, *blue);
            let img = filter.apply(&rgb_img);
            img.save(&cli.output)?;
        }
        Commands::Halftone => {
            let filter: HalftoneFilter = HalftoneFilter;
            let img = filter.apply(&rgb_img);
            img.save(&cli.output)?;
        }
        Commands::Gamma { gamma } => {
            if *gamma <= 0.0 {
                return Err("Gamma value must be greater than 0.0".into());
            }
            let filter: GammaFilter = GammaFilter::new(*gamma);
            let img = filter.apply(&rgb_img);
            img.save(&cli.output)?;
        }
        Commands::Invert => {
            let filter: InvertFilter = InvertFilter;
            let img = filter.apply(&rgb_img);
            img.save(&cli.output)?;
        }
        Commands::Edge { method, intensity } => match method {
            EdgeMethod::Sobel => {
                if *intensity < 0.0 {
                    return Err("Intensity must be >= 0.0".into());
                }
                let filter: SobelFilter = SobelFilter::new(*intensity);
                let img = filter.apply(&rgb_img);
                img.save(&cli.output)?;
            }
        },
    }
    println!("Compute time: {:?}", start.elapsed());
    Ok(())
}
