# Track BR: Shared Workflow Surface Facade

## Overview

Define and implement the shared workflow surface that all native, web and automation adapters use for comparable Fe Reader capabilities. The facade must make platform parity measurable without moving platform concerns into `fe_reader_core`.

## Functional Requirements

- Define stable operation IDs for open, recents, metadata/hash, navigation, zoom, search, accessibility, validate, annotations, forms, page organization, redaction, conversion and approved-plan apply.
- Expose the facade through adapter-safe CLI JSON, UniFFI, C ABI and web postMessage contracts.
- Preserve read-only and plan-only defaults for all automation and web/browser surfaces.
- Add API compatibility notes for CLI JSON, UniFFI, C ABI, MCP, web postMessage, Android intents, iOS App Intents, Windows COM, AppleScript and Linux D-Bus.

## Non-Functional Requirements

- `fe_reader_core` remains pure and must not gain UI/platform/renderer/web/plugin/MCP dependencies.
- All write-capable operations must flow through `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.

## Acceptance Criteria

- Stable facade operation IDs exist in a contract checked by CI.
- Adapter snapshots or compatibility notes are updated for every public surface changed.
- Platform parity matrix references the facade as evidence.

## Out of Scope

- Building full platform shells.
- Enabling direct redaction or high-risk apply shortcuts.
