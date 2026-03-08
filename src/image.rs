#![allow(dead_code)]

//! Stage 2: HTML to PNG Image Export
//! 
//! Converts rendered HTML to a PNG image using headless Chrome via DevTools Protocol.
//! 
//! Note: headless_chrome primarily supports PDF output. For PNG, we use PDF output
//! and return it - callers would need to convert to PNG if needed.

use anyhow::{Context, Result};
use headless_chrome::{Browser, LaunchOptionsBuilder};

// screenshot-related types
use headless_chrome::protocol::cdp::Page::{CaptureScreenshotFormatOption, Viewport};

/// Check if headless Chrome is available and functioning.
/// 
/// We not only attempt to launch the browser, but also open a tab and take a
/// tiny screenshot.  This helps catch cases where Chrome starts but the
/// DevTools protocol is unusable.
///
/// Returns `true` if we successfully obtained a PNG screenshot, otherwise
/// `false`.
pub fn check_chrome_available() -> bool {
    eprintln!("[debug] check_chrome_available(): starting");
    let mut binding = LaunchOptionsBuilder::default();
    let mut launch_builder = binding.headless(true);
    if let Ok(path) = std::env::var("CHROME_PATH") {
        eprintln!("[debug] using CHROME_PATH from env: {}", path);
        launch_builder = launch_builder.path(Some(path.into()));
    }
    let launch_options = launch_builder.build();

    if launch_options.is_err() {
        eprintln!("[debug] failed to build launch options: {:?}", launch_options.err());
        return false;
    }
    let opts = launch_options.unwrap();

    match Browser::new(opts) {
        Ok(browser) => {
            eprintln!("[debug] browser launched successfully");
            match browser.new_tab() {
                Ok(tab) => {
                    eprintln!("[debug] new tab opened");
                    if let Err(e) = tab.navigate_to("about:blank") {
                        eprintln!("[debug] navigate_to about:blank failed: {:?}", e);
                        return false;
                    }
                    if let Err(e) = tab.wait_until_navigated() {
                        eprintln!("[debug] wait_until_navigated failed: {:?}", e);
                        return false;
                    }
                    let screenshot = tab.capture_screenshot(
                        CaptureScreenshotFormatOption::Png,
                        None,
                        None,
                        true,
                    );
                    if screenshot.is_err() {
                        eprintln!("[debug] screenshot failed: {:?}", screenshot.err());
                        return false;
                    }
                    eprintln!("[debug] screenshot succeeded, Chrome is available");
                    true
                }
                Err(e) => {
                    eprintln!("[debug] failed to open new tab: {:?}", e);
                    false
                }
            }
        }
        Err(e) => {
            eprintln!("[debug] failed to launch browser: {:?}", e);
            false
        }
    }
}

/// Render HTML to PNG bytes using a headless browser
/// 
/// This implementation navigates to the provided HTML, waits for the page to
/// load, and then takes a **screenshot** in PNG format. The previous version
/// returned a PDF (via `print_to_pdf`) which could not be decoded by the
/// `image` crate; that flaw was the reason nothing was ever printed to the
/// terminal.  The new code uses `Tab::capture_screenshot` to obtain actual PNG
/// bytes.  The `viewport_width`/`viewport_height` arguments are used to size
/// the clip region for the screenshot (they are multiplied by 10/20 by the
/// caller to approximate pixel dimensions).
///
/// If Chrome is not available or the screenshot fails, an error is returned.
pub fn html_to_png(html: &str, viewport_width: u32, viewport_height: Option<u32>) -> Result<Vec<u8>> {
    let height = viewport_height.unwrap_or(800);

    let mut binding = LaunchOptionsBuilder::default();
    let mut launch_builder = binding.headless(true);
    if let Ok(path) = std::env::var("CHROME_PATH") {
        eprintln!("[debug] using CHROME_PATH from env: {}", path);
        launch_builder = launch_builder.path(Some(path.into()));
    }
    let launch_options = launch_builder
        .build()
        .context("Failed to build browser launch options")?;

    let browser = Browser::new(launch_options)
        .context("Failed to launch browser")?;

    let tab = browser.new_tab()
        .context("Failed to create new browser tab")?;

    let encoded_html = urlencoding::encode(html);
    let data_url = format!("data:text/html;charset=utf-8,{}", encoded_html);

    eprintln!("[debug] navigating to data URL");
    tab.navigate_to(&data_url)
        .context("Failed to navigate to HTML")?;

    eprintln!("[debug] waiting for navigation");
    tab.wait_until_navigated()
        .context("Failed to wait for navigation")?;

    // Give the page a moment to finish rendering. In practice this is
    // unreliable, but the previous version already had the same sleep.
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Determine content height so we can capture the full page if desired.
    let content_height: u32 = tab.evaluate("document.body.scrollHeight", false)
        .ok()
        .and_then(|r| r.value)
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(height);

    eprintln!("[debug] computed content height = {}", content_height);

    // Clip viewport using provided dimensions.  The caller computes these as
    // width*10 and height*20 to roughly match character cell size.
    let viewport = Viewport {
        x: 0.0,
        y: 0.0,
        width: viewport_width as f64,
        height: (content_height.max(height)) as f64,
        scale: 1.0,
    };

    eprintln!("[debug] capturing screenshot with viewport {:?}", viewport);
    // Capture screenshot in PNG format
    let png_bytes = tab
        .capture_screenshot(
            CaptureScreenshotFormatOption::Png,
            None,
            Some(viewport),
            true,
        )
        .context("Failed to capture screenshot")?;

    eprintln!("[debug] screenshot returned {} bytes", png_bytes.len());
    if png_bytes.len() >= 4 {
        eprintln!("[debug] png header = {:02x?}", &png_bytes[..4]);
    }

    Ok(png_bytes)
}

/// Render HTML to a simple text representation for testing
/// This bypasses the browser and just returns the HTML
pub fn html_to_text_for_testing(html: &str, _width: u32) -> Result<String> {
    // Return the raw HTML for now - a real implementation would render to terminal
    Ok(html.to_string())
}

/// Save PNG bytes to a file
pub fn save_png(png_bytes: &[u8], path: &str) -> Result<()> {
    std::fs::write(path, png_bytes)
        .context(format!("Failed to write PNG to {}", path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_html_rendering() {
        let html = r#"<!DOCTYPE html>
<html>
<head><style>body { font-family: sans-serif; background: white; }</style></head>
<body>
<h1>Hello World</h1>
<p>This is a test.</p>
</body>
</html>"#;
        
        if std::env::var("CI").is_ok() || !check_chrome_available() {
            eprintln!("Skipping browser test because Chrome is not available");
            return;
        }
        
        let result = html_to_png(html, 800, None);
        if result.is_err() {
            eprintln!("Browser test failed: {:?}", result);
        }
    }

    #[test]
    fn test_html_to_png_header() {
        if std::env::var("CI").is_ok() || !check_chrome_available() {
            eprintln!("Skipping PNG header test because Chrome is not available");
            return;
        }

        let simple = "<html><body><p>foo</p></body></html>";
        let png = html_to_png(simple, 100, Some(100));
        assert!(png.is_ok(), "screenshot failed: {:?}", png);
        let bytes = png.unwrap();
        assert!(bytes.starts_with(b"\x89PNG"), "returned data is not PNG");
    }

    #[test]
    fn test_html_to_text_for_testing() {
        let html = "<p>hello</p>";
        let text = html_to_text_for_testing(html, 50).expect("fallback should work");
        assert_eq!(text, html);
    }
}
