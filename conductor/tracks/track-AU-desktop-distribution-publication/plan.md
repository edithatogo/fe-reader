# Track AU: Desktop Distribution Publication Plan

## Phase AU1 - GitHub Release readiness

- [ ] Task: Make GitHub Release creation reproducible.
    - [ ] Validate tag, changelog, release notes, artifacts and checksums.
    - [ ] Require maintainer approval before publication.
    - [ ] Document `gh` commands and manual fallback.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AU1 --auto-fix`.

## Phase AU2 - Registry manifest validation

- [ ] Task: Validate desktop registry manifests.
    - [ ] Validate Homebrew manifest.
    - [ ] Validate Winget manifest.
    - [ ] Validate Chocolatey manifest.
    - [ ] Validate Scoop manifest.
    - [ ] Validate Flatpak, Snap and AUR manifests or concrete blockers.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AU2 --auto-fix`.

## Phase AU3 - Registry status and blockers

- [ ] Task: Replace generic publication deferrals with concrete states.
    - [ ] Mark ready surfaces as ready pending approval.
    - [ ] Mark external-blocked surfaces with exact blocker.
    - [ ] Preserve no-secret policy for credentials and tokens.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AU3 --auto-fix`.

## Phase AU4 - Public links

- [ ] Task: Link release artifacts from public project surfaces.
    - [ ] Update README.
    - [ ] Update docs site release/install pages.
    - [ ] Update GitHub homepage metadata where represented in repo files.
- [ ] Task: Run `scripts/conductor_phase_gate.sh --phase AU4 --auto-fix`.

## Exit Criteria

- Desktop distribution surfaces are ready, published or explicitly externally blocked.
- Publication requires signed artifacts, evidence and maintainer approval.

