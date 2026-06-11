# Track AP: Native UI Wireframe Discovery Plan

## Phase AP1: UX discovery and shell framing

- [x] Task: Review the current native shell, docs, and existing track boundaries.
    - [x] Identify which current layout pieces are placeholder-only.
    - [x] Capture the shell surfaces that must exist in the finished UI.
    - [x] Record non-goals so the track stays focused on native UX discovery.
- [x] Task: Write the native shell UX brief.
    - [x] Define primary users and top workflows.
    - [x] Define first-run, empty, loading, open-document, error, and read-only states.
    - [x] Define command hierarchy and explicit anti-goals.
- [x] Task: Build the first-pass shell wireframe in Figma.
    - [x] Model the title bar, sidebar, document canvas, and inspector.
    - [x] Model empty, loading, open-document, and error states.
    - [x] Keep the wireframe low-fidelity and implementation-guiding.
- [x] Task: Extract implementation guidance from Figma.
    - [x] Name each frame/state that AQ and AR must reference.
    - [x] Define spacing, typography, color roles, icon rules, density, and component anatomy.
    - [x] Record expected native macOS controls, menus, toolbars, and split-view behavior.
- [x] Task: Write the interaction model for the shell.
    - [x] Define primary navigation and command paths.
    - [x] Define keyboard entry points for the main actions.
    - [x] Define resize and focus expectations for the shell.
- [x] Task: Documented via UX brief, design tokens, interaction model, and architecture docs.
    - [x] Phase gate approved: build_and_run.sh compiles and launches successfully.

## Phase AP2: Handoff and UX contract freeze

- [x] Task: Review the wireframe against the product boundary and existing tracks.
    - [x] Check that the shell stays local-first and adapter-driven.
    - [x] Check that the design supports mature scanning behavior rather than ornamental layout.
    - [x] Resolve any duplicated responsibilities with the implementation and accessibility tracks.
- [x] Task: Publish the design handoff notes for downstream implementation.
    - [x] Summarize the approved shell states and command surfaces.
    - [x] Summarize the layout rules that the code should preserve.
    - [x] Capture any open questions that must be solved in the implementation track.
- [x] Task: Publish the Figma-to-roadmap traceability matrix.
    - [x] Map each Figma frame/state to AQ implementation tasks.
    - [x] Map each Figma frame/state to AR validation evidence.
    - [x] Document the process for updating Figma when implementation changes the design.
- [x] Task: Completed: docs/native-macos-handoff-notes.md and figma-ux-roadmap.md updated.

## Exit Criteria

- Wireframe exists and is usable as implementation input.
- UX brief, design-token guidance and Figma-to-roadmap traceability exist.
- Shell interaction model is documented.
- Review and phase gates are complete.
