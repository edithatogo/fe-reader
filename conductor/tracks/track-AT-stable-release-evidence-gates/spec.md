# Track AT: Stable Release Evidence Gates

## Overview

Replace preview-grade release evidence with stable-channel release evidence gates. Stable release must require real SBOM, provenance, checksums, signing readiness, compatibility, performance, accessibility, security and visual-regression reports.

## Scope

- Stable release evidence bundle.
- SBOM and provenance/attestation generation.
- Security/dependency report.
- Performance budget report.
- Compatibility corpus report.
- Accessibility report.
- Visual regression report.
- Explicit waiver model with owner, expiry, rationale and rollback path.

## Functional Requirements

- Harden `scripts/release_readiness_check.sh` and related evidence checks for stable release.
- Ensure placeholder/advisory evidence cannot satisfy stable release requirements.
- Validate evidence paths, schema shape, source commit, tag, artifact hashes and signing readiness.
- Add test fixtures for passing and failing stable evidence bundles.
- Emit actionable error messages for each missing evidence class.

## Non-Functional Requirements

- Release evidence must be machine-readable.
- Stable gate must fail closed.
- Secrets must not appear in evidence.
- Evidence checks must be runnable locally and in GitHub Actions.

## Acceptance Criteria

- Stable release readiness rejects placeholder SBOM/provenance/signing evidence.
- Stable release readiness passes with a complete real-evidence fixture.
- Release CI uploads a complete release evidence artifact.
- Waivers are explicit, expiring and reviewable.

## Out of Scope

- Mobile public release.
- Cloud service telemetry.
- Model-based feature launch gates.

