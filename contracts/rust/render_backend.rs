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
    pub document_id: String,
    pub page_index: u32,
    pub tile_x: u32,
    pub tile_y: u32,
    pub tile_width: u32,
    pub tile_height: u32,
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
    fn render_page_thumbnail(&self, document_id: &str, page_index: u32, max_px: u32) -> anyhow::Result<RenderedTile>;
}
