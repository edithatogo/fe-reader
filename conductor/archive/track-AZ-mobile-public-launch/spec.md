# Track AZ: Mobile Public Launch

## Overview

Prepare Android and iOS public launch readiness after desktop stable gates. This track does not block desktop stable launch.

## Scope

- Android emulator and device smoke evidence.
- iOS simulator and App Intents/share extension smoke evidence.
- Store signing, privacy declarations and package review checklists.
- Mobile-specific accessibility and power/battery checks.

## Functional Requirements

- Keep Android and iOS launch status separate from desktop release readiness.
- Maintain read-only or plan-only automation defaults for mobile intents.
- Require mobile store artifacts, signing evidence and privacy declarations before publication.

## Non-Functional Requirements

- Feature gate: `mobile_public_launch`.
- Rollback: keep mobile packages deferred and release desktop-only notes.
- Exit criteria: mobile public artifacts and store evidence are complete.

## Acceptance Criteria

- Android and iOS launch checklists are actionable.
- CI/mobile smoke evidence exists for each public claim.
- Store submission blockers are explicit.

## Out of Scope

- Desktop release publication.
- Cloud sync.
- ML/RAG features.
