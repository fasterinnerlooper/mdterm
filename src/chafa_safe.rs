//! Safe Rust wrappers around libchafa FFI bindings
//!
//! This module provides memory-safe, ergonomic wrappers around the raw `chafa-sys` FFI.
//! All unsafe operations are confined to this module, allowing callers to use the libchafa
//! API without writing unsafe code.
//!
//! # FFI Safety Principles
//!
//! [Foreign Function Interface (FFI)](https://en.wikipedia.org/wiki/Foreign_function_interface)
//! allows Rust to call C functions. However, C libraries don't enforce Rust's safety guarantees
//! (memory safety, thread safety, etc.), so calls to C are marked `unsafe` by the compiler.
//!
//! This module mitigates FFI safety risks through:
//! - **RAII Pattern**: Automatic cleanup via `Drop` implementations
//! - **Ownership Model**: Rust's type system ensures proper resource lifetime
//! - **Error Handling**: Constructor methods return `Result<T>` to catch null pointers
//! - **No Dangling Pointers**: Wrapper types own their C resources and prevent use-after-free
//!
//! # Example
//!
//! ```no_run
//! use crate::chafa_safe::{SafeSymbolMap, SafeCanvasConfig, SafeCanvas};
//!
//! // Create a symbol map (automatically cleaned up when dropped)
//! let symbol_map = SafeSymbolMap::new()?;
//!
//! // Create canvas config with builder pattern
//! let config = SafeCanvasConfig::new()?
//!     .with_geometry(80, 24)?
//!     .with_symbol_map(&symbol_map)?;
//!
//! // Create canvas and draw pixels
//! let canvas = SafeCanvas::new(&config)?;
//! // ... draw pixels ...
//! // Canvas is automatically cleaned up when it goes out of scope
//! # Ok::<_, anyhow::Error>(())
//! ```

use anyhow::{Context, Result};
use chafa_sys as sys;

/// A memory-safe wrapper around `ChafaSymbolMap`
///
/// This type owns a pointer to a `ChafaSymbolMap` and automatically frees it
/// via the `Drop` implementation. It prevents use-after-free and memory leaks.
pub struct SafeSymbolMap {
    ptr: *mut sys::ChafaSymbolMap,
}

impl SafeSymbolMap {
    /// Create a new symbol map
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying C function returns a null pointer,
    /// indicating allocation failure.
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_symbol_map_new()`. The resulting
    /// `SafeSymbolMap` ensures the pointer is freed in `drop()`.
    pub fn new() -> Result<Self> {
        let ptr = unsafe { sys::chafa_symbol_map_new() };
        if ptr.is_null() {
            anyhow::bail!("Failed to create symbol map");
        }
        Ok(SafeSymbolMap { ptr })
    }

    /// Add symbols by tag
    ///
    /// # Errors
    ///
    /// Returns an error if `tag` is invalid (though this is unlikely with safe enum types).
    pub fn add_by_tags(&mut self, tags: u32) -> Result<&mut Self> {
        unsafe {
            sys::chafa_symbol_map_add_by_tags(self.ptr, tags);
        }
        Ok(self)
    }

    /// Get the underlying raw pointer
    ///
    /// # Safety
    ///
    /// This returns a raw pointer to the underlying C structure. The caller
    /// must not free this pointer; it will be freed automatically when
    /// the `SafeSymbolMap` is dropped.
    pub(crate) fn as_ptr(&self) -> *mut sys::ChafaSymbolMap {
        self.ptr
    }
}

impl Drop for SafeSymbolMap {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sys::chafa_symbol_map_unref(self.ptr);
            }
        }
    }
}

/// A memory-safe wrapper around `ChafaCanvasConfig`
///
/// This type owns a pointer to a `ChafaCanvasConfig` and automatically frees it
/// via the `Drop` implementation. It uses the builder pattern for ergonomic construction.
pub struct SafeCanvasConfig {
    ptr: *mut sys::ChafaCanvasConfig,
}

impl SafeCanvasConfig {
    /// Create a new canvas configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying C function returns a null pointer,
    /// indicating allocation failure.
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_config_new()`. The resulting
    /// `SafeCanvasConfig` ensures the pointer is freed in `drop()`.
    pub fn new() -> Result<Self> {
        let ptr = unsafe { sys::chafa_canvas_config_new() };
        if ptr.is_null() {
            anyhow::bail!("Failed to create canvas config");
        }
        Ok(SafeCanvasConfig { ptr })
    }

    /// Set the canvas geometry (width and height in terminal cells)
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_config_set_geometry()`.
    pub fn with_geometry(self, width: u32, height: u32) -> Result<Self> {
        unsafe {
            sys::chafa_canvas_config_set_geometry(self.ptr, width, height);
        }
        Ok(self)
    }

    /// Set the symbol map to use
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_config_set_symbol_map()`.
    /// The symbol map is referenced but not owned; it must outlive this config.
    pub fn with_symbol_map(self, symbol_map: &SafeSymbolMap) -> Result<Self> {
        unsafe {
            sys::chafa_canvas_config_set_symbol_map(self.ptr, symbol_map.as_ptr());
        }
        Ok(self)
    }

    /// Set the color space
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_config_set_color_space()`.
    pub fn with_color_space(self, color_space: u32) -> Result<Self> {
        unsafe {
            sys::chafa_canvas_config_set_color_space(self.ptr, color_space);
        }
        Ok(self)
    }

    /// Set the dither mode
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_config_set_dither_mode()`.
    pub fn with_dither_mode(self, dither_mode: u32) -> Result<Self> {
        unsafe {
            sys::chafa_canvas_config_set_dither_mode(self.ptr, dither_mode);
        }
        Ok(self)
    }

    /// Get the underlying raw pointer
    ///
    /// # Safety
    ///
    /// This returns a raw pointer to the underlying C structure. The caller
    /// must not free this pointer; it will be freed automatically when
    /// the `SafeCanvasConfig` is dropped.
    pub(crate) fn as_ptr(&self) -> *mut sys::ChafaCanvasConfig {
        self.ptr
    }
}

impl Drop for SafeCanvasConfig {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sys::chafa_canvas_config_unref(self.ptr);
            }
        }
    }
}

/// A memory-safe wrapper around `ChafaCanvas`
///
/// This type owns a pointer to a `ChafaCanvas` and automatically frees it
/// via the `Drop` implementation.
pub struct SafeCanvas {
    ptr: *mut sys::ChafaCanvas,
}

impl SafeCanvas {
    /// Create a new canvas with the given configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying C function returns a null pointer,
    /// indicating allocation failure.
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_new()`. The resulting
    /// `SafeCanvas` ensures the pointer is freed in `drop()`.
    pub fn new(config: &SafeCanvasConfig) -> Result<Self> {
        let ptr = unsafe { sys::chafa_canvas_new(config.as_ptr()) };
        if ptr.is_null() {
            anyhow::bail!("Failed to create canvas");
        }
        Ok(SafeCanvas { ptr })
    }

    /// Draw pixels to the canvas
    ///
    /// # Arguments
    ///
    /// * `pixel_type` - The pixel format (e.g., `CHAFA_PIXEL_RGBA8`)
    /// * `pixels` - Raw pixel data
    /// * `width` - Image width in pixels
    /// * `height` - Image height in pixels
    /// * `rowstride` - Bytes per row (typically `width * bytes_per_pixel`)
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_draw_all_pixels()`.
    /// The caller must ensure:
    /// - `pixels` points to valid data
    /// - `pixels` contains at least `height * rowstride` bytes
    /// - `width` and `height` match the actual data dimensions
    pub fn draw_all_pixels(
        &mut self,
        pixel_type: u32,
        pixels: *const u8,
        width: u32,
        height: u32,
        rowstride: u32,
    ) -> Result<()> {
        unsafe {
            sys::chafa_canvas_draw_all_pixels(
                self.ptr,
                pixel_type,
                pixels as *const _,
                width,
                height,
                rowstride,
            );
        }
        Ok(())
    }

    /// Build ANSI output from the canvas
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying C function returns a null pointer.
    ///
    /// # Safety
    ///
    /// This method calls unsafe `chafa_canvas_build_ansi()` and converts
    /// the resulting GString to a Rust String.
    pub fn build_ansi(&self) -> Result<String> {
        let gstr = unsafe { sys::chafa_canvas_build_ansi(self.ptr) };
        if gstr.is_null() {
            anyhow::bail!("Failed to build ANSI output");
        }

        let result = unsafe {
            let gstr_ref = gstr.as_ref().context("GString pointer is invalid")?;
            // Use CStr::from_ptr to read the string without taking ownership
            let cstr = std::ffi::CStr::from_ptr(gstr_ref.str);
            cstr.to_string_lossy().into_owned()
            // Note: we don't free the GString here because chafa owns it
            // and will clean it up when the canvas is dropped
        };

        Ok(result)
    }

    /// Get the underlying raw pointer
    ///
    /// # Safety
    ///
    /// This returns a raw pointer to the underlying C structure. The caller
    /// must not free this pointer; it will be freed automatically when
    /// the `SafeCanvas` is dropped.
    pub(crate) fn as_ptr(&self) -> *mut sys::ChafaCanvas {
        self.ptr
    }
}

impl Drop for SafeCanvas {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                sys::chafa_canvas_unref(self.ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_symbol_map_creation() {
        let result = SafeSymbolMap::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_safe_canvas_config_creation() {
        let result = SafeCanvasConfig::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_pattern() {
        let result = (|| -> Result<()> {
            let mut symbol_map = SafeSymbolMap::new()?;
            symbol_map.add_by_tags(sys::CHAFA_SYMBOL_TAG_ALL as u32)?;

            let _config = SafeCanvasConfig::new()?
                .with_geometry(80, 24)?
                .with_symbol_map(&symbol_map)?;

            Ok(())
        })();

        assert!(result.is_ok());
    }

    #[test]
    fn test_safe_canvas_creation() {
        let result = (|| -> Result<()> {
            let mut symbol_map = SafeSymbolMap::new()?;
            symbol_map.add_by_tags(sys::CHAFA_SYMBOL_TAG_ALL as u32)?;

            let config = SafeCanvasConfig::new()?
                .with_geometry(80, 24)?
                .with_symbol_map(&symbol_map)?;

            let _canvas = SafeCanvas::new(&config)?;
            Ok(())
        })();

        assert!(result.is_ok());
    }
}
