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
    Negaposi,
}

fn main() {
    let start = time::Instant::now();
    let cli = Cli::parse();
    let dynamic_img: image::DynamicImage = image::open(cli.target).unwrap();
    match &cli.command {
        Commands::Grayscale { red, green, blue } => {
            if (red + green + blue) > 1.0 {
                panic!("The sum of the RGB values must be less than 1.0")
            }
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::grayscale(rgb_img, *red, *green, *blue);
            img.save(cli.out).unwrap()
        }
        Commands::Halftone => {
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::halftoning(rgb_img);
            img.save(cli.out).unwrap()
        }
        Commands::Gamma { gamma } => {
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::gamma(rgb_img, *gamma);
            img.save(cli.out).unwrap()
        }
        Commands::Negaposi => {
            let rgb_img = dynamic_img.to_rgb8();
            let img = filters::negaposi(rgb_img);
            img.save(cli.out).unwrap()
        }
    }
    println!("Compute time:{:?}", start.elapsed());
}
