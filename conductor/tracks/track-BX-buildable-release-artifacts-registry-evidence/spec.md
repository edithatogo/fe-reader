# Track BX: Buildable Release Artifacts and Registry Evidence

## Overview

Produce buildable unsigned or development release artifacts and checksum evidence wherever credentials are unavailable, while keeping public signing and registry publication as external gates.

## Functional Requirements

- Add buildable dev artifact paths for macOS, Windows, Linux, Android, iOS, Web/PWA and browser extension where feasible.
- Generate checksums and release evidence manifests.
- Update packaging and registry status without falsely claiming signed/notarized/store-published status.
- Keep external gates explicit for notarization, code signing, store submission and registry review.

## Non-Functional Requirements

- No secrets or private credentials may be committed.
- CI permissions stay minimal and explicit.

## Acceptance Criteria

- Buildable artifact smoke produces manifests and checksums.
- Packaging status distinguishes dev artifacts from public release artifacts.
- Registry status remains fail-closed for external-gated publication.

## Out of Scope

- Actually submitting to stores or registries.
- Removing signing/notarization requirements for stable public release.
