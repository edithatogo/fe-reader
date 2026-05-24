//! PDF optimisation contracts.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimisationLevel {
    InspectOnly,
    SafeRewrite,
    SizeOptimise,
    Aggressive,
    WebDeliveryLinearized,
}

#[derive(Debug, Clone)]
pub struct PdfOptimisationPlan {
    pub plan_id: String,
    pub document_sha256: String,
    pub level: OptimisationLevel,
    pub preserve_signatures: bool,
    pub preserve_metadata: bool,
    pub allow_image_recompression: bool,
    pub max_visual_delta: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct PdfOptimisationReceipt {
    pub plan_id: String,
    pub input_sha256: String,
    pub output_sha256: String,
    pub bytes_before: u64,
    pub bytes_after: u64,
    pub linearized: bool,
    pub signatures_preserved: bool,
    pub warnings: Vec<String>,
}

pub trait PdfOptimiser: Send + Sync {
    fn plan(&self, document_id: &str, level: OptimisationLevel) -> Result<PdfOptimisationPlan, String>;
    fn apply(&self, plan: PdfOptimisationPlan, out_path: &str) -> Result<PdfOptimisationReceipt, String>;
}
