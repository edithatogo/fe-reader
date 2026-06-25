# Track BS: Desktop Shell Parity

## Overview

Bring Windows and Linux desktop shell behavior toward the macOS SwiftUI/PDFKit reference while preserving adapter boundaries and mutation safety.

## Functional Requirements

- Keep the macOS SwiftUI/PDFKit shell as the reference reader surface.
- Add or harden Windows and Linux Tauri v2 desktop shell parity for open, recents, navigation, zoom, search, inspect, validate and workflow planning.
- Provide desktop smoke tests for macOS, Windows and Linux.
- Keep direct mutations disabled unless they use the approved mutation pipeline.

## Non-Functional Requirements

- Desktop shells must call adapter surfaces or CLI/core contracts rather than embedding document logic in UI code.
- Windows and Linux packaging status must remain honest about signing and registry blockers.

## Acceptance Criteria

- macOS, Windows and Linux can launch the desktop shell and open the baseline fixture in smoke tests.
- Search, inspect and validate have equivalent user-visible paths on all desktop platforms.
- Workflow actions expose plan/review gates and do not bypass policy.

## Out of Scope

- Store publication.
- Direct redaction apply.
