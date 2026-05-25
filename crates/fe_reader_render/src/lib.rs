//! Rendering contracts for Fe Reader.
//!
//! Production renderers live in adapter crates. The core renderer API is tile-based so desktop,
//! mobile and web viewers can share scheduling logic.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_pdf_model::{PageIndex, PdfRect};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

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

/// Hardware acceleration preference for render planning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccelerationPreference {
    /// Let the adapter choose a safe default.
    Auto,
    /// Force deterministic CPU rendering.
    CpuOnly,
    /// Permit GPU compositing after policy and capability checks.
    GpuCompositing,
    /// Experimental vector GPU path. Frontier lane only.
    GpuVectorExperimental,
}

/// Request for one render tile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderTileRequest {
    /// Stable document reference for cache isolation.
    pub document_ref: String,
    /// Page index.
    pub page_index: PageIndex,
    /// Tile rectangle in page coordinates.
    pub tile_rect: PdfRect,
    /// Scale multiplier.
    pub scale: f32,
    /// Clockwise page rotation in degrees.
    pub rotation_degrees: i32,
    /// Colour mode.
    pub color_mode: ColorMode,
    /// Requested acceleration policy.
    pub acceleration: AccelerationPreference,
}

impl RenderTileRequest {
    /// Creates a deterministic cache key for a tile request.
    #[must_use]
    pub fn cache_key(&self) -> String {
        format!(
            "{}:{}:{:.3}:{:.3}:{:.3}:{:.3}:{:.4}:{}:{:?}:{:?}",
            self.document_ref,
            self.page_index.0,
            self.tile_rect.x,
            self.tile_rect.y,
            self.tile_rect.width,
            self.tile_rect.height,
            self.scale,
            self.rotation_degrees,
            self.color_mode,
            self.acceleration
        )
    }

    /// Validates tile geometry and scale before dispatching to a renderer.
    ///
    /// # Errors
    ///
    /// Returns a render error if the tile request cannot be fulfilled safely.
    pub fn validate(&self) -> Result<(), RenderError> {
        if self.document_ref.trim().is_empty() {
            return Err(RenderError::invalid_request(
                "document_ref must not be empty",
            ));
        }
        if !self.tile_rect.is_non_empty() {
            return Err(RenderError::invalid_request("tile_rect must be non-empty"));
        }
        if !self.scale.is_finite() || self.scale <= 0.0 {
            return Err(RenderError::invalid_request(
                "scale must be positive and finite",
            ));
        }
        if self.rotation_degrees.rem_euclid(90) != 0 {
            return Err(RenderError::invalid_request(
                "rotation_degrees must be a multiple of 90",
            ));
        }
        Ok(())
    }
}

/// Rendered tile payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedTile {
    /// Deterministic cache key for this tile.
    pub cache_key: String,
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

impl RenderError {
    /// Creates an invalid request error.
    #[must_use]
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Creates an unavailable backend error.
    #[must_use]
    pub fn unavailable(backend_name: &str) -> Self {
        Self {
            message: format!("{backend_name} backend is unavailable in this build"),
        }
    }
}

/// Abstract render backend.
pub trait RenderBackend: Send + Sync {
    /// Stable backend name for diagnostics.
    fn backend_name(&self) -> &'static str;

    /// Renders a tile.
    fn render_tile(&self, request: RenderTileRequest) -> Result<RenderedTile, RenderError>;

    /// Renders a deterministic page thumbnail.
    ///
    /// # Errors
    ///
    /// Returns a render error if the backend cannot render the thumbnail.
    fn render_page_thumbnail(
        &self,
        document_ref: &str,
        page_index: PageIndex,
        max_px: u32,
    ) -> Result<RenderedTile, RenderError> {
        let side = max_px.max(1) as f32;
        self.render_tile(RenderTileRequest {
            document_ref: document_ref.to_string(),
            page_index,
            tile_rect: PdfRect::new(0.0, 0.0, side, side),
            scale: 1.0,
            rotation_degrees: 0,
            color_mode: ColorMode::Normal,
            acceleration: AccelerationPreference::CpuOnly,
        })
    }
}

/// A deterministic placeholder renderer used by Wave 0 tests.
#[derive(Debug, Default, Clone)]
pub struct NullRenderBackend;

impl RenderBackend for NullRenderBackend {
    fn backend_name(&self) -> &'static str {
        "null"
    }

    fn render_tile(&self, request: RenderTileRequest) -> Result<RenderedTile, RenderError> {
        request.validate()?;
        let width = (request.tile_rect.width * request.scale).max(1.0) as u32;
        let height = (request.tile_rect.height * request.scale).max(1.0) as u32;
        Ok(RenderedTile {
            cache_key: request.cache_key(),
            width,
            height,
            pixel_format: PixelFormat::Rgba8,
            bytes: vec![0; width as usize * height as usize * 4],
        })
    }
}

/// In-memory tile cache used by adapter smoke tests and UI scheduling prototypes.
#[derive(Debug, Clone)]
pub struct RenderTileCache {
    max_entries: usize,
    order: VecDeque<String>,
    tiles: HashMap<String, RenderedTile>,
}

impl RenderTileCache {
    /// Creates a tile cache with a fixed entry count.
    #[must_use]
    pub fn new(max_entries: usize) -> Self {
        Self {
            max_entries: max_entries.max(1),
            order: VecDeque::new(),
            tiles: HashMap::new(),
        }
    }

    /// Inserts a rendered tile by its cache key.
    pub fn insert(&mut self, tile: RenderedTile) {
        if !self.tiles.contains_key(&tile.cache_key) {
            self.order.push_back(tile.cache_key.clone());
        }
        self.tiles.insert(tile.cache_key.clone(), tile);
        while self.tiles.len() > self.max_entries {
            if let Some(oldest) = self.order.pop_front() {
                self.tiles.remove(&oldest);
            }
        }
    }

    /// Returns a cached tile.
    #[must_use]
    pub fn get(&self, cache_key: &str) -> Option<&RenderedTile> {
        self.tiles.get(cache_key)
    }

    /// Returns the number of cached tiles.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    /// Returns true when no tiles are cached.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
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
                document_ref: "doc".to_string(),
                page_index: PageIndex(0),
                tile_rect: PdfRect::new(0.0, 0.0, 10.0, 10.0),
                scale: 2.0,
                rotation_degrees: 0,
                color_mode: ColorMode::Normal,
                acceleration: AccelerationPreference::CpuOnly,
            })
            .unwrap();
        assert_eq!(backend.backend_name(), "null");
        assert_eq!(tile.width, 20);
        assert_eq!(tile.height, 20);
        assert_eq!(tile.bytes.len(), 20 * 20 * 4);
        assert!(tile.cache_key.contains("doc:0"));
    }

    #[test]
    fn request_validation_rejects_bad_scale() {
        let request = RenderTileRequest {
            document_ref: "doc".to_string(),
            page_index: PageIndex(0),
            tile_rect: PdfRect::new(0.0, 0.0, 10.0, 10.0),
            scale: 0.0,
            rotation_degrees: 0,
            color_mode: ColorMode::Normal,
            acceleration: AccelerationPreference::CpuOnly,
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn tile_cache_evicts_oldest_entry() {
        let backend = NullRenderBackend;
        let mut cache = RenderTileCache::new(1);
        let first = backend
            .render_tile(RenderTileRequest {
                document_ref: "doc".to_string(),
                page_index: PageIndex(0),
                tile_rect: PdfRect::new(0.0, 0.0, 10.0, 10.0),
                scale: 1.0,
                rotation_degrees: 0,
                color_mode: ColorMode::Normal,
                acceleration: AccelerationPreference::CpuOnly,
            })
            .unwrap();
        let first_key = first.cache_key.clone();
        cache.insert(first);
        let second = backend
            .render_tile(RenderTileRequest {
                document_ref: "doc".to_string(),
                page_index: PageIndex(1),
                tile_rect: PdfRect::new(0.0, 0.0, 10.0, 10.0),
                scale: 1.0,
                rotation_degrees: 0,
                color_mode: ColorMode::Normal,
                acceleration: AccelerationPreference::CpuOnly,
            })
            .unwrap();
        let second_key = second.cache_key.clone();
        cache.insert(second);
        assert_eq!(cache.len(), 1);
        assert!(cache.get(&first_key).is_none());
        assert!(cache.get(&second_key).is_some());
    }
}
