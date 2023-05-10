use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
    Test {
        /// test values
        #[arg(short, long)]
        list: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Test { list } => {
            println!("{}", list)
        }
    }
}
