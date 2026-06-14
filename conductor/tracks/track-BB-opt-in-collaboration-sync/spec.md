# Track BB: Opt-in Collaboration and Sync

## Overview

Explore collaboration and sync as explicit opt-in, local-first functionality after desktop stable launch. This track does not block desktop stable launch.

## Scope

- Local-first collaboration packet contracts.
- Sync provider capability discovery.
- Privacy-sensitive cache and packet handling.
- Offline-first failure modes and user consent.

## Functional Requirements

- cloud collaboration is opt-in and disabled by default.
- No silent upload, sync, analytics or phone-home behavior.
- Collaboration packets, cache entries and quality signals are privacy-sensitive.

## Non-Functional Requirements

- Feature gate: `opt_in_collaboration_sync`.
- Rollback: disable sync providers and retain local-only workspaces.
- Exit criteria: collaboration data contracts are privacy-reviewed, reversible and user-controlled.

## Acceptance Criteria

- Collaboration/sync contracts are documented.
- Sync providers use capability discovery and clear failure modes.
- Local-only behavior remains the default.

## Out of Scope

- Default cloud accounts.
- Remote analytics.
- Launch-blocking desktop work.
