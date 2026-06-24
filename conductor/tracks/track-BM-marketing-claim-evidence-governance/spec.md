# Track BM: Marketing Claim Evidence Governance

## Overview

Make marketing readiness evidence-driven. Fe Reader can be marketed as a technical preview, beta or mature stable product only when release artifacts, capability evidence and support processes match the claim.

## Functional Requirements

- Define marketing readiness levels: technical preview, public beta, stable desktop, mature stable and v2 roadmap.
- Map each readiness level to required artifacts, CI results, parity evidence, support posture, screenshots, docs and release notes.
- Add a claim scanner for README, homepage, docs site, release notes and package metadata.
- Require explicit approval before broad marketing copy or release status changes.

## Non-Functional Requirements

- Public copy must avoid vendor-clone framing.
- Claims must be neutral, evidence-linked and privacy-safe.
- Known limitations must remain visible for preview and beta channels.

## Acceptance Criteria

- A marketing readiness report exists and is attached to release evidence.
- CI fails if marketing copy makes unsupported stable or parity claims.
- Public docs clearly distinguish technical preview, beta and stable.

## Out of Scope

- Paid advertising execution.
- Analytics or tracking by default.
