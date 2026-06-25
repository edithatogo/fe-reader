# Track BY: Parity Dashboard and Claim Governance

## Overview

Publish a local-first parity dashboard and claim-governance gate that ties platform claims to evidence from the matrix, QA reports and release artifacts.

## Functional Requirements

- Add a dashboard/report that summarizes parity by platform and capability.
- Gate public marketing and release claims against `docs/platform-parity-matrix.json`.
- Show supported, plan-only, documented limitation and external-gate states without collapsing them.
- Keep dashboard data local-first and privacy-preserving.

## Non-Functional Requirements

- No telemetry, analytics SDK or phone-home behavior.
- Dashboard output must be deterministic and suitable for CI artifact publication.

## Acceptance Criteria

- Dashboard/checker fails if public docs claim unsupported parity.
- Dashboard links each supported claim to evidence.
- Existing PDF parity and platform parity checks remain green.

## Out of Scope

- Hosted analytics.
- Marketing claims not backed by evidence.
