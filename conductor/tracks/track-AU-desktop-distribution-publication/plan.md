# Track AU: Desktop Distribution Publication Plan

## Phase AU1 - GitHub Release readiness

- [x] Task: Make GitHub Release creation reproducible.
    - [x] Validate tag, changelog, release notes, artifacts and checksums.
    - [x] Require maintainer approval before publication.
    - [x] Document `gh` commands and manual fallback.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AU1 --auto-fix`.

## Phase AU2 - Registry manifest validation

- [x] Task: Validate desktop registry manifests.
    - [x] Validate Homebrew manifest.
    - [x] Validate Winget manifest.
    - [x] Validate Chocolatey manifest.
    - [x] Validate Scoop manifest.
    - [x] Validate Flatpak, Snap and AUR manifests or concrete blockers.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AU2 --auto-fix`.

## Phase AU3 - Registry status and blockers

- [x] Task: Replace generic publication deferrals with concrete states.
    - [x] Mark ready surfaces as ready pending approval.
    - [x] Mark external-blocked surfaces with exact blocker.
    - [x] Preserve no-secret policy for credentials and tokens.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AU3 --auto-fix`.

## Phase AU4 - Public links

- [x] Task: Link release artifacts from public project surfaces.
    - [x] Update README.
    - [x] Update docs site release/install pages.
    - [x] Update GitHub homepage metadata where represented in repo files.
- [x] Task: Run `scripts/conductor_phase_gate.sh --phase AU4 --auto-fix`.

## Exit Criteria

- Desktop distribution surfaces are ready, published or explicitly externally blocked.
- Publication requires signed artifacts, evidence and maintainer approval.

## Completion Evidence

- Added `packaging/desktop-distribution.yaml` as the desktop publication source of truth.
- Added `scripts/desktop_distribution_publication_check.py` and wired it into the release workflow, provenance check and readiness evidence.
- Updated desktop registry manifests to match `v0.1.0-preview.1` artifact names.
- Added release notes, desktop publication docs, README package links and repository metadata.
- Verified dev-mode distribution checks pass and stable-mode publication fails closed without signed artifacts/checksums.
- Ran `scripts/conductor_phase_gate.sh --phase AU1 --auto-fix`.
- Ran `scripts/conductor_phase_gate.sh --phase AU2 --auto-fix`.
- Ran `scripts/conductor_phase_gate.sh --phase AU3 --auto-fix`.
- Ran `scripts/conductor_phase_gate.sh --phase AU4 --auto-fix`.
