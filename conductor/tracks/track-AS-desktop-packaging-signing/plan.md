# Track AS: Desktop Packaging and Signing Plan

## Phase AS1 - Packaging matrix hardening

- [x] Task: Add desktop artifact matrix validation for macOS, Windows and Linux.
    - [x] Verify macOS artifact definition covers signed DMG or PKG.
    - [x] Verify Windows artifact definition covers signed MSI/MSIX or installer equivalent.
    - [x] Verify Linux artifact definition covers AppImage, deb, rpm and tarball or documented waivers.
    - [x] Add focused tests or script checks for missing artifact definitions.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AS1 --auto-fix`.

## Phase AS2 - Signing and notarization readiness

- [x] Task: Harden signing readiness checks for desktop stable release.
    - [x] Require macOS Developer ID and notarization evidence for stable macOS artifacts.
    - [x] Require Windows signing certificate evidence for stable Windows artifacts.
    - [x] Require Linux checksum/signing policy evidence for stable Linux artifacts.
    - [x] Ensure all signing secrets stay external to the repository.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AS2 --auto-fix`.

## Phase AS3 - Checksums and artifact layout

- [x] Task: Implement deterministic release artifact naming and checksum validation.
    - [x] Define stable artifact paths and names.
    - [x] Validate detached SHA-256 checksums for every public desktop artifact.
    - [x] Link checksum output into the release evidence bundle.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AS3 --auto-fix`.

## Phase AS4 - Documentation and registry status

- [x] Task: Update packaging documentation and registry status.
    - [x] Document local release artifact commands.
    - [x] Document CI release artifact commands.
    - [x] Replace generic desktop packaging blockers with concrete readiness states or external blockers.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AS4 --auto-fix`.

## Exit Criteria

- Desktop packaging and signing checks pass.
- Release readiness consumes the desktop artifact/signing evidence.
- Any external account, certificate or notarization blocker is explicitly recorded without exposing secrets.

## Completion Evidence

- `packaging/package-matrix.yaml` now declares desktop release artifact definitions for macOS, Windows and Linux.
- `scripts/desktop_packaging_signing_check.py` validates desktop artifact definitions, checksum paths and concrete registry readiness states.
- `.github/workflows/07-release.yml` runs the desktop packaging/signing check before release readiness.
- `scripts/release_readiness_check.sh` includes `desktop_packaging_signing` evidence in the release readiness bundle.
- `docs/desktop-release-packaging.md` documents local and CI checks.
