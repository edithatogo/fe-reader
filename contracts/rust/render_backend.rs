//! Rendering backend contract.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccelerationPreference {
    Auto,
    CpuOnly,
    GpuCompositing,
    GpuVectorExperimental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTileRequest {
    pub document_ref: String,
    pub page_index: u32,
    pub tile_x: f32,
    pub tile_y: f32,
    pub tile_width: f32,
    pub tile_height: f32,
    pub scale: f32,
    pub rotation_degrees: i32,
    pub color_mode: String,
    pub acceleration: AccelerationPreference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedTile {
    pub width: u32,
    pub height: u32,
    pub pixel_format: String,
    pub bytes: Vec<u8>,
    pub cache_key: String,
}

pub trait RenderBackend: Send + Sync {
    fn backend_name(&self) -> &'static str;
    fn render_tile(&self, request: RenderTileRequest) -> anyhow::Result<RenderedTile>;
    fn render_page_thumbnail(
        &self,
        document_ref: &str,
        page_index: u32,
        max_px: u32,
    ) -> anyhow::Result<RenderedTile>;
}
