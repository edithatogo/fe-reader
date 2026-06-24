# Track BN: Stable Launch Cutover and Registries

## Overview

Move Fe Reader from prerelease placeholder assets to real stable release artifacts, checksums, evidence, registry submissions and public download messaging.

## Functional Requirements

- Remove or supersede `NOT_AN_INSTALLER` assets once real artifacts exist.
- Publish real macOS public-quality artifact first, with Windows/Linux beta artifacts where available.
- Generate checksums, signatures, SBOM, provenance, release evidence and marketing readiness evidence.
- Update GitHub Releases, registry manifests and repository homepage links only after evidence and approval.
- Submit package registries in a controlled sequence with rollback notes.

## Non-Functional Requirements

- No secrets in repository files.
- Registry submissions require maintainer approval.
- Stable channel must fail closed if signed artifacts or evidence are missing.

## Acceptance Criteria

- GitHub Release has real installable assets and no misleading placeholders for published platforms.
- Registry status accurately reflects ready, submitted, published, blocked or deferred state.
- Release evidence is complete and linked from release notes.

## Out of Scope

- Automatically submitting to third-party stores without approval.
- Mobile public launch unless separately promoted.
