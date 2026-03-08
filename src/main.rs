//! mdterm - Markdown Terminal Viewer
//!
//! A three-stage pipeline that converts Markdown to terminal art:
//! Stage 1: Markdown → styled HTML (using comrak)
//! Stage 2: HTML → PNG image (using headless_chrome)
//! Stage 3: PNG → ANSI terminal art (using libchafa)
//!
//! Also supports direct PNG viewing with --image flag

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read};

mod chafa;
mod chafa_safe;
mod image;
mod markdown;

use chafa::{ChafaConfig, OutputFormat};
use markdown::Theme;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to run
    #[command(subcommand)]
    command: Option<Commands>,

    /// Terminal width in columns (for image conversion)
    #[arg(short = 'w', long = "width", default_value_t = 80)]
    width: u32,

    /// Terminal height in rows (for image conversion)
    #[arg(short = 'H', long = "height", default_value_t = 24)]
    height: u32,

    /// Output format: ansi, truecolor, sixel, kitty, iterm2
    #[arg(short = 'f', long = "format", default_value = "ansi")]
    format: String,

    /// Disable dithering
    #[arg(long = "no-dither")]
    no_dither: bool,

    /// Verbose output
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Demo: Convert a small 3x3 pixel array to terminal art (like original C example)
    Demo,
    /// Convert markdown to terminal art (default)
    Render {
        /// Input markdown file (optional, use --stdin for pipe input)
        #[arg(value_name = "FILE")]
        file: Option<String>,

        /// Read markdown from stdin
        #[arg(short = 's', long = "stdin")]
        stdin: bool,

        /// Color theme: "light" or "dark"
        #[arg(long = "theme", default_value = "light")]
        theme: String,

        /// Save intermediate PNG to a file (for debugging)
        #[arg(long = "save-image")]
        save_image: Option<String>,
    },
    /// View a PNG image directly in the terminal
    View {
        /// Input PNG file
        #[arg(value_name = "FILE")]
        file: String,
    },
}

fn main() -> Result<()> {
    // Initialize logging
    if std::env::var("RUST_LOG").is_err() {
        if std::env::var("DEBUG").is_ok() {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
                .init();
        } else {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
                .init();
        }
    }

    let args = Args::parse();

    // Handle commands
    match args.command {
        Some(Commands::Demo) => {
            // Run the demo that mimics the original C code
            demo_chafa()?;
        }
        Some(Commands::View { ref file }) => {
            // Direct PNG viewing mode
            if args.verbose {
                eprintln!("Viewing PNG: {}", file);
            }
            view_png(file, &args)?;
        }
        Some(Commands::Render {
            ref file,
            stdin,
            ref theme,
            ref save_image,
        }) => {
            // Markdown rendering mode
            render_markdown(file.clone(), stdin, theme.clone(), save_image.clone(), &args)?;
        }
        None => {
            // Default: treat positional arg as markdown file
            // For backward compatibility, support direct file argument
            render_markdown(None, false, "light".to_string(), None, &args)?;
        }
    }

    Ok(())
}

/// View a PNG file directly in the terminal
fn view_png(path: &str, args: &Args) -> Result<()> {
    let config = ChafaConfig {
        width: args.width,
        height: args.height,
        format: OutputFormat::from_str(&args.format),
        dither: !args.no_dither,
    };

    let terminal_output = chafa::png_file_to_terminal_art(path, &config)?;

    if args.verbose {
        eprintln!(
            "Converted {}x{} image to {} bytes of terminal output",
            args.width,
            args.height,
            terminal_output.len()
        );
    }

    // Print to stdout
    print!("{}", terminal_output);

    Ok(())
}

/// Demo: Convert a 3x3 pixel array to terminal art (mimics original C code)
fn demo_chafa() -> Result<()> {
    use libc::c_uint;
    use crate::chafa_safe::{SafeSymbolMap, SafeCanvasConfig, SafeCanvas};

    const PIX_WIDTH: c_uint = 3;
    const PIX_HEIGHT: c_uint = 3;
    const N_CHANNELS: c_uint = 4;

    // Same pixel data as the original C example
    let pixels: [u8; PIX_WIDTH as usize * PIX_HEIGHT as usize * N_CHANNELS as usize] = [
        0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff,
        0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff,
        0xff, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0xff
    ];

    // Create symbol map using safe wrapper
    let mut symbol_map = SafeSymbolMap::new()
        .context("Failed to create symbol map")?;
    symbol_map.add_by_tags(chafa_sys::CHAFA_SYMBOL_TAG_ALL as u32)
        .context("Failed to add symbols")?;

    // Create canvas config using safe wrapper with builder pattern
    let config = SafeCanvasConfig::new()
        .context("Failed to create canvas config")?
        .with_geometry(40, 20)
        .context("Failed to set geometry")?
        .with_symbol_map(&symbol_map)
        .context("Failed to set symbol map")?;

    // Create canvas using safe wrapper
    let mut canvas = SafeCanvas::new(&config)
        .context("Failed to create canvas")?;

    // Draw pixels
    canvas.draw_all_pixels(
        chafa_sys::CHAFA_PIXEL_RGBA8_UNASSOCIATED as u32,
        pixels.as_ptr(),
        PIX_WIDTH,
        PIX_HEIGHT,
        PIX_WIDTH * N_CHANNELS,
    ).context("Failed to draw pixels")?;

    // Generate and print output
    let result = canvas.build_ansi()
        .context("Failed to build ANSI output")?;
    println!("{}", result);

    // All resources are automatically cleaned up via Drop
    Ok(())
}

/// Render markdown to terminal art
fn render_markdown(
    file: Option<String>,
    stdin: bool,
    theme: String,
    save_image: Option<String>,
    args: &Args,
) -> Result<()> {
    // Read markdown content
    let markdown_content = if stdin {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        content
    } else if let Some(ref file) = file {
        fs::read_to_string(file).context(format!("Failed to read file: {}", file))?
    } else {
        eprintln!("Error: No input specified. Use a FILE argument, --stdin, or 'view' command.");
        std::process::exit(1);
    };

    if args.verbose {
        eprintln!("Stage 1: Parsing Markdown with comrak...");
    }

    // Stage 1: Markdown → HTML
    let theme = Theme::from_str(&theme);
    let html = markdown::markdown_to_styled_html(&markdown_content, theme);

    if args.verbose {
        eprintln!("Stage 1 complete: Generated {} bytes of HTML", html.len());
        // debug snippet of HTML
        eprintln!("[debug] HTML snippet: {}", &html.chars().take(200).collect::<String>());
    }

    // Stage 2: HTML → PNG (or fallback to raw HTML if Chrome is missing)
    if args.verbose {
        eprintln!("Stage 2: Rendering HTML to PNG with headless_chrome...");
    }

    // Calculate viewport dimensions
    // Each character cell is roughly 8x16 pixels in monospace
    // For good quality, we need ~2x horizontal resolution
    let viewport_width = args.width * 10;
    let viewport_height = args.height * 20;

    let terminal_output = if image::check_chrome_available() {
        if args.verbose {
            eprintln!("Chrome reported available, proceeding with PNG conversion");
        }
        let png_bytes = image::html_to_png(&html, viewport_width, Some(viewport_height))?;

        eprintln!("[debug] html_to_png produced {} bytes", png_bytes.len());
        if png_bytes.len() >= 4 {
            eprintln!("[debug] png header after html_to_png = {:02x?}", &png_bytes[..4]);
        }

        if args.verbose {
            eprintln!("Stage 2 complete: Generated {} bytes of PNG", png_bytes.len());
        }

        // Optionally save PNG
        if let Some(ref save_path) = save_image {
            image::save_png(&png_bytes, save_path)?;
            eprintln!("Saved PNG to: {}", save_path);
        }

        // Stage 3: PNG → Terminal Art
        if args.verbose {
            eprintln!("Stage 3: Converting PNG to terminal art with libchafa...");
        }

        let config = ChafaConfig {
            width: args.width,
            height: args.height,
            format: OutputFormat::from_str(&args.format),
            dither: !args.no_dither,
        };

        let output = chafa::png_to_terminal_art(&png_bytes, &config)?;

        if args.verbose {
            eprintln!(
                "Stage 3 complete: Generated {} bytes of terminal output",
                output.len()
            );
        }
        output
    } else {
        eprintln!("Warning: headless Chrome not found; outputting HTML instead of terminal art.\nInstall Chrome/Chromium or set CHROME_PATH environment variable.");
        if args.verbose {
            eprintln!("Chrome not available; falling back to raw HTML output");
        }
        html.clone()
    };

    // Print to stdout
    print!("{}", terminal_output);

    Ok(())
}
