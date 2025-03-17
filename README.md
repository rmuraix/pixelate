# Pixelate

[![License](https://img.shields.io/github/license/rmuraix/pixelate)](./LICENSE)
[![Build](https://github.com/rmuraix/pixelate/actions/workflows/build.yml/badge.svg)](https://github.com/rmuraix/pixelate/actions/workflows/build.yml)

## About

Command line applications for image processing

## Usage

Click [here](https://github.com/image-rs/image#supported-image-formats) to see supported image formats.

```shell
Usage: pixelate --input <FILE> --output <FILE> <COMMAND>

Commands:
  grayscale  Convert the image to grayscale
  halftone   Apply halftoning using the dithering method
  gamma      Perform gamma correction
  invert     Apply negative-positive inversion
  help       Print this message or the help of the given subcommand(s)

Options:
  -i, --input <FILE>   Path to the image file to be processed
  -o, --output <FILE>  Output path for the processed image file
  -h, --help           Print help
  -V, --version        Print version
```

## Contributing

Your contribution is always welcome. Please read [Contributing Guide](.github/CONTRIBUTING.md).
