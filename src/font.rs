use fontdue::{Font, FontSettings};
use std::collections::HashMap;

pub struct FontRenderer {
    font: Option<Font>,
    font_cache: HashMap<(char, u32), (Vec<u8>, usize, usize)>, // (bitmap, width, height)
}

impl FontRenderer {
    pub fn new(font_data: &[u8]) -> Self {
        let font = Font::from_bytes(font_data, FontSettings::default()).ok();
        FontRenderer {
            font,
            font_cache: HashMap::new(),
        }
    }

    pub fn new_empty() -> Self {
        FontRenderer {
            font: None,
            font_cache: HashMap::new(),
        }
    }

    /// Rasterize a character at the given pixel size.
    /// Returns (bitmap, width, height) where bitmap is a grayscale alpha channel.
    pub fn rasterize_char(&mut self, ch: char, size: f32) -> (Vec<u8>, usize, usize) {
        let key = (ch, size.to_bits());
        if let Some(cached) = self.font_cache.get(&key) {
            return cached.clone();
        }

        if let Some(ref font) = self.font {
            let (metrics, bitmap) = font.rasterize(ch, size);
            let result = (bitmap, metrics.width, metrics.height);
            self.font_cache.insert(key, result.clone());
            result
        } else {
            (Vec::new(), 0, 0)
        }
    }

    pub fn get_char_advance(&self, ch: char, size: f32) -> usize {
        if let Some(ref font) = self.font {
            let metrics = font.metrics(ch, size);
            metrics.advance_width as usize
        } else {
            (size * 0.6) as usize
        }
    }

    pub fn has_font(&self) -> bool {
        self.font.is_some()
    }
}
