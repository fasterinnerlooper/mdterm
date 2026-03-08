//! Stage 1: Markdown Rendering
//! 
//! Converts Markdown text to styled HTML using comrak (GitHub's parser).

use comrak::{markdown_to_html, Options};

/// Theme for HTML rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dark" => Theme::Dark,
            _ => Theme::Light,
        }
    }
}

/// Convert Markdown to a complete HTML document with embedded CSS styling
pub fn markdown_to_styled_html(markdown: &str, theme: Theme) -> String {
    // Parse markdown to HTML using comrak
    // Use default options - GFM is enabled by default in newer versions
    let options = Options::default();
    
    let html_fragment = markdown_to_html(markdown, &options);
    
    // Wrap in complete HTML document with CSS
    wrap_in_html_template(&html_fragment, theme)
}

/// Wrap HTML fragment in a complete document with embedded CSS
fn wrap_in_html_template(html_fragment: &str, theme: Theme) -> String {
    let css = match theme {
        Theme::Light => LIGHT_THEME_CSS,
        Theme::Dark => DARK_THEME_CSS,
    };
    
    format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
{css}
    </style>
</head>
<body>
{html_fragment}
</body>
</html>"#)
}

/// Light theme CSS - GitHub-inspired styling
const LIGHT_THEME_CSS: &str = r#"
* {
    box-sizing: border-box;
}
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 1.5;
    color: #1f2328;
    background-color: #ffffff;
    padding: 24px;
    margin: 0;
    max-width: 100%;
}
h1, h2, h3, h4, h5, h6 {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.25;
}
h1 { font-size: 2em; border-bottom: 1px solid #d0d7de; padding-bottom: 0.3em; }
h2 { font-size: 1.5em; border-bottom: 1px solid #d0d7de; padding-bottom: 0.3em; }
h3 { font-size: 1.25em; }
h4 { font-size: 1em; }
h5 { font-size: 0.875em; }
h6 { font-size: 0.85em; color: #656d76; }
p { margin-top: 0; margin-bottom: 16px; }
a { color: #0969da; text-decoration: none; }
a:hover { text-decoration: underline; }
code {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace;
    font-size: 85%;
    padding: 0.2em 0.4em;
    background-color: rgba(175, 184, 193, 0.2);
    border-radius: 6px;
}
pre {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace;
    font-size: 85%;
    padding: 16px;
    overflow: auto;
    line-height: 1.45;
    background-color: #f6f8fa;
    border-radius: 6px;
}
pre code {
    padding: 0;
    background-color: transparent;
    border-radius: 0;
}
blockquote {
    margin: 0;
    padding: 0 1em;
    color: #656d76;
    border-left: 0.25em solid #d0d7de;
}
ul, ol {
    margin-top: 0;
    margin-bottom: 16px;
    padding-left: 2em;
}
li { margin-top: 0.25em; }
li + li { margin-top: 0.25em; }
table {
    border-spacing: 0;
    border-collapse: collapse;
    margin-bottom: 16px;
    width: 100%;
}
table th, table td {
    padding: 6px 13px;
    border: 1px solid #d0d7de;
}
table th { font-weight: 600; background-color: #f6f8fa; }
table tr:nth-child(2n) { background-color: #f6f8fa; }
hr {
    height: 0.25em;
    padding: 0;
    margin: 24px 0;
    background-color: #d0d7de;
    border: 0;
}
input[type="checkbox"] {
    margin-right: 0.5em;
}
img {
    max-width: 100%;
    height: auto;
}
"#;

/// Dark theme CSS
const DARK_THEME_CSS: &str = r#"
* {
    box-sizing: border-box;
}
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 1.5;
    color: #e6edf3;
    background-color: #0d1117;
    padding: 24px;
    margin: 0;
    max-width: 100%;
}
h1, h2, h3, h4, h5, h6 {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.25;
    color: #e6edf3;
}
h1 { font-size: 2em; border-bottom: 1px solid #30363d; padding-bottom: 0.3em; }
h2 { font-size: 1.5em; border-bottom: 1px solid #30363d; padding-bottom: 0.3em; }
h3 { font-size: 1.25em; }
h4 { font-size: 1em; }
h5 { font-size: 0.875em; }
h6 { font-size: 0.85em; color: #8b949e; }
p { margin-top: 0; margin-bottom: 16px; }
a { color: #58a6ff; text-decoration: none; }
a:hover { text-decoration: underline; }
code {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace;
    font-size: 85%;
    padding: 0.2em 0.4em;
    background-color: rgba(110, 118, 129, 0.4);
    border-radius: 6px;
    color: #e6edf3;
}
pre {
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Consolas, "Liberation Mono", monospace;
    font-size: 85%;
    padding: 16px;
    overflow: auto;
    line-height: 1.45;
    background-color: #161b22;
    border-radius: 6px;
}
pre code {
    padding: 0;
    background-color: transparent;
    border-radius: 0;
}
blockquote {
    margin: 0;
    padding: 0 1em;
    color: #8b949e;
    border-left: 0.25em solid #30363d;
}
ul, ol {
    margin-top: 0;
    margin-bottom: 16px;
    padding-left: 2em;
}
li { margin-top: 0.25em; }
li + li { margin-top: 0.25em; }
table {
    border-spacing: 0;
    border-collapse: collapse;
    margin-bottom: 16px;
    width: 100%;
}
table th, table td {
    padding: 6px 13px;
    border: 1px solid #30363d;
}
table th { font-weight: 600; background-color: #161b22; }
table tr:nth-child(2n) { background-color: #161b22; }
hr {
    height: 0.25em;
    padding: 0;
    margin: 24px 0;
    background-color: #30363d;
    border: 0;
}
input[type="checkbox"] {
    margin-right: 0.5em;
}
img {
    max-width: 100%;
    height: auto;
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_light_theme_output() {
        let md = "# Hello\n\nThis is **bold** and *italic*.";
        let html = markdown_to_styled_html(md, Theme::Light);
        assert!(html.contains("<h1>"));
        assert!(html.contains("font-weight: 600"));
    }
    
    #[test]
    fn test_dark_theme_output() {
        let md = "# Hello";
        let html = markdown_to_styled_html(md, Theme::Dark);
        assert!(html.contains("background-color: #0d1117"));
    }
}
