use crate::font::FontRenderer;
use std::collections::HashMap;

pub struct TerminalRenderer {
    /// Maximum render width in pixels
    width: usize,
    pixel_size: f32,
    block_elements: HashMap<char, char>,
}

impl TerminalRenderer {
    pub fn new(width: usize) -> Self {
        let mut block_elements = HashMap::new();
        block_elements.insert('▀', '\u{2580}');
        block_elements.insert('▄', '\u{2584}');
        block_elements.insert('█', '\u{2588}');

        TerminalRenderer {
            width,
            pixel_size: 8.0,
            block_elements,
        }
    }

    /// Render text as pixel art using Unicode half-block characters.
    /// Each terminal cell represents 1×2 pixels (top half / bottom half).
    /// Text is word-wrapped to fit within `self.width` pixels.
    pub fn render_text(&mut self, text: &str, font_renderer: &mut FontRenderer) -> String {
        if !font_renderer.has_font() {
            return text.to_string();
        }

        let size = self.pixel_size;
        let max_px_width = self.width;

        // Split text into words and wrap into lines that fit within max_px_width
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() {
            return String::new();
        }

        // Measure word widths in pixels
        let space_width = font_renderer.get_char_advance(' ', size).max(4);

        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();
        let mut current_width: usize = 0;

        for word in &words {
            let word_width: usize = word.chars()
                .map(|c| font_renderer.get_char_advance(c, size).max(1))
                .sum();

            if current_line.is_empty() {
                current_line.push_str(word);
                current_width = word_width;
            } else if current_width + space_width + word_width <= max_px_width {
                current_line.push(' ');
                current_line.push_str(word);
                current_width += space_width + word_width;
            } else {
                lines.push(current_line.clone());
                current_line = word.to_string();
                current_width = word_width;
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        // Render each wrapped line as pixels
        let mut result = String::new();
        for line in &lines {
            result.push_str(&self.render_line_pixels(line, font_renderer));
        }
        result
    }

    /// Render a single line of text as pixel art (no wrapping).
    fn render_line_pixels(&mut self, text: &str, font_renderer: &mut FontRenderer) -> String {
        let size = self.pixel_size;
        let threshold: u8 = 64;

        struct Glyph {
            bitmap: Vec<u8>,
            width: usize,
            height: usize,
            advance: usize,
        }

        let mut glyphs: Vec<Glyph> = Vec::new();
        let mut total_width: usize = 0;
        let mut max_height: usize = 0;

        for ch in text.chars() {
            let (bitmap, w, h) = font_renderer.rasterize_char(ch, size);
            let advance = font_renderer.get_char_advance(ch, size).max(if w > 0 { w } else { 4 });
            if h > max_height {
                max_height = h;
            }
            total_width += advance;
            glyphs.push(Glyph { bitmap, width: w, height: h, advance });
        }

        if max_height == 0 || total_width == 0 {
            return text.to_string();
        }

        // Cap width at max_px_width
        let render_width = total_width.min(self.width);

        // Build a flat pixel buffer
        let mut pixels = vec![0u8; render_width * max_height];

        let mut x_offset = 0usize;
        for glyph in &glyphs {
            if x_offset >= render_width {
                break;
            }
            let y_offset = if max_height > glyph.height {
                max_height - glyph.height
            } else {
                0
            };
            for gy in 0..glyph.height {
                for gx in 0..glyph.width {
                    let src_idx = gy * glyph.width + gx;
                    let dst_x = x_offset + gx;
                    let dst_y = y_offset + gy;
                    if dst_x < render_width && dst_y < max_height {
                        let dst_idx = dst_y * render_width + dst_x;
                        if src_idx < glyph.bitmap.len() {
                            pixels[dst_idx] = glyph.bitmap[src_idx];
                        }
                    }
                }
            }
            x_offset += glyph.advance;
        }

        // Convert pixel buffer to Unicode half-block characters
        let mut result = String::new();
        let rows = (max_height + 1) / 2;

        for row in 0..rows {
            let top_y = row * 2;
            let bot_y = row * 2 + 1;

            for x in 0..render_width {
                let top_px = if top_y < max_height {
                    pixels[top_y * render_width + x]
                } else {
                    0
                };
                let bot_px = if bot_y < max_height {
                    pixels[bot_y * render_width + x]
                } else {
                    0
                };

                let top_on = top_px >= threshold;
                let bot_on = bot_px >= threshold;

                match (top_on, bot_on) {
                    (true, true) => result.push('█'),
                    (true, false) => result.push('▀'),
                    (false, true) => result.push('▄'),
                    (false, false) => result.push(' '),
                }
            }
            result.push('\n');
        }

        result
    }

    pub fn render_code_block(&mut self, code: &str, font_renderer: &mut FontRenderer) -> String {
        let mut result = String::from("┌─ code ──────────────────────────────────────────────────────────────────────┐\n");
        for line in code.lines() {
            let rendered = self.render_line_pixels(line, font_renderer);
            result.push_str(&rendered);
        }
        result.push_str("└─────────────────────────────────────────────────────────────────────────────┘\n");
        result
    }

    pub fn render_blockquote(&mut self, text: &str, font_renderer: &mut FontRenderer) -> String {
        let rendered = self.render_text(text, font_renderer);
        let mut result = String::new();
        for line in rendered.lines() {
            result.push_str("│ ");
            result.push_str(line);
            result.push('\n');
        }
        result
    }

    pub fn render_list(&mut self, items: &[crate::markdown::MarkdownElement], font_renderer: &mut FontRenderer) -> String {
        let mut result = String::new();
        for item in items {
            result.push_str(&self.render_list_item(&item.content, font_renderer));
        }
        result
    }

    pub fn render_list_item(&mut self, item: &str, font_renderer: &mut FontRenderer) -> String {
        let bullet = self.render_line_pixels("• ", font_renderer);
        let content = self.render_text(item, font_renderer);
        let bullet_lines: Vec<&str> = bullet.lines().collect();
        let content_lines: Vec<&str> = content.lines().collect();
        let max_lines = bullet_lines.len().max(content_lines.len());
        let bullet_width = bullet_lines.first().map(|l| l.len()).unwrap_or(2);
        let mut result = String::new();
        for i in 0..max_lines {
            let b = bullet_lines.get(i).copied().unwrap_or("");
            let c = content_lines.get(i).copied().unwrap_or("");
            result.push_str(&format!("{:<width$}{}\n", b, c, width = bullet_width));
        }
        result
    }

    pub fn render_unicode_blocks(&mut self, width: usize, height: usize) -> String {
        let mut result = String::new();
        for y in 0..height {
            for x in 0..width {
                if (x + y) % 2 == 0 {
                    result.push('█');
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        result
    }

    pub fn detect_terminal_font_size(&self) -> (u32, u32) {
        (8, 16)
    }

    pub fn get_terminal_protocol(&self) -> String {
        "unicode_blocks".to_string()
    }

    pub fn optimize_rendering(&mut self, content: &str) -> String {
        content.to_string()
    }
}
