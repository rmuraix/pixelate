//! Pixelate library crate.
//!
//! This crate exposes image filters and small composition utilities used by the
//! `pixelate` CLI. You can depend on it directly to reuse filters in your own
//! applications
//!
//! Example
//! ```no_run
//! use image::GenericImageView;
//! use pixelate::filters::{Filter, HalftoneFilter};
//!
//! let img = image::open("input.jpg").unwrap().to_rgb8();
//! let out = HalftoneFilter.apply(&img);
//! out.save("out.jpg").unwrap();
//! ```

/// Shared color-related constants (e.g., sRGB luminance weights).
pub mod color;
/// Filter definitions and built-in filters.
pub mod filters;
/// Simple, typed filter composition utilities.
pub mod pipeline;
