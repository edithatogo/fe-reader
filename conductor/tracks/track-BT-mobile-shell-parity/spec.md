# Track BT: Mobile Shell Parity

## Overview

Create Android and iOS mobile shell parity through UniFFI-backed read and plan surfaces, with native document intake and mobile-specific accessibility evidence.

## Functional Requirements

- Add Android and iOS wrapper surfaces for document open/share, search, inspect, validate, accessibility and plan-only workflow actions.
- Use UniFFI or adapter contracts; do not duplicate document logic in mobile UI code.
- Preserve read-only and plan-only defaults for mobile intents and App Intents.
- Attach emulator/simulator smoke evidence and update the platform parity matrix.

## Non-Functional Requirements

- Mobile packages remain release-deferred until signed artifacts and store evidence exist.
- Real-device power/accessibility evidence may remain an external evidence gate if unavailable.

## Acceptance Criteria

- Android emulator smoke and iOS simulator smoke pass.
- Mobile contract snapshots remain read-only or plan-only unless an approved apply path is explicitly implemented in Track BV.
- Platform parity matrix records accurate mobile support.

## Out of Scope

- Play Store or App Store submission.
- Cloud sync, analytics or hidden uploads.
