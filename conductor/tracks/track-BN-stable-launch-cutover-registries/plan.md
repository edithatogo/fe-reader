# Track BN: Stable Launch Cutover and Registries Plan

## Phase BN1 - Release candidate assembly

- [ ] Task: Assemble release candidate.
    - [ ] Build target artifacts.
    - [ ] Generate checksums, SBOM, provenance and release evidence.
    - [ ] Run stable reader, parity, launch QA and marketing readiness checks.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BN1 --auto-fix`.

## Phase BN2 - GitHub Release cutover

- [ ] Task: Replace placeholder assets.
    - [ ] Upload real installable assets.
    - [ ] Remove or supersede `NOT_AN_INSTALLER` placeholders for published platforms.
    - [ ] Attach release evidence and clear limitations.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BN2 --auto-fix`.

## Phase BN3 - Registry submissions

- [ ] Task: Submit registries in evidence order.
    - [ ] Submit Homebrew after macOS artifact/checksum evidence.
    - [ ] Submit winget/Chocolatey/Scoop after Windows artifact evidence.
    - [ ] Submit Linux package registries after Linux artifact evidence.
    - [ ] Record external review blockers.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BN3 --auto-fix`.

## Phase BN4 - Stable launch verification

- [ ] Task: Verify public launch state.
    - [ ] Validate download links, checksums and release notes.
    - [ ] Validate homepage, README, repository metadata and package links.
    - [ ] Produce launch cutover report.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BN4 --auto-fix`.
