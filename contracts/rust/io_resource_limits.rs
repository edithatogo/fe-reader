//! Contract sketch for zero-copy, memory-map and resource-limit policy.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfResourceLimits {
    pub max_objects: u32,
    pub max_decoded_stream_bytes: u64,
    pub max_recursion_depth: u32,
    pub max_text_spans_per_page: u32,
    pub max_render_tile_pixels: u32,
    pub max_repair_attempts: u32,
    pub max_operation_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputAccessMode {
    OwnedBytes,
    BufferedFile,
    MemoryMappedReadOnly,
    PlatformDocumentProvider,
    WebBlob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputAccessReport {
    pub mode: InputAccessMode,
    pub file_size_bytes: Option<u64>,
    pub resource_limits: PdfResourceLimits,
    pub mmap_used: bool,
    pub fallback_reason: Option<String>,
}
