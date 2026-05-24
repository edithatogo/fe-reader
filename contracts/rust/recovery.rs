//! Contract sketch for safe-open, repair planning and recovery reports.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpenMode { NormalOpen, SafeOpen, DiagnosticOpen, RepairPlanOnly }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryPlan {
    pub plan_id: String,
    pub document_sha256: String,
    pub mode: OpenMode,
    pub actions: Vec<RecoveryAction>,
    pub signature_risk: SignatureRisk,
    pub write_policy: RecoveryWritePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryAction {
    RebuildXref,
    RecoverTrailer,
    RecoverPageTree,
    RepairStreamLength { object_id: String },
    RegenerateAnnotationAppearance { object_id: String },
    DropOrphanedObject { object_id: String },
    NormaliseXmp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureRisk { NoneDetected, MayInvalidate, WillInvalidate }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryWritePolicy { PlanOnly, SaveCopyOnly, AllowOverwriteWithExplicitApproval }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryReceipt {
    pub plan_id: String,
    pub original_sha256: String,
    pub output_sha256: String,
    pub applied_actions: Vec<RecoveryAction>,
    pub removed_object_count: u32,
    pub warnings: Vec<String>,
}
