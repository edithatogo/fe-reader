//! GPU render-compositor policy scaffolding.
//!
//! This crate describes GPU capability decisions without enabling a GPU renderer by default.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use fe_reader_render::AccelerationPreference;
use serde::{Deserialize, Serialize};

/// Crate name exposed for smoke tests and workspace health checks.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// Crate semantic version exposed for compatibility smoke tests.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GPU compositor option exposed to higher-level adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GpuCompositorOption {
    /// GPU compositor disabled.
    Disabled,
    /// GPU compositor can be used for compositing already-rendered tiles.
    CompositingOnly,
    /// Experimental vector rendering path.
    VectorExperimental,
}

/// Hardware acceleration capability flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareAccelerationFlags {
    /// Whether GPU compositing is supported by this build.
    pub gpu_compositing_supported: bool,
    /// Whether experimental vector rendering is supported by this build.
    pub gpu_vector_supported: bool,
    /// Whether the user or policy has disabled all hardware acceleration.
    pub disabled_by_policy: bool,
}

impl HardwareAccelerationFlags {
    /// Returns the conservative Wave 0 flags.
    #[must_use]
    pub fn wave0_default() -> Self {
        Self {
            gpu_compositing_supported: false,
            gpu_vector_supported: false,
            disabled_by_policy: true,
        }
    }

    /// Resolves a render acceleration request to a compositor option.
    #[must_use]
    pub fn resolve(&self, preference: AccelerationPreference) -> GpuCompositorOption {
        if self.disabled_by_policy {
            return GpuCompositorOption::Disabled;
        }
        match preference {
            AccelerationPreference::Auto | AccelerationPreference::CpuOnly => {
                GpuCompositorOption::Disabled
            }
            AccelerationPreference::GpuCompositing if self.gpu_compositing_supported => {
                GpuCompositorOption::CompositingOnly
            }
            AccelerationPreference::GpuVectorExperimental if self.gpu_vector_supported => {
                GpuCompositorOption::VectorExperimental
            }
            AccelerationPreference::GpuCompositing
            | AccelerationPreference::GpuVectorExperimental => GpuCompositorOption::Disabled,
        }
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
    fn wave0_gpu_policy_disables_acceleration() {
        let flags = HardwareAccelerationFlags::wave0_default();
        assert_eq!(
            flags.resolve(AccelerationPreference::GpuCompositing),
            GpuCompositorOption::Disabled
        );
        assert_eq!(
            flags.resolve(AccelerationPreference::GpuVectorExperimental),
            GpuCompositorOption::Disabled
        );
    }

    #[test]
    fn capability_flags_allow_compositing_when_policy_allows_it() {
        let flags = HardwareAccelerationFlags {
            gpu_compositing_supported: true,
            gpu_vector_supported: false,
            disabled_by_policy: false,
        };
        assert_eq!(
            flags.resolve(AccelerationPreference::GpuCompositing),
            GpuCompositorOption::CompositingOnly
        );
    }
}
