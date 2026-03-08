# mdterm

A three-stage pipeline that converts Markdown files to terminal art using established libraries:

1. **Stage 1 (Markdown → HTML):** Uses [`comrak`](https://crates.io/crates/comrak) - the same GitHub-flavored Markdown parser used by GitHub
2. **Stage 2 (HTML → PNG):** Uses [`headless_chrome`](https://crates.io/crates/headless_chrome) to render HTML via Chrome DevTools Protocol
3. **Stage 3 (PNG → Terminal):** Uses [`libchafa`](https://hpjansson.org/chafa/) via FFI for high-quality terminal art

## Features

- GitHub-flavored Markdown support (tables, code blocks, task lists, etc.)
- Light and dark themes with GitHub-inspired styling
- Renders Markdown through a real browser engine for pixel-perfect output
- Outputs ANSI terminal art using industry-standard libchafa

## Requirements

### All Platforms
- [Rust toolchain](https://rustup.rs/) (1.70+)
- Chrome, Chromium, or Microsoft Edge browser installed

### Windows
- [MSYS2](https://www.msys2.org/) with mingw-w64-x86_64-chafa for libchafa:
  ```powershell
  pacman -S mingw-w64-x86_64-chafa
  ```

### Linux
- libchafa development files:
  ```bash
  # Ubuntu/Debian
  apt install libchafa-dev
  
  # Fedora
  dnf install chafa-devel
  
  # Arch
  pacman -S chafa
  ```

### macOS
- libchafa via Homebrew:
  ```bash
  brew install chafa
  ```

## Building

```bash
# Clone and build
cargo build --release

# Or build in debug mode
cargo build
```

## Usage

```bash
# Render a markdown file
cargo run --release -- test.md

# Use dark theme
cargo run --release -- --theme dark test.md

# Custom terminal dimensions
cargo run --release -- --width 120 --height 40 test.md

# Read from stdin
cat test.md | cargo run --release -- --stdin

# Save intermediate PNG for debugging
cargo run --release -- --save-image output.png test.md

# Verbose output
cargo run --release -- -v test.md
```

## CLI Options

| Option | Description | Default |
|--------|-------------|---------|
| `FILE` | Input markdown file | (required or --stdin) |
| `-s, --stdin` | Read from stdin | false |
| `-w, --width` | Terminal width in columns | 80 |
| `-H, --height` | Terminal height in rows | 24 |
| `-t, --theme` | Color theme: light/dark | light |
| `-f, --format` | Output format: ansi/truecolor/sixel/kitty/iterm2 | ansi |
| `--no-dither` | Disable dithering | false |
| `--save-image` | Save intermediate PNG to file | (none) |
| `-v, --verbose` | Enable verbose logging | false |

## Current Status

⚠️ **This is a work in progress.** The pipeline works up to Stage 2:

- ✅ Stage 1: Markdown → HTML (working with comrak)
- ✅ Stage 2: HTML → PNG (headless_chrome returns PDF, needs conversion)
- ⚠️ Stage 3: PNG → Terminal (needs libchafa to be installed)

To complete the pipeline, you need to:
1. Install libchafa (see platform-specific instructions above)
2. Uncomment the `chafa-sys` dependency in `Cargo.toml`
3. Implement the full chafa FFI bindings in `src/chafa.rs`

## Architecture

```
Markdown → comrak → HTML+CSS → headless_chrome → PDF → libchafa → ANSI → stdout
```

## License

MIT
