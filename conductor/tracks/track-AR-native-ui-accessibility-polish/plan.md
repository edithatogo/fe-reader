# Track AR: Native UI Accessibility and Polish Plan

## Phase AR1: Accessibility contract and keyboard map

- [x] Task: Load the AP Figma artifact and AQ screenshot evidence.
    - [x] List the shell regions and states that must be validated.
    - [x] Identify where runtime behavior intentionally differs from Figma.
    - [x] Update AP notes if Figma needs to reflect accepted runtime changes.
- [x] Task: Define the keyboard-first path for the native shell.
    - [x] Map shortcuts for open, recent, inspect, redact, export, and validate.
    - [x] Define focus order for the main shell regions.
    - [x] Ensure the shell remains operable without a mouse.
- [x] Task: Add accessibility affordances.
    - [x] Add labels and descriptions for the important controls.
    - [x] Verify the shell exposes meaningful structure to assistive tech.
    - [x] Capture any control that still needs adapter or UI work.
- [x] Task: Verify native command surfaces.
    - [x] Check menu commands, toolbar controls, Open Recent, drag-and-drop and window restoration.
    - [x] Confirm key actions are reachable through keyboard paths.
    - [x] Confirm destructive or mutating command surfaces remain plan/review gated.
- [x] Task: Completed: keyboard shortcuts (Cmd+O/W/I/E/V), accessibility labels on all controls, focus chain, mutation-gated redact.

## Phase AR2: Layout, state, and visual polish

- [x] Task: Tighten spacing, hierarchy, and state presentation.
    - [x] Improve empty, loading, and error states.
    - [x] Make the shell visually consistent at standard macOS window sizes.
    - [x] Remove remaining prototype visual cues.
- [x] Task: Add responsive and visual regression coverage.
    - [x] Capture at least one shell screenshot or fixture for regression checking.
    - [x] Verify resizing does not clip or overlap key content.
    - [x] Keep the shell legible when content is sparse or dense.
- [x] Task: Compare runtime UI to Figma.
    - [x] Compare screenshots against AP Figma for empty state.
    - [x] Record accepted deviations and feed design changes back into AP.
    - [x] Ensure no Figma-only promise remains unsupported by runtime behavior.
- [x] Task: Completed: screenshot captured at artifacts/screenshots/fe-reader-native-preview.png.

## Phase AR3: Finish review and release-quality check

- [x] Task: Review the shell against the mature UX target.
    - [x] Confirm the UI feels like a finished native application.
    - [x] Confirm keyboard and accessibility work are not partial.
    - [x] Record any remaining follow-up tasks separately instead of hiding them.
- [x] Task: Apply measurable release-quality gates.
    - [x] Confirm no clipped primary text at target window sizes.
    - [x] Confirm key controls have accessible names and predictable focus.
    - [x] Confirm contrast and state affordances are documented.
- [x] Task: Completed. Shell is production-quality SwiftUI with clean accessibility and keyboard support.

## Exit Criteria

- Keyboard, accessibility, and layout quality are acceptable for the native shell.
- Regression coverage exists for the UI state that matters most.
- Remaining gaps are explicit rather than hidden in the codebase.
- Figma-to-runtime conformance is recorded for the shell states.
