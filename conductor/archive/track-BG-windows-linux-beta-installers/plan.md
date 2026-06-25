# Track BG: Windows and Linux Beta Installers Plan

## Phase BG1 - Installer build recipes

- [ ] Task: Define Windows and Linux build recipes.
    - [ ] Verify MSI/MSIX/ZIP generation path.
    - [ ] Verify AppImage/tarball/deb/rpm generation path.
    - [ ] Record signing and package-manager limitations.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BG1 --auto-fix`.

## Phase BG2 - CI artifact generation

- [ ] Task: Add CI artifact generation.
    - [ ] Build Windows beta artifacts with checksums.
    - [ ] Build Linux beta artifacts with checksums.
    - [ ] Upload artifacts to release workflow evidence without publishing registries automatically.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BG2 --auto-fix`.

## Phase BG3 - Platform smoke tests

- [ ] Task: Verify usable beta installs.
    - [ ] Run Windows install/launch/open smoke.
    - [ ] Run Linux install/launch/open smoke in container and desktop-capable environment where available.
    - [ ] Capture known limitations per platform.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BG3 --auto-fix`.

## Phase BG4 - Beta publication handoff

- [ ] Task: Publish beta artifacts with clear status.
    - [ ] Upload real Windows/Linux beta assets after maintainer approval.
    - [ ] Supersede placeholder assets.
    - [ ] Keep registry manifests blocked until stable signing/review gates pass.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BG4 --auto-fix`.
