# Track AS: Desktop Packaging and Signing

## Overview

Make Fe Reader's desktop release artifacts concrete and release-gated for an enterprise-ready stable public launch. This track covers macOS, Windows and Linux packaging, signing and notarization readiness, checksum policy, artifact naming, and packaging matrix alignment.

## Scope

- macOS signed and notarized desktop artifact readiness.
- Windows signed MSI/MSIX or equivalent installer readiness.
- Linux AppImage, deb, rpm and tarball packaging matrix readiness.
- Detached checksums for all desktop release artifacts.
- Stable artifact naming, versioning and directory layout.
- No signing secrets in repository files, logs or artifacts.

## Functional Requirements

- Build or validate desktop artifact definitions for macOS, Windows and Linux.
- Add or tighten release packaging checks so missing artifact definitions fail the stable gate.
- Add signing readiness checks for macOS Developer ID/notarization, Windows signing and Linux checksum/signing policy.
- Update `packaging/registry-status.yaml` with concrete desktop artifact blockers or readiness states.
- Document exact local and CI commands for producing release candidate artifacts.

## Non-Functional Requirements

- Keep `fe_reader_core` free of packaging, signing and platform dependency leakage.
- Keep GitHub Actions permissions minimal and explicit.
- Do not commit certificates, keys, notary profiles, passwords or account tokens.
- Signing gates fail closed for stable release channels when evidence is missing.

## Acceptance Criteria

- A desktop packaging/signing check runs locally and in CI.
- The packaging matrix enumerates macOS, Windows and Linux desktop artifacts.
- Stable channel signing readiness rejects missing required signing evidence.
- Detached checksum requirements are documented and checked.
- All new checks are wired into release readiness evidence.

## Out of Scope

- Android/iOS public store launch.
- ML/RAG/frontier intelligence.
- Cloud sync or cloud collaboration services.

