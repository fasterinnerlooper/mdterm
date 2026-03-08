# mdterm

A three-stage pipeline that converts Markdown files to terminal art using established libraries:

1. **Stage 1 (Markdown → HTML):** Uses [`comrak`](https://crates.io/crates/comrak) - the same GitHub-flavored Markdown parser used by GitHub
1. **Stage 2 (HTML → PNG):** Uses [`headless_chrome`](https://crates.io/crates/headless_chrome) to render HTML via Chrome DevTools Protocol and take a PNG screenshot of the page
3. **Stage 3 (PNG → Terminal):** Uses [`libchafa`](https://hpjansson.org/chafa/) via FFI for high-quality terminal art

## Features

- GitHub-flavored Markdown support (tables, code blocks, task lists, etc.)
- Light and dark themes with GitHub-inspired styling
- Renders Markdown through a real browser engine for pixel-perfect output
- Outputs ANSI terminal art using industry-standard libchafa

## Requirements

### All Platforms
- [Rust toolchain](https://rustup.rs/) (1.70+)
- Chrome, Chromium, or Microsoft Edge browser installed (if missing, mdterm
  will fall back to printing the intermediate HTML rather than terminal art)

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
| `-f, --format` | Output format: ansi/truecolor/sixel/kitty/iterm2 (ANSI/truecolor use built-in libchafa; other formats delegate to the `chafa` CLI) | ansi |
| `--no-dither` | Disable dithering | false |
| `--save-image` | Save intermediate PNG to file | (none) |
| `-v, --verbose` | Enable verbose logging | false |

## Current Status

✅ **Pipeline is now functional** - Markdown is converted all the way to terminal art

- ✅ Stage 1: Markdown → HTML (comrak)
- ✅ Stage 2: HTML → PNG (headless_chrome screenshot)
- ✅ Stage 3: PNG → Terminal (libchafa via FFI; ANSI output now honours background colors and respects the `--format` flag)

You still need to have the external dependencies installed (Chrome/Chromium and
libchafa) for the stages to run correctly.  If Chrome is missing the program
will exit with a helpful error.  The libchafa library is used for the built‑in
ANSI/truecolor rendering; when you request `sixel`, `kitty` or `iterm2` format
mdterm now shells out to the external `chafa` CLI, so that tool must also be
available on your PATH for those modes to work.

## Architecture

```
Markdown → comrak → HTML+CSS → headless_chrome → PDF → libchafa → ANSI → stdout
```

## License

MIT
