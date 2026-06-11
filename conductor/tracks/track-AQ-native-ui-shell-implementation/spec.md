# Track AQ: Native UI Shell Implementation

## Overview

Implement the native macOS shell using the Track AP Figma wireframe and UX roadmap as the source of truth. Replace the current placeholder shell with a stateful desktop UI that feels like a finished application instead of a paint-only mock.

Framework decision: `docs/adr/0011-native-macos-shell-framework.json`.

## Functional Requirements

- Replace the current placeholder drawing surface with a real native layout.
- Use a SwiftUI-first shell with AppKit interop only for platform behavior that SwiftUI cannot cover cleanly.
- Trace each implemented shell region back to the matching named Figma area or AP design decision.
- Implement the main shell regions: toolbar/title area, library sidebar, document surface, and inspector panel.
- Implement native macOS menu, toolbar, Open Recent, drag-and-drop and window restoration decisions where feasible for the preview shell.
- Wire real actions for open document, recent documents, open folder, inspect, redact, export, and validate.
- Support the important shell states: empty, loading, document open, and error.
- Preserve window behavior that feels native: activation, focus, resize, and close/reopen behavior.
- Keep the shell local-first and adapter-driven so UI code does not absorb core logic.
- Define a `native/macos` adapter boundary for document opening, recent documents, document preview/inspect, and command dispatch.
- Capture runtime screenshot evidence for the major shell states as implementation evidence.

## Non-Functional Requirements

- The shell must resize cleanly on macOS window changes.
- The shell must be readable and usable without relying on decorative placeholders.
- The implementation must not introduce UI dependencies into the core layer.
- The layout must remain compatible with later accessibility and keyboard work.
- Figma deviations must be intentional and documented before the phase closes.
- No command that mutates document content may bypass the project mutation pipeline.

## Acceptance Criteria

- The app opens to a functioning native shell instead of the current placeholder drawing output.
- The main shell regions exist and are visible at runtime.
- At least the primary shell actions are wired to concrete behavior.
- The implementation tracks the wireframe closely enough to remain recognizable.
- Runtime screenshots exist for empty, loading or simulated loading, open-document or sample-document, and error states.
- The shell records its adapter boundaries and Figma deviations.

## Out of Scope

- Signing, notarization, and store distribution.
- PDF rendering engine implementation beyond what the shell needs to host.
- Non-macOS platform shells.
- Accessibility and regression hardening beyond the minimum needed for this track.
