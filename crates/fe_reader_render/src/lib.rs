//! Rendering contracts for Fe Reader.
//!
//! Production renderers live in adapter crates. The core renderer API is tile-based so desktop,
//! mobile and web viewers can share scheduling logic.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_pdf_model::{PageIndex, PdfRect};
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Pixel format for rendered buffers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PixelFormat {
    /// 8-bit RGBA.
    Rgba8,
    /// 8-bit BGRA.
    Bgra8,
    /// 8-bit grayscale.
    Gray8,
}

/// Render colour mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorMode {
    /// Normal colour rendering.
    Normal,
    /// Grayscale rendering.
    Grayscale,
    /// High contrast rendering.
    HighContrast,
    /// Inverted colours for reading comfort.
    Inverted,
}

/// Request for one render tile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderTileRequest {
    /// Page index.
    pub page_index: PageIndex,
    /// Tile rectangle in page coordinates.
    pub tile_rect: PdfRect,
    /// Scale multiplier.
    pub scale: f32,
    /// Colour mode.
    pub color_mode: ColorMode,
}

/// Rendered tile payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedTile {
    /// Pixel width.
    pub width: u32,
    /// Pixel height.
    pub height: u32,
    /// Pixel format.
    pub pixel_format: PixelFormat,
    /// Pixel bytes.
    pub bytes: Vec<u8>,
}

/// Renderer error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[error("render error: {message}")]
pub struct RenderError {
    /// Error message.
    pub message: String,
}

/// Abstract render backend.
pub trait RenderBackend: Send + Sync {
    /// Renders a tile.
    fn render_tile(&self, request: RenderTileRequest) -> Result<RenderedTile, RenderError>;
}

/// A deterministic placeholder renderer used by Wave 0 tests.
#[derive(Debug, Default, Clone)]
pub struct NullRenderBackend;

impl RenderBackend for NullRenderBackend {
    fn render_tile(&self, request: RenderTileRequest) -> Result<RenderedTile, RenderError> {
        let width = (request.tile_rect.width * request.scale).max(1.0) as u32;
        let height = (request.tile_rect.height * request.scale).max(1.0) as u32;
        Ok(RenderedTile {
            width,
            height,
            pixel_format: PixelFormat::Rgba8,
            bytes: vec![0; width as usize * height as usize * 4],
        })
    }
}

/// Returns a stable identity string for diagnostics.
#[must_use]
pub fn crate_identity() -> String {
    format!("{}@{}", CRATE_NAME, CRATE_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_renderer_returns_sized_tile() {
        let backend = NullRenderBackend;
        let tile = backend
            .render_tile(RenderTileRequest {
                page_index: PageIndex(0),
                tile_rect: PdfRect::new(0.0, 0.0, 10.0, 10.0),
                scale: 2.0,
                color_mode: ColorMode::Normal,
            })
            .unwrap();
        assert_eq!(tile.width, 20);
        assert_eq!(tile.height, 20);
        assert_eq!(tile.bytes.len(), 20 * 20 * 4);
    }
}
