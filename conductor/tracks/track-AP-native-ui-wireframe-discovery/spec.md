# Track AP: Native UI Wireframe Discovery

## Overview

Define the native macOS shell before implementation so the app moves toward a mature, predictable UX instead of growing ad hoc controls. The track produces a Figma wireframe, a UX brief, design-token guidance, an interaction model, and the first pass of navigation/state decisions for the shell.

Reference wireframe file:
`https://www.figma.com/design/LY0vDEVEjqd96xcjvcYqW1`

## Functional Requirements

- Produce a native macOS desktop shell wireframe in Figma.
- Treat the Figma wireframe as the design source of truth for AP, AQ and AR until a newer approved design artifact supersedes it.
- Produce a written UX brief covering primary users, top workflows, first-run behavior, empty/error states, command hierarchy, and explicit anti-goals.
- Define the shell layout: title/toolbar, library sidebar, document canvas, and inspector panel.
- Define the key shell states: empty, loading, open document, error, and read-only/confirmation states.
- Define the primary command surface for open, recent, inspect, redact, export, and validate.
- Define design-token guidance for spacing, typography, color roles, icon usage, density, and component anatomy.
- Document the interaction model for keyboard navigation, window resizing, and state transitions.
- Record the design decisions that should flow into implementation tracks.
- Maintain a roadmap mapping from each Figma frame/state to the implementing phase and validation phase.

## Non-Functional Requirements

- The wireframe must stay aligned with the product boundary: local-first, privacy-preserving, and core/adapters separated.
- The UX should be designed for scanning and repeated use, not marketing presentation.
- The wireframe must remain implementation-guiding, not decorative.
- Design decisions must be traceable: every shell region, state, and primary action in AQ/AR should link back to a named Figma frame or a documented AP decision.
- The design system should use restrained, native-feeling macOS affordances and avoid ornamental card-heavy layouts.

## Acceptance Criteria

- A Figma wireframe exists for the native shell.
- The wireframe covers the main shell layout and the important states.
- A UX brief and design-token note exist and are linked from this track.
- The interaction model is documented clearly enough to drive implementation.
- The AP roadmap maps the Figma artifact to AQ implementation tasks and AR validation tasks.
- Implementation tracks can use the artifact as a source of truth.
- Any deviation from the Figma wireframe has a documented rationale before implementation proceeds.

## Out of Scope

- Native shell implementation code.
- Signing, notarization, packaging, or store distribution.
- Non-macOS platform shells.
- PDF rendering engine work.
