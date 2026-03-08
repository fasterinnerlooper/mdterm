# PNG Generation Refactoring Plan: Headless Chrome → WeasyPrint

## Executive Summary

This document outlines the refactoring plan to replace the headless Chrome dependency with WeasyPrint for HTML to PNG conversion in the mdterm project.

## Current State Analysis

### Files Using Headless Chrome
- **`Cargo.toml`** (line 15): Dependency declaration `headless_chrome = "1.0"`
- **`src/image.rs`** (lines 1-97): Complete implementation using headless_chrome

### Current Implementation Issues
1. The function `html_to_png()` actually returns **PDF bytes**, not PNG
2. Requires Chrome/Chromium browser installation
3. Heavy resource consumption (full browser process)
4. Slow startup time

### Function Signatures to Preserve (Backward Compatibility)
```rust
// Current signatures that must be preserved
pub fn html_to_png(html: &str, viewport_width: u32, viewport_height: Option<u32>) -> Result<Vec<u8>>
pub fn save_png(png_bytes: &[u8], path: &str) -> Result<()>
```

---

## Implementation Approach

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        mdterm Pipeline                          │
├─────────────────────────────────────────────────────────────────┤
│  Stage 1          Stage 2              Stage 3                 │
│  Markdown    →    HTML+CSS        →    PNG        →   Terminal │
│  (comrak)         (WeasyPrint)        (Cairo)         (libchafa)│
└─────────────────────────────────────────────────────────────────┘
```

### WeasyPrint Integration Strategy

Since WeasyPrint is a Python library, we'll use a hybrid approach:

1. **Invoke WeasyPrint via Python subprocess**
2. **Generate PDF first** (WeasyPrint's native output)
3. **Convert PDF to PNG** using a Rust PDF rendering library

#### Option A: WeasyPrint + pdf-to-png converter (RECOMMENDED)
- WeasyPrint generates PDF with excellent CSS support
- Use `pdf-to-png` or `resvg` for conversion
- Pros: Best CSS compatibility, reliable output
- Cons: Requires Python installation

#### Option B: Direct PNG via WeasyPrint CLI
- WeasyPrint can output PNG directly via Cairo
- Invoke: `python -m weasyprint input.html output.png`
- Pros: Single step
- Cons: Less control over resolution

---

## Detailed Implementation Steps

### Step 1: Update Cargo.toml

**Remove:**
```toml
# Stage 2: HTML → PNG (screenshot)
headless_chrome = "1.0"
```

**Add:**
```toml
# For subprocess execution
tokio = { version = "1", features = ["process", "rt-multi-thread"] }

# For PDF to PNG conversion (choose one)
# Option A: PDF rendering
# printpdf = "0.7"
# Option B: SVG-based rendering  
# resvg = "0.37"
```

### Step 2: Modify src/image.rs

Replace the headless_chrome implementation with:

```rust
//! Stage 2: HTML to PNG Image Export
//! 
//! Converts rendered HTML to PNG using WeasyPrint (Python) via subprocess.
//! WeasyPrint provides excellent CSS support and produces high-quality output.

use anyhow::{Context, Result};
use std::process::Command;
use std::fs;
use std::path::Path;

/// Check if WeasyPrint is available
pub fn check_weasyprint_available() -> bool {
    Command::new("python")
        .args(["-m", "weasyprint", "--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Render HTML to PNG bytes using WeasyPrint
/// 
/// This function:
/// 1. Writes HTML to a temporary file
/// 2. Invokes WeasyPrint via Python to convert HTML → PDF
/// 3. Converts PDF → PNG using resvg or similar
/// 4. Returns PNG bytes
pub fn html_to_png(html: &str, viewport_width: u32, viewport_height: Option<u32>) -> Result<Vec<u8>> {
    let height = viewport_height.unwrap_or(800);
    
    // Create temporary files for HTML and output
    let temp_dir = std::env::temp_dir();
    let html_path = temp_dir.join("mdterm_input.html");
    let pdf_path = temp_dir.join("mdterm_output.pdf");
    let png_path = temp_dir.join("mdterm_output.png");
    
    // Write HTML to temporary file
    fs::write(&html_path, html)
        .context("Failed to write temporary HTML file")?;
    
    // Calculate CSS dimensions for viewport
    let css_width = format!("{}px", viewport_width * 10);
    let css_height = format!("{}px", height);
    
    // Invoke WeasyPrint to generate PDF
    let weasyprint_result = Command::new("python")
        .args([
            "-m", "weasyprint",
            "-w", &css_width,
            &html_path.to_string_lossy(),
            &pdf_path.to_string_lossy(),
        ])
        .output()
        .context("Failed to execute WeasyPrint")?;
    
    if !weasyprint_result.status.success() {
        let stderr = String::from_utf8_lossy(&weasyprint_result.stderr);
        anyhow::bail!("WeasyPrint failed: {}", stderr);
    }
    
    // Convert PDF to PNG using resvg or pdf-render
    // ... (implementation details)
    
    // Read PNG output
    let png_bytes = fs::read(&png_path)
        .context("Failed to read PNG output")?;
    
    // Cleanup temporary files
    let _ = fs::remove_file(&html_path);
    let _ = fs::remove_file(&pdf_path);
    let _ = fs::remove_file(&png_path);
    
    Ok(png_bytes)
}

/// Generate PDF output (preserving existing functionality)
pub fn html_to_pdf(html: &str, viewport_width: u32) -> Result<Vec<u8>> {
    // Similar implementation but output PDF directly
    let temp_dir = std::env::temp_dir();
    let html_path = temp_dir.join("mdterm_input.html");
    let pdf_path = temp_dir.join("mdterm_output.pdf");
    
    fs::write(&html_path, html)?;
    
    let css_width = format!("{}px", viewport_width * 10);
    
    let output = Command::new("python")
        .args([
            "-m", "weasyprint",
            "-w", &css_width,
            &html_path.to_string_lossy(),
            &pdf_path.to_string_lossy(),
        ])
        .output()
        .context("Failed to execute WeasyPrint")?;
    
    if !output.status.success() {
        anyhow::bail!("WeasyPrint failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    
    let pdf_bytes = fs::read(&pdf_path)?;
    
    // Cleanup
    let _ = fs::remove_file(&html_path);
    let _ = fs::remove_file(&pdf_path);
    
    Ok(pdf_bytes)
}
```

### Step 3: Add PDF to PNG Conversion

For converting PDF to PNG, consider these Rust libraries:

| Library | Pros | Cons |
|---------|------|------|
| `resvg` | Fast, accurate | Requires converting HTML→SVG first |
| `printpdf` | PDF native | Limited rendering |
| `pdf-to-png` | Direct conversion | Less maintained |

**Recommendation**: Use WeasyPrint's built-in PNG support via CLI:
```bash
python -m weasyprint -w 800 input.html output.png
```

This avoids the need for a separate PDF→PNG converter.

### Step 4: Update main.rs

Add WeasyPrint availability check at startup:

```rust
fn main() -> Result<()> {
    // Check for WeasyPrint availability
    if !image::check_weasyprint_available() {
        eprintln!("Error: WeasyPrint is not installed.");
        eprintln!("Please install WeasyPrint:");
        eprintln!("  pip install weasyprint");
        std::process::exit(1);
    }
    // ... rest of main
}
```

### Step 5: Update README.md

Update installation instructions:

```markdown
## Requirements

### All Platforms
- [Rust toolchain](https://rustup.rs/) (1.70+)
- **Python 3.7+** with WeasyPrint: `pip install weasyprint`

### Optional
- libchafa for terminal art output (see platform-specific instructions)
```

---

## Backward Compatibility Considerations

1. **Function Signatures**: Maintain `html_to_png()` and `save_png()` signatures
2. **Return Type**: Must return actual PNG bytes (not PDF)
3. **Error Handling**: Preserve existing error message format
4. **CLI Arguments**: No changes to CLI interface

---

## Testing Strategy

### Test Cases
1. **Basic HTML rendering**: Simple H1, paragraph, bold text
2. **Complex styling**: Tables, code blocks, blockquotes
3. **Dark/Light themes**: Verify CSS applies correctly
4. **Large documents**: Multi-page HTML with scrolling
5. **Unicode content**: Non-ASCII characters
6. **Responsive dimensions**: Various viewport sizes

### Verification Steps
1. Run existing tests: `cargo test`
2. Test with sample markdown: `cargo run --release -- test.md`
3. Save PNG output: `cargo run --release -- --save-image output.png test.md`
4. Verify PNG is valid: Check file header (PNG magic bytes)

---

## Dependencies Summary

| Dependency | Purpose | Platform |
|------------|---------|----------|
| `comrak` | Markdown → HTML | Rust (built-in) |
| `weasyprint` | HTML → PDF/PNG | Python (external) |
| `resvg` or `printpdf` | PDF → PNG (if needed) | Rust |
| `image` | PNG processing | Rust (existing) |
| `libchafa` | Terminal art | System library |

---

## Migration Checklist

- [ ] Remove `headless_chrome` from Cargo.toml
- [ ] Add required Rust dependencies
- [ ] Implement WeasyPrint subprocess integration
- [ ] Add PDF→PNG conversion (or use WeasyPrint PNG directly)
- [ ] Update main.rs with WeasyPrint availability check
- [ ] Update README.md with new installation instructions
- [ ] Run tests to verify functionality
- [ ] Verify backward compatibility
- [ ] Test with various HTML templates and styles
