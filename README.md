# Pixelate

[![License](https://img.shields.io/github/license/rmuraix/pixelate)](./LICENSE)
[![Build](https://github.com/rmuraix/pixelate/actions/workflows/build.yml/badge.svg)](https://github.com/rmuraix/pixelate/actions/workflows/build.yml)

## About

Command line applications for image processing

## Usage

Click [here](https://github.com/image-rs/image#supported-image-formats) to see supported image formats.

```shell
Usage: pixelate --target <FILE> --out <FILE> <COMMAND>

Commands:
  grayscale  Convert to grayscale image
  halftone   halftoning using the dither method
  gamma      gamma correction
  negaposi   Negative-positive reversal
  help       Print this message or the help of the given subcommand(s)

Options:
  -t, --target <FILE>  Path to the image file to be processed
  -o, --out <FILE>     Output path of the processed image file
  -h, --help           Print help
  -V, --version        Print version
```

## Contributing

Your contribution is always welcome. Please read [Contributing Guide](.github/CONTRIBUTING.md).
