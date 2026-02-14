# Pixelate

[![License](https://img.shields.io/github/license/rmuraix/pixelate)](./LICENSE)
[![Build](https://github.com/rmuraix/pixelate/actions/workflows/build.yml/badge.svg)](https://github.com/rmuraix/pixelate/actions/workflows/build.yml)

## About

Command line applications for image processing

## Usage

Click [here](https://github.com/image-rs/image#supported-image-formats) to see supported image formats.

```bash
Usage: pixelate --input <FILE> --output <FILE> <COMMAND>

Commands:
  grayscale  Convert the image to grayscale
  halftone   Apply halftoning using the dithering method
  gamma      Perform gamma correction
  invert     Apply negative-positive inversion
  edge       Detect edges (e.g., Sobel)
  help       Print this message or the help of the given subcommand(s)

Options:
  -i, --input <FILE>   Path to the image file to be processed
  -o, --output <FILE>  Output path for the processed image file
  -h, --help           Print help
  -V, --version        Print version
```

For detailed usage examples of each command, see [docs/commands.md](./docs/commands.md).


## Development

- Build: `cargo build` (release: `cargo build --release`)
- Test: `cargo test`
- Format: `cargo fmt --all` (CI enforces `-- --check`)
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`

## Library Usage

You can use Pixelate as a library. Example (apply halftone):

```rust
use image::GenericImageView;
use pixelate::filters::{Filter, HalftoneFilter};

let img = image::open("input.jpg")?.to_rgb8();
let out = HalftoneFilter.apply(&img);
out.save("out.jpg")?;
```

## Contributing

Your contribution is always welcome. Please read [Contributing Guide](https://github.com/rmuraix/.github/blob/main/.github/CONTRIBUTING.md).
