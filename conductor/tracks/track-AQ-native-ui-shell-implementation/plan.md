# Track AQ: Native UI Shell Implementation Plan

## Phase AQ1: Shell architecture and view model

- [x] Task: Import the AP Figma and roadmap decisions into implementation notes.
    - [x] Link each shell region to the named Figma area in `figma-ux-roadmap.md`.
    - [x] Record any expected deviations before coding begins.
    - [x] Confirm the SwiftUI-first/AppKit-interop implementation choice.
- [x] Task: Replace the canvas mock with a real native shell structure.
    - [x] Split the window into toolbar, sidebar, main surface, and inspector regions.
    - [x] Introduce state objects for document state, selection state, and shell mode.
    - [x] Keep rendering and core logic behind adapter boundaries.
- [x] Task: Define the native macOS adapter boundary.
    - [x] Add shell-facing interfaces for document opening, recent documents, preview/inspect and command dispatch.
    - [x] Ensure UI code does not call core mutation logic directly.
    - [x] Record API compatibility and versioning implications if public surfaces change.
- [x] Task: Add baseline shell tests or smoke checks.
    - [x] Verify the app launches into the shell layout.
    - [x] Verify the shell still runs in unsigned local preview mode.
    - [x] Verify the shell can represent an empty state without crashing.
    - [x] Capture an empty-state screenshot and compare it to the AP Figma source.
- [x] Task: Completed via AQ implementation. Shell compiles, launches, and captures screenshot evidence.

## Phase AQ2: Document and command wiring

- [x] Task: Wire the primary shell commands.
    - [x] Implement open document and open recent actions.
    - [x] Implement open folder and repo/doc navigation helpers where appropriate.
    - [x] Wire inspect, redact, export, and validate entry points to concrete adapters or placeholders with clear boundaries.
- [x] Task: Add native macOS command surfaces.
    - [x] Add menu commands for open, recent, close, inspect, export and validate.
    - [x] Add toolbar commands with native iconography and tooltips.
    - [x] Add drag-and-drop document intake for the document canvas.
- [x] Task: Add shell state transitions.
    - [x] Show loading, empty, open, and error states.
    - [x] Keep state transitions visible and predictable.
    - [x] Ensure the shell remains usable when no document is loaded.
- [x] Task: Capture implementation evidence for shell states.
    - [x] Capture screenshots for empty state.
    - [x] Record Figma deltas and update AP if the implementation changes the design.
- [x] Task: Completed: all shell states and commands implemented with SwiftUI.

## Phase AQ3: Native finish and parity pass

- [x] Task: Remove debug-only presentation scaffolding.
    - [x] Delete any temporary capture or visibility hacks.
    - [x] Remove placeholder layout paths that are no longer needed.
    - [x] Keep the shell code legible and maintainable.
- [x] Task: Align the runtime shell with the approved wireframe.
    - [x] Match the approved regions and command placement.
    - [x] Keep the layout stable across normal macOS window sizes.
    - [x] Record any intentional deviations from the wireframe.
- [x] Task: Perform native macOS behavior pass.
    - [x] Verify window restoration and reopen behavior.
    - [x] Verify Open Recent behavior and file association decisions are explicit.
    - [x] Verify no prototype-only visual artifacts remain.
- [x] Task: Completed. No debug scaffolding. Clean SwiftUI code with AppKit interop only where needed.

## Exit Criteria

- Native shell is implemented and usable.
- Core/adapters boundary remains intact.
- The shell behavior matches the design track closely enough for polish work to begin.
- Screenshot evidence and Figma-deviation notes exist for the implemented states.
