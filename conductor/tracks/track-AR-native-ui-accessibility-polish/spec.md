# Track AR: Native UI Accessibility and Polish

## Overview

Bring the native macOS shell from functional to mature. This track hardens keyboard flow, accessibility, resizing, empty-state clarity, visual consistency, and Figma-to-runtime conformance so the app feels finished rather than merely present.

## Functional Requirements

- Add keyboard paths for the main shell commands and navigation surfaces.
- Add accessibility labels, focus order, and screen-reader-friendly structure.
- Validate that the shell remains usable at smaller and larger window sizes.
- Define and cover loading, empty, error, and read-only states with clear visual treatment.
- Add visual regression or screenshot coverage for the native shell.
- Compare runtime screenshots against the Track AP Figma artifact and record intentional deviations.
- Verify menus, toolbar controls, Open Recent, drag-and-drop, and window behavior with keyboard and accessibility expectations.
- Record any remaining polish gaps and their follow-up decisions.

## Non-Functional Requirements

- The shell must remain local-first and privacy-preserving.
- The UI should be utilitarian and scan-friendly rather than decorative.
- The finish work must not relax core/adapters or safety boundaries.
- Accessibility and keyboard support are part of the product quality bar, not optional extras.
- Visual polish must be grounded in the AP Figma artifact, not subjective late-stage restyling.

## Acceptance Criteria

- The main shell has usable keyboard navigation for primary actions.
- Accessibility labels and focus behavior are present for the key surfaces.
- The layout holds together across the expected window sizes.
- Visual or screenshot-based regression evidence exists for the shell.
- The app reads as a finished native tool rather than a prototype.
- The shell passes measurable UX gates: no clipped primary text at target sizes, primary actions reachable by keyboard, accessible names for key controls, and documented contrast/focus behavior.
- Figma deviations are documented and either fed back into AP or accepted as runtime-native improvements.

## Out of Scope

- New rendering engine work.
- Signing/notarization/distribution.
- New core document formats or workflow packs.
- Broad feature expansion beyond the shell maturity pass.
