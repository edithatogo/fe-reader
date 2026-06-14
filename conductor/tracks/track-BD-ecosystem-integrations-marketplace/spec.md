# Track BD: Ecosystem Integrations and Marketplace

## Overview

Expand SDK, plugin, workflow-pack and marketplace integration surfaces after launch gates are stable. This track does not block desktop stable launch.

## Scope

- SDK compatibility snapshots.
- Plugin/runtime publication safety.
- Workflow-pack distribution and provenance.
- Integration marketplace metadata.

## Functional Requirements

- Automation and plugin mutations remain read-only or plan-only by default.
- SDK, plugin and integration API changes require compatibility notes.
- Package and marketplace publication requires evidence and maintainer approval.

## Non-Functional Requirements

- Feature gate: `ecosystem_integrations_marketplace`.
- Rollback: disable plugin/runtime publication and remove marketplace claims.
- Exit criteria: SDK, plugin and integration surfaces have compatibility snapshots, safety gates and publication evidence.

## Acceptance Criteria

- Integration expansion backlog is documented.
- Compatibility snapshots cover public APIs.
- Publication gates do not weaken automation safety.

## Out of Scope

- Unsafe plugin runtime enablement.
- Store publication without signed artifacts.
- Launch-blocking desktop release work.
