//! PDFium rendering adapter boundary for Fe Reader.
//!
//! Wave 0 exposes the adapter shape and unavailable-runtime behavior without binding to a local
//! PDFium dynamic library at startup.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_pdf_model::PageIndex;
use fe_reader_render::{RenderBackend, RenderError, RenderTileRequest, RenderedTile};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// PDFium-backed render adapter.
#[derive(Debug, Clone, Default)]
pub struct PdfiumRenderBackend {
    runtime_available: bool,
}

impl PdfiumRenderBackend {
    /// Creates a PDFium adapter in unavailable-runtime mode.
    ///
    /// Real library discovery is intentionally deferred until the platform policy layer can
    /// describe library search paths and signing requirements.
    #[must_use]
    pub fn unavailable() -> Self {
        Self {
            runtime_available: false,
        }
    }

    /// Returns whether a PDFium runtime is available.
    #[must_use]
    pub fn runtime_available(&self) -> bool {
        self.runtime_available
    }
}

impl RenderBackend for PdfiumRenderBackend {
    fn backend_name(&self) -> &'static str {
        "pdfium"
    }

    fn render_tile(&self, request: RenderTileRequest) -> Result<RenderedTile, RenderError> {
        request.validate()?;
        Err(RenderError::unavailable(self.backend_name()))
    }

    fn render_page_thumbnail(
        &self,
        document_ref: &str,
        page_index: PageIndex,
        max_px: u32,
    ) -> Result<RenderedTile, RenderError> {
        let _ = (document_ref, page_index, max_px);
        Err(RenderError::unavailable(self.backend_name()))
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
    fn identity_contains_name() {
        assert!(crate_identity().contains(CRATE_NAME));
    }

    #[test]
    fn pdfium_adapter_reports_unavailable_runtime() {
        let backend = PdfiumRenderBackend::unavailable();
        assert_eq!(backend.backend_name(), "pdfium");
        assert!(!backend.runtime_available());
        let err = backend
            .render_tile(RenderTileRequest {
                document_ref: "doc".to_string(),
                page_index: PageIndex(0),
                tile_rect: fe_reader_pdf_model::PdfRect::new(0.0, 0.0, 10.0, 10.0),
                scale: 1.0,
                rotation_degrees: 0,
                color_mode: fe_reader_render::ColorMode::Normal,
                acceleration: fe_reader_render::AccelerationPreference::CpuOnly,
            })
            .unwrap_err();
        assert!(err.message.contains("pdfium backend is unavailable"));
    }
}
