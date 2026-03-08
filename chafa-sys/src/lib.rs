//! FFI bindings to libchafa
//!
//! This crate provides low-level FFI bindings to the libchafa C library.
//! These bindings are generated from the libchafa header files and allow Rust code
//! to directly call C functions.
//!
//! # FFI Safety
//!
//! All functions in this module are `unsafe` because:
//! - C functions don't enforce Rust's memory safety guarantees
//! - Raw pointers can be null or point to invalid memory
//! - C code can violate borrowing rules
//! - Type safety is not guaranteed across the language boundary
//!
//! For a safer, more ergonomic API, use the `chafa_safe` module which wraps these
//! bindings with Rust's type system and RAII patterns.
//!
//! # Memory Management
//!
//! libchafa uses reference counting for memory management. Objects created with
//! functions like `chafa_symbol_map_new()` must be freed with corresponding
//! `chafa_*_unref()` functions. The `chafa_safe` module handles this automatically.
//!
//! # Typical Usage
//!
//! 1. Create a symbol map: `chafa_symbol_map_new()`
//! 2. Add symbols to it: `chafa_symbol_map_add_by_tags()`
//! 3. Create canvas config: `chafa_canvas_config_new()`
//! 4. Configure canvas: `chafa_canvas_config_set_*()` functions
//! 5. Create canvas: `chafa_canvas_new()`
//! 6. Draw pixels: `chafa_canvas_draw_all_pixels()`
//! 7. Generate output: `chafa_canvas_build_ansi()`
//! 8. Free all resources (manually here, automatic in `chafa_safe`)
//!
//! # Example (Unsafe - Use chafa_safe Instead)
//!
//! ```ignore
//! unsafe {
//!     let map = chafa_symbol_map_new();
//!     chafa_symbol_map_add_by_tags(map, CHAFA_SYMBOL_TAG_ALL);
//!
//!     let config = chafa_canvas_config_new();
//!     chafa_canvas_config_set_geometry(config, 80, 24);
//!     chafa_canvas_config_set_symbol_map(config, map);
//!
//!     let canvas = chafa_canvas_new(config);
//!     chafa_canvas_draw_all_pixels(canvas, CHAFA_PIXEL_RGBA8, pixels, width, height, rowstride);
//!
//!     let output = chafa_canvas_build_ansi(canvas);
//!     // Use output...
//!
//!     chafa_canvas_unref(canvas);
//!     chafa_canvas_config_unref(config);
//!     chafa_symbol_map_unref(map);
//! }
//! ```

use libc::{c_char, c_int, c_uint, c_void};

// ============================================================================
// Type Definitions
// ============================================================================

/// Opaque pointer to a ChafaSymbolMap
///
/// This is a reference-counted object that holds a set of symbols used for rendering.
/// Must be freed with `chafa_symbol_map_unref()` when no longer needed.
/// Can be shared between multiple canvas configs.
///
/// # Safety
///
/// The pointer returned by `chafa_symbol_map_new()` is valid until `chafa_symbol_map_unref()`
/// is called. After unref, the pointer becomes invalid and must not be used.
#[repr(C)]
pub struct ChafaSymbolMap(c_void);

/// Opaque pointer to a ChafaCanvasConfig
///
/// Configuration object that specifies how a canvas should be created.
/// Holds settings like geometry, symbol map, color space, and dithering mode.
/// Must be freed with `chafa_canvas_config_unref()` when no longer needed.
///
/// # Safety
///
/// The pointer returned by `chafa_canvas_config_new()` is valid until `chafa_canvas_config_unref()`
/// is called. The config must outlive any canvas created from it only if the canvas references it.
#[repr(C)]
pub struct ChafaCanvasConfig(c_void);

/// Opaque pointer to a ChafaCanvas
///
/// The main canvas object for drawing and rendering pixels to terminal art.
/// Created with a canvas config, supports drawing pixels and generating output.
/// Must be freed with `chafa_canvas_unref()` when no longer needed.
///
/// # Safety
///
/// The pointer returned by `chafa_canvas_new()` is valid until `chafa_canvas_unref()`
/// is called. The canvas should not be used after unref.
#[repr(C)]
pub struct ChafaCanvas(c_void);

/// GString structure (from GLib)

/// Opaque pointer to a ChafaTermInfo (terminal capabilities).
///
/// The terminfo object is used by `chafa_canvas_print` to decide what control
/// sequences to emit (background vs foreground colours, sixel/kitty/etc).  We
/// don't yet use it from Rust, but binding it here so the API is available
/// later.
#[repr(C)]
pub struct ChafaTermInfo(c_void);

///
/// A string structure returned by libchafa functions like `chafa_canvas_build_ansi()`.
/// The `str` field points to a null-terminated C string.
/// This is a simple definition sufficient for our FFI needs.
///
/// # Safety
///
/// The `str` pointer must be freed using appropriate GLib functions (usually `g_string_free()`).
/// The `str` pointer may be null for empty strings.
#[repr(C)]
pub struct GString {
    pub str: *mut c_char,
    pub len: usize,
    pub allocated_len: usize,
}

// ============================================================================
// Enums
// ============================================================================

/// Symbol tags for selecting symbol sets
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum ChafaSymbolTag {
    /// All symbols
    All = 0,
    /// Solid blocks
    Solid = 1,
    /// Block elements
    Block = 2,
    /// Box drawing characters
    Box = 3,
    /// Braille patterns
    Braille = 4,
    /// ASCII characters
    Ascii = 5,
    /// Narrow characters
    Narrow = 6,
}

impl ChafaSymbolTag {
}

/// Color space types
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum ChafaColorSpace {
    /// RGB color space
    Rgb = 0,
    /// RGBA color space
    TypeRgba8 = 1,
}

impl ChafaColorSpace {
}

/// Dither modes
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum ChafaDitherMode {
    /// No dithering
    None = 0,
    /// Ordered dithering
    Ordered = 1,
    /// Diffusion dithering
    Diffusion = 2,
}

impl ChafaDitherMode {
}

/// Pixel types
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum ChafaPixelType {
    /// RGBA8 unassociated
    Rgba8Unassociated = 0,
    /// RGBA8
    Rgba8 = 1,
}

impl ChafaPixelType {
}

// ============================================================================
// Function Declarations
// ============================================================================

extern "C" {
    /// Create a new symbol map
    ///
    /// Allocates and returns a new, empty symbol map. The caller must free it
    /// with `chafa_symbol_map_unref()`.
    ///
    /// # Returns
    ///
    /// A pointer to a new `ChafaSymbolMap`, or null if allocation fails.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It may return a null pointer
    /// - The caller is responsible for freeing the returned pointer
    pub fn chafa_symbol_map_new() -> *mut ChafaSymbolMap;

    /// Add symbols to the map by tag
    ///
    /// Adds a set of symbols to the symbol map based on the provided tag.
    /// Multiple tags can be combined using bitwise operations.
    ///
    /// # Arguments
    ///
    /// * `symbol_map` - Pointer to a symbol map created with `chafa_symbol_map_new()`
    /// * `tag` - Symbol tag (or bitwise OR of multiple tags) to add
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `symbol_map` is a valid pointer returned by `chafa_symbol_map_new()`
    /// - `symbol_map` is not used after being freed
    /// - `tag` is a valid ChafaSymbolTag or combination of tags
    pub fn chafa_symbol_map_add_by_tags(symbol_map: *mut ChafaSymbolMap, tag: u32);

    /// Decrease the reference count and free a symbol map if count reaches zero
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `symbol_map` is a valid pointer or null
    /// - `symbol_map` is not dereferenced after this call
    /// - This is the last reference to the symbol map
    pub fn chafa_symbol_map_unref(symbol_map: *mut ChafaSymbolMap);

    /// Create a new canvas configuration
    ///
    /// Allocates and returns a new canvas config with default settings.
    /// The caller must free it with `chafa_canvas_config_unref()`.
    ///
    /// # Returns
    ///
    /// A pointer to a new `ChafaCanvasConfig`, or null if allocation fails.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It may return a null pointer
    /// - The caller is responsible for freeing the returned pointer
    pub fn chafa_canvas_config_new() -> *mut ChafaCanvasConfig;

    /// Set the canvas geometry (width and height in characters)
    ///
    /// Configures the output dimensions of the canvas in terminal character cells.
    ///
    /// # Arguments
    ///
    /// * `config` - Pointer to a canvas config created with `chafa_canvas_config_new()`
    /// * `width` - Canvas width in terminal columns
    /// * `height` - Canvas height in terminal rows
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `config` is a valid pointer returned by `chafa_canvas_config_new()`
    /// - `width` and `height` are reasonable values (> 0)
    pub fn chafa_canvas_config_set_geometry(config: *mut ChafaCanvasConfig, width: c_uint, height: c_uint);

    /// Set the symbol map for the canvas
    ///
    /// Assigns a symbol map to the canvas config. The symbol map is referenced
    /// but not copied, so it must remain valid while the config and any canvas
    /// created from it are in use.
    ///
    /// # Arguments
    ///
    /// * `config` - Pointer to a canvas config
    /// * `symbol_map` - Pointer to a symbol map (or null to use a default set)
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `config` is a valid canvas config pointer
    /// - `symbol_map` is either null or a valid symbol map pointer
    /// - The symbol map outlives the config and any canvas created from it
    pub fn chafa_canvas_config_set_symbol_map(config: *mut ChafaCanvasConfig, symbol_map: *mut ChafaSymbolMap);

    /// Set the color space
    ///
    /// Configures how colors should be interpreted in the input pixel data.
    ///
    /// # Arguments
    ///
    /// * `config` - Pointer to a canvas config
    /// * `color_space` - Color space type (e.g., CHAFA_COLOR_TYPERGBA8)
    ///
    /// # Safety
    ///
    /// The caller must ensure `config` is a valid canvas config pointer.
    pub fn chafa_canvas_config_set_color_space(config: *mut ChafaCanvasConfig, color_space: u32);

    /// Set the dither mode
    ///
    /// Configures dithering to improve image quality when reducing colors.
    ///
    /// # Arguments
    ///
    /// * `config` - Pointer to a canvas config
    /// * `mode` - Dither mode (none, ordered, or diffusion)
    ///
    /// # Safety
    ///
    /// The caller must ensure `config` is a valid canvas config pointer.
    pub fn chafa_canvas_config_set_dither_mode(config: *mut ChafaCanvasConfig, mode: u32);

    /// Free a canvas configuration
    ///
    /// Decreases the reference count and frees the config if count reaches zero.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `config` is a valid pointer or null
    /// - `config` is not dereferenced after this call
    pub fn chafa_canvas_config_unref(config: *mut ChafaCanvasConfig);

    /// Create a new canvas with the given config
    ///
    /// Allocates a canvas configured with the provided settings.
    /// The caller must free it with `chafa_canvas_unref()`.
    ///
    /// # Arguments
    ///
    /// * `config` - Pointer to a canvas config created with `chafa_canvas_config_new()`
    ///
    /// # Returns
    ///
    /// A pointer to a new `ChafaCanvas`, or null if creation fails.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It may return a null pointer
    /// - The caller is responsible for freeing the returned pointer
    pub fn chafa_canvas_new(config: *const ChafaCanvasConfig) -> *mut ChafaCanvas;

    /// Draw all pixels to the canvas
    ///
    /// Loads pixel data from memory and converts it to terminal symbols on the canvas.
    /// The pixel data format is specified by the `pixel_type` parameter.
    ///
    /// # Arguments
    ///
    /// * `canvas` - Pointer to a canvas created with `chafa_canvas_new()`
    /// * `pixel_type` - Format of the pixel data (e.g., CHAFA_PIXEL_RGBA8)
    /// * `pixels` - Pointer to raw pixel data
    /// * `width` - Width of the image in pixels
    /// * `height` - Height of the image in pixels
    /// * `rowstride` - Bytes per row (typically `width * bytes_per_pixel`)
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `canvas` is a valid pointer returned by `chafa_canvas_new()`
    /// - `pixels` points to valid memory containing at least `height * rowstride` bytes
    /// - `width`, `height`, and `rowstride` accurately describe the pixel data
    /// - `pixel_type` matches the actual format of the pixel data
    /// - The pixel data remains valid for the duration of the call
    pub fn chafa_canvas_draw_all_pixels(
        canvas: *mut ChafaCanvas,
        pixel_type: u32,
        pixels: *const c_void,
        width: c_uint,
        height: c_uint,
        rowstride: c_uint,
    );

    /// Generate ANSI output from the canvas
    ///
    /// Converts the canvas content to an ANSI-formatted string for terminal display.
    /// Returns a GString that the caller must free.
    ///
    /// # Arguments
    ///
    /// * `canvas` - Pointer to a canvas created with `chafa_canvas_new()`
    ///
    /// # Returns
    ///
    /// A pointer to a GString containing ANSI escape codes, or null on failure.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `canvas` is a valid pointer returned by `chafa_canvas_new()`
    /// - The returned GString pointer is freed (either by extracting the string or using `g_string_free()`)
    pub fn chafa_canvas_build_ansi(canvas: *mut ChafaCanvas) -> *mut GString;

    // terminfo support and higher‑level printing (used by the CLI, not yet by
    // mdterm):
    /// Opaque pointer to a ChafaTermInfo (see chafa-term-info.h)
    pub fn chafa_canvas_print(canvas: *mut ChafaCanvas, term_info: *mut ChafaTermInfo) -> *mut GString;

    /// Create a new (empty) terminfo object
    pub fn chafa_term_info_new() -> *mut ChafaTermInfo;
    pub fn chafa_term_info_ref(term_info: *mut ChafaTermInfo);
    pub fn chafa_term_info_unref(term_info: *mut ChafaTermInfo);

    /// Free a canvas
    ///
    /// Decreases the reference count and frees the canvas if count reaches zero.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `canvas` is a valid pointer or null
    /// - `canvas` is not dereferenced after this call
    pub fn chafa_canvas_unref(canvas: *mut ChafaCanvas);

    /// Free a GString
    ///
    /// Frees a GString returned by libchafa functions.
    ///
    /// # Arguments
    ///
    /// * `gstring` - Pointer to a GString returned by libchafa
    /// * `free_segment` - Whether to free the string data (usually 1)
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    /// - `gstring` is a valid pointer or null
    /// - `gstring` has not already been freed
    pub fn g_string_free(gstring: *mut GString, free_segment: c_int);
}

// ============================================================================
// Constants
// ============================================================================

/// All symbols
pub const CHAFA_SYMBOL_TAG_ALL: ChafaSymbolTag = ChafaSymbolTag::All;
/// Solid blocks
pub const CHAFA_SYMBOL_TAG_SOLID: ChafaSymbolTag = ChafaSymbolTag::Solid;
/// Block elements
pub const CHAFA_SYMBOL_TAG_BLOCK: ChafaSymbolTag = ChafaSymbolTag::Block;
/// Box drawing characters
pub const CHAFA_SYMBOL_TAG_BOX: ChafaSymbolTag = ChafaSymbolTag::Box;
/// Braille patterns
pub const CHAFA_SYMBOL_TAG_BRAILLE: ChafaSymbolTag = ChafaSymbolTag::Braille;
/// ASCII characters
pub const CHAFA_SYMBOL_TAG_ASCII: ChafaSymbolTag = ChafaSymbolTag::Ascii;
/// Narrow characters
pub const CHAFA_SYMBOL_TAG_NARROW: ChafaSymbolTag = ChafaSymbolTag::Narrow;

/// RGB color space
pub const CHAFA_COLOR_SPACE_RGB: ChafaColorSpace = ChafaColorSpace::Rgb;
/// RGBA color space
pub const CHAFA_COLOR_TYPERGBA8: ChafaColorSpace = ChafaColorSpace::TypeRgba8;

/// No dithering
pub const CHAFA_DITHER_MODE_NONE: ChafaDitherMode = ChafaDitherMode::None;
/// Ordered dithering
pub const CHAFA_DITHER_MODE_ORDERED: ChafaDitherMode = ChafaDitherMode::Ordered;
/// Diffusion dithering
pub const CHAFA_DITHER_MODE_DIFFUSION: ChafaDitherMode = ChafaDitherMode::Diffusion;

/// RGBA8 unassociated
pub const CHAFA_PIXEL_RGBA8_UNASSOCIATED: ChafaPixelType = ChafaPixelType::Rgba8Unassociated;
/// RGBA8
pub const CHAFA_PIXEL_RGBA8: ChafaPixelType = ChafaPixelType::Rgba8;

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_values() {
        // Verify that enum values are correct
        assert_eq!(ChafaSymbolTag::CHAFA_SYMBOL_TAG_ALL as i32, 0);
        assert_eq!(ChafaColorSpace::CHAFA_COLOR_TYPERGBA8 as i32, 1);
        assert_eq!(ChafaDitherMode::CHAFA_DITHER_MODE_DIFFUSION as i32, 2);
        assert_eq!(ChafaPixelType::CHAFA_PIXEL_RGBA8 as i32, 1);
    }
}
