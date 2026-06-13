# Track AW: Launch QA, Documentation and Homepage

## Overview

Close the public-facing desktop launch gap with release QA, desktop smoke evidence, README/docs/homepage updates, limitations, support information and install instructions.

## Scope

- Desktop smoke tests.
- CLI and contract gates.
- UI smoke tests where a desktop shell exists.
- Compatibility, performance, accessibility, security and visual regression evidence.
- README and docs site release pages.
- GitHub homepage metadata represented in repo files.
- Known limitations and support policy.

## Functional Requirements

- Define a launch QA checklist for stable desktop release.
- Add docs that link install commands, release artifacts, checksums and evidence.
- Ensure public-facing claims match available evidence.
- Document limitations for mobile, ML/RAG and cloud collaboration.
- Provide support and vulnerability reporting links.

## Non-Functional Requirements

- Documentation must avoid overstating unsupported capabilities.
- QA evidence must be reproducible locally or in CI.
- Accessibility and privacy claims must be evidence-backed.

## Acceptance Criteria

- README and docs site contain stable desktop install and verification instructions.
- Launch QA command summarizes required desktop release checks.
- GitHub homepage metadata has correct project URL/package links where repo-managed.
- Known limitations are documented.

## Out of Scope

- Marketing site redesign.
- Mobile store pages.
- Remote telemetry.

