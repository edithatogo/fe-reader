# Track BF: macOS Public Quality Release Plan

## Phase BF1 - macOS release prerequisites

- [ ] Task: Audit macOS build and signing readiness.
    - [ ] Verify app bundle, entitlements, hardened runtime, sandbox decisions and privacy prompts.
    - [ ] Verify Developer ID and notarization credential availability outside the repo.
    - [ ] Document unresolved Apple account or verification blockers.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BF1 --auto-fix`.

## Phase BF2 - Installable artifact build

- [ ] Task: Produce macOS release artifacts.
    - [ ] Build release app bundle.
    - [ ] Package DMG and optional PKG.
    - [ ] Generate SHA256 files and release artifact inventory evidence.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BF2 --auto-fix`.

## Phase BF3 - Signing, notarization and smoke

- [ ] Task: Sign, notarize and verify macOS artifacts.
    - [ ] Run codesign, notarization submission and stapling.
    - [ ] Verify `spctl`, Gatekeeper open path, quarantine behavior and launch smoke.
    - [ ] Run macOS UI smoke and accessibility checks.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BF3 --auto-fix`.

## Phase BF4 - Public macOS release handoff

- [ ] Task: Publish macOS-ready evidence.
    - [ ] Upload real macOS artifacts to prerelease or stable release after maintainer approval.
    - [ ] Update registry status and Homebrew manifest only with final artifact URLs/checksums.
    - [ ] Remove or supersede macOS `NOT_AN_INSTALLER` placeholders.
- [ ] Task: Conductor phase gate.
    - [ ] Run `scripts/conductor_phase_gate.sh --phase BF4 --auto-fix`.
