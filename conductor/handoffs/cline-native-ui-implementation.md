# Cline Handoff: Native macOS UI Implementation

## Invocation Requirements

- Run headless, not TUI.
- Provider: `deepseek`
- Model: `deepseek-v4-flash`
- Reasoning effort: `medium`
- Use as many Cline subagents as the local configuration, rate limits and task decomposition safely allow.
- Use nested subagents when useful for independent review, implementation, validation and documentation tasks.

## Repository

Working directory: `/Volumes/PortableSSD/GitHub/fe-reader`

## Product Constraints

Follow `AGENTS.md`.

Non-negotiables:

- Keep `fe_reader_core` pure: no UI, platform, renderer, AI, MCP or plugin runtime dependencies.
- Native UI code belongs under adapters/native/platform layers, not in core crates.
- All mutating document flows must follow `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
- Automation and platform integrations are read-only by default unless policy/review gates are satisfied.
- Do not broaden scope beyond the native macOS UI tracks unless a missing contract blocks implementation.

## Source Tracks

Implement the native macOS UI workstream in this order:

1. `conductor/tracks/track-AP-native-ui-wireframe-discovery/`
2. `conductor/tracks/track-AQ-native-ui-shell-implementation/`
3. `conductor/tracks/track-AR-native-ui-accessibility-polish/`

Use the Figma file as the design source of truth:

`https://www.figma.com/design/LY0vDEVEjqd96xcjvcYqW1`

Use this roadmap file as the traceability contract:

`conductor/tracks/track-AP-native-ui-wireframe-discovery/figma-ux-roadmap.md`

Framework decision:

`docs/adr/0011-native-macos-shell-framework.json`

## Implementation Goal

Replace the current placeholder native macOS app with a mature SwiftUI-first native shell using narrow AppKit interop only where required.

The finished local preview should:

- Launch as an unsigned local macOS app.
- Show a native-feeling shell with toolbar/title region, library sidebar, document canvas and inspector.
- Use the Figma shell regions and states as implementation milestones.
- Avoid manual whole-window drawing for normal UI.
- Remove prototype-only visual/debug scaffolding unless used only by verification tooling.
- Provide concrete actions for opening files, recent documents, inspect, export and validate where feasible.
- Keep redaction or other mutating actions plan/review gated and never bypass the mutation pipeline.
- Capture runtime screenshot evidence for key states.
- Update track plans as tasks are completed.

## Subagent Decomposition

Use parallel subagents for independent work where useful:

- Design traceability: compare code and screenshots against the Figma roadmap.
- Native architecture: design SwiftUI/AppKit structure and adapter boundaries.
- Implementation: replace placeholder AppKit drawing with SwiftUI views and state.
- macOS behavior: menus, toolbar, Open Recent, drag-and-drop, window restoration.
- Accessibility and keyboard: focus order, shortcuts, accessible names.
- Verification: build, launch, screenshot, JSON/contract checks and phase gates.
- Documentation: README, roadmap, track plans and deviation notes.

## Required Verification

Run the strongest feasible subset and report exact failures:

- `./script/build_and_run.sh --verify`
- Native preview screenshot capture, if available.
- JSON validation for any touched metadata/ADR files.
- Relevant Conductor phase gates:
  - `scripts/conductor_phase_gate.sh --phase AP1 --auto-fix`
  - `scripts/conductor_phase_gate.sh --phase AP2 --auto-fix`
  - `scripts/conductor_phase_gate.sh --phase AQ1 --auto-fix`
  - `scripts/conductor_phase_gate.sh --phase AQ2 --auto-fix`
  - `scripts/conductor_phase_gate.sh --phase AQ3 --auto-fix`
  - `scripts/conductor_phase_gate.sh --phase AR1 --auto-fix`
  - `scripts/conductor_phase_gate.sh --phase AR2 --auto-fix`
  - `scripts/conductor_phase_gate.sh --phase AR3 --auto-fix`

If a phase gate fails because the script does not recognize these new phases, document that as a tooling/registry gap and continue with direct build/test evidence.

## Deliverables

- Implemented native macOS shell.
- Updated AP/AQ/AR plans with completed tasks where appropriate.
- Figma deviation notes if implementation intentionally differs.
- Screenshot or preview evidence.
- Clear final summary with what was implemented, what was verified and what remains.
