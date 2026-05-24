//! E2E scenario contract for app-shell tests.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2eScenario {
    pub scenario_id: String,
    pub platforms: Vec<String>,
    pub fixture: String,
    pub steps: Vec<E2eStep>,
    pub expected: Vec<E2eExpectation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum E2eStep {
    OpenFile { path: String },
    Click { role: String, name: String },
    PressKey { key: String },
    TypeText { text: String },
    WaitForJob { job_kind: String },
    Screenshot { label: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum E2eExpectation {
    VisibleText { text: String },
    FocusedControl { role: String, name: String },
    NoAccessibilityErrors,
    ScreenshotMatches { label: String, baseline: String },
}
