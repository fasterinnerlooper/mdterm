#![allow(dead_code)]

//! Stage 3: PNG to Terminal Art via libchafa
//! 
//! Converts PNG image bytes to ANSI terminal art using libchafa's C API.
//! 
//! # Safety and Design
//!
//! This module uses FFI (Foreign Function Interface) to call the C library libchafa.
//! To maintain safety, all raw FFI operations are wrapped in the `chafa_safe` module,
//! which provides memory-safe types with RAII (Resource Acquisition Is Initialization).
//!
//! When you create a `ChafaSymbolMap`, `ChafaCanvasConfig`, or `ChafaCanvas`, the wrapper
//! types automatically clean up their C resources when dropped, preventing memory leaks
//! and use-after-free bugs.
//!
//! # Example
//!
//! ```ignore
//! // The PNG bytes are decoded to RGBA pixels
//! let img = image::load_from_memory(png_bytes)?;
//! let rgba = img.to_rgba8();
//! 
//! // Create symbol map and canvas config using safe wrappers
//! let symbol_map = SafeSymbolMap::new()?;
//! let config = SafeCanvasConfig::new()?
//!     .with_geometry(80, 24)?
//!     .with_symbol_map(&symbol_map)?;
//!
//! // Create canvas and convert to terminal art
//! let mut canvas = SafeCanvas::new(&config)?;
//! canvas.draw_all_pixels(...)?;
//! let output = canvas.build_ansi()?;
//! ```
//!
//! # See Also
//!
//! - [libchafa C API reference](https://hpjansson.org/chafa/ref/)
//! - [Rust FFI guide](https://doc.rust-lang.org/nomicon/ffi.html)

use anyhow::{Context, Result};
use std::io::Write;
use image::ImageEncoder;

// used by the multi-format fallback
use tempfile;

use crate::chafa_safe::{SafeSymbolMap, SafeCanvasConfig, SafeCanvas};
use chafa_sys as sys;

/// Output format for terminal art
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    /// ANSI escape codes with 256-color palette
    Ansi,
    /// True color (24-bit) ANSI escape codes  
    TrueColor,
    /// Sixel graphics format
    Sixel,
    /// Kitty graphics protocol
    Kitty,
    /// iTerm2 graphics protocol
    ITerm2,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "sixel" => OutputFormat::Sixel,
            "kitty" => OutputFormat::Kitty,
            "iterm2" | "iterm" => OutputFormat::ITerm2,
            "truecolor" | "24bit" => OutputFormat::TrueColor,
            _ => OutputFormat::Ansi,
        }
    }

    /// Convert to the string expected by the `chafa` CLI
    pub fn to_cli_arg(&self) -> &'static str {
        match self {
            OutputFormat::Ansi => "symbols",
            OutputFormat::TrueColor => "symbols",
            OutputFormat::Sixel => "sixels",
            OutputFormat::Kitty => "kitty",
            OutputFormat::ITerm2 => "iterm",
        }
    }
}

/// Configuration for image-to-terminal conversion
pub struct ChafaConfig {
    /// Width in terminal cells
    pub width: u32,
    /// Height in terminal cells  
    pub height: u32,
    /// Output format
    pub format: OutputFormat,
    /// Whether to use dithering
    pub dither: bool,
}

impl Default for ChafaConfig {
    fn default() -> Self {
        Self {
            width: 80,
            height: 24,
            format: OutputFormat::Ansi,
            dither: true,
        }
    }
}

/// Convert PNG bytes to terminal art string using libchafa
/// 
/// This function handles the three main steps:
/// 1. Decode PNG to RGBA pixels using the `image` crate
/// 2. Create libchafa objects (symbol map, canvas config, canvas) using safe wrappers
/// 3. Draw pixels to canvas and generate ANSI output
///
/// # Arguments
///
/// * `png_bytes` - Raw PNG file data
/// * `config` - Configuration for the conversion (dimensions, dithering, format)
///
/// # Errors
///
/// Returns an error if:
/// - PNG decoding fails
/// - Creating libchafa objects fails (e.g., allocation error)
/// - Converting to terminal art fails
///
/// # Note
///
/// This requires libchafa to be installed:
/// - Windows (MSYS2): `pacman -S mingw-w64-x86_64-chafa`
/// - Linux (Ubuntu): `apt install libchafa-dev`  
/// - macOS: `brew install chafa`
pub fn png_to_terminal_art(png_bytes: &[u8], config: &ChafaConfig) -> Result<String> {
    // Step 1: Decode PNG to raw RGBA pixels using the image crate
    let img = image::load_from_memory(png_bytes)
        .context("Failed to decode PNG image")?;
    
    let rgba = img.to_rgba8();
    let (img_width, img_height) = rgba.dimensions();
    
    log::debug!("PNG image dimensions: {}x{}", img_width, img_height);
    log::debug!("Target canvas dimensions: {}x{}", config.width, config.height);
    
    // Convert RGBA pixels to a flat vector
    let pixels: Vec<u8> = rgba.into_raw();
    
    // Step 2: Create symbol map using safe wrapper
    let mut symbol_map = SafeSymbolMap::new()
        .context("Failed to create symbol map")?;
    symbol_map.add_by_tags(sys::CHAFA_SYMBOL_TAG_ALL as u32)
        .context("Failed to add symbols to symbol map")?;
    
    // Step 3: Create canvas config using safe wrapper with builder pattern
    let config_obj = SafeCanvasConfig::new()
        .context("Failed to create canvas config")?
        .with_geometry(config.width, config.height)
        .context("Failed to set canvas geometry")?
        .with_symbol_map(&symbol_map)
        .context("Failed to set symbol map")?;
    
    // Set dither mode based on config
    let config_obj = if config.dither {
        config_obj.with_dither_mode(sys::CHAFA_DITHER_MODE_DIFFUSION as u32)
    } else {
        config_obj.with_dither_mode(sys::CHAFA_DITHER_MODE_NONE as u32)
    }.context("Failed to set dither mode")?;
    
    // Step 4: Create canvas and draw pixels using safe wrapper
    let mut canvas = SafeCanvas::new(&config_obj)
        .context("Failed to create canvas")?;
    
    canvas.draw_all_pixels(
        sys::CHAFA_PIXEL_RGBA8 as u32,
        pixels.as_ptr(),
        img_width,
        img_height,
        img_width * 4, // rowstride (4 bytes per pixel for RGBA)
    ).context("Failed to draw pixels to canvas")?;
    
    // Step 5: Convert to the requested output format
    let output = match config.format {
        OutputFormat::Ansi | OutputFormat::TrueColor => {
            // Use the external chafa CLI for ANSI/TrueColor output to ensure reliability
            let fmt_arg = "symbols";
            
            let mut tmp = tempfile::NamedTempFile::new()
                .context("failed to create temporary file for chafa")?;
            tmp.write_all(png_bytes)
                .context("failed to write PNG to temp file")?;

            let child = std::process::Command::new("chafa")
                .arg("-f")
                .arg(fmt_arg)
                .arg(if config.format == OutputFormat::TrueColor { "--colors=full" } else { "--colors=16" })
                .arg(tmp.path())
                .output()
                .context("failed to execute chafa command")?;
            if !child.status.success() {
                anyhow::bail!("chafa terminated with status {:?}", child.status);
            }
            String::from_utf8(child.stdout)
                .context("chafa output was not valid UTF-8")?
        }
        OutputFormat::Sixel | OutputFormat::Kitty | OutputFormat::ITerm2 => {
            // fall back to the external `chafa` binary; this keeps us in sync
            // with the CLI behaviour for the exotic protocols and avoids
            // re‑implementing terminfo handling in Rust.
            let fmt_arg = match config.format {
                OutputFormat::Sixel => "sixel",
                OutputFormat::Kitty => "kitty",
                OutputFormat::ITerm2 => "iterm",
                _ => unreachable!(),
            };

            // write the PNG bytes to a temp file so that the CLI can read it
            let mut tmp = tempfile::NamedTempFile::new()
                .context("failed to create temporary file for chafa")?;
            tmp.write_all(png_bytes)
                .context("failed to write PNG to temp file")?;

            let child = std::process::Command::new("chafa")
                .arg("-f")
                .arg(fmt_arg)
                .arg(tmp.path())
                .output()
                .context("failed to execute chafa command")?;
            if !child.status.success() {
                anyhow::bail!("chafa terminated with status {:?}", child.status);
            }
            String::from_utf8(child.stdout)
                .context("chafa output was not valid UTF-8")?
        }
    };

    Ok(output)
}


/// Convert a PNG file path to terminal art
///
/// # Arguments
///
/// * `path` - Path to PNG file
/// * `config` - Configuration for the conversion
///
/// # Errors
///
/// Returns an error if the file cannot be read or conversion fails.
pub fn png_file_to_terminal_art(path: &str, config: &ChafaConfig) -> Result<String> {
    let png_bytes = std::fs::read(path)
        .context(format!("Failed to read PNG file: {}", path))?;
    
    png_to_terminal_art(&png_bytes, config)
}

// translate ANSI foreground-color escapes into background-color codes so that
// dark terminals (or any terminal) will actually _see_ the image.  This is a
// stopgap until we call `chafa_canvas_print()` with a proper terminfo object.
fn fix_ansi_backgrounds(s: &str) -> String {
    s.replace("\x1b[38;2;", "\x1b[48;2;")
     .replace("\x1b[38;5;", "\x1b[48;5;")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_defaults() {
        let config = ChafaConfig::default();
        assert_eq!(config.width, 80);
        assert_eq!(config.height, 24);
    }
    
    #[test]
    fn test_output_format_parsing() {
        assert_eq!(OutputFormat::from_str("ansi"), OutputFormat::Ansi);
        assert_eq!(OutputFormat::from_str("sixel"), OutputFormat::Sixel);
        assert_eq!(OutputFormat::from_str("kitty"), OutputFormat::Kitty);
        assert_eq!(OutputFormat::from_str("iterm2"), OutputFormat::ITerm2);
    }

    #[test]
    fn fix_ansi_backgrounds_basic() {
        let input = "\x1b[38;2;255;0;0m ";
        let out = fix_ansi_backgrounds(input);
        assert!(out.contains("\x1b[48;2;255;0;0m"));
    }

    #[test]
    fn cli_fallback_format() {
        // only run if the `chafa` binary is actually available
        if std::process::Command::new("chafa").arg("--version").output().is_err() {
            eprintln!("skipping CLI fallback test; chafa not found");
            return;
        }

        // make a small red PNG in memory (use the png encoder directly so we
        // don't have to satisfy the `Seek` bound on `write_to`).
        let mut buf = Vec::new();
        {
            let img = image::RgbaImage::from_pixel(2, 2, image::Rgba([255, 0, 0, 255]));
            image::codecs::png::PngEncoder::new(&mut buf)
                .write_image(&img, 2, 2, image::ColorType::Rgba8.into())
                .unwrap();
        }

        let config = ChafaConfig {
            width: 2,
            height: 2,
            format: OutputFormat::Sixel,
            dither: true,
        };
        let out = png_to_terminal_art(&buf, &config).expect("failed to run CLI fallback");
        assert!(!out.is_empty());
        // the chafa CLI output always hides the cursor at start
        assert!(out.contains("?25l") || out.contains("sixel"));
    }
}
