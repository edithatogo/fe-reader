# Stable Release Cutover and Registries Contract

Fe Reader meets the stable release cutover and registries contract only when the stable desktop cutover evidence is present and the publication registries remain gated by signed artifacts, checksums, release evidence and approval.

## Contract

- Stable release cutover uses `docs/stable-desktop-release.md` and the release evidence bundle as the public handoff point.
- Registry publication remains manual-gated until signed artifacts, checksums, release evidence and maintainer approval exist.
- Registry status files remain the source of truth for each desktop surface.
- GitHub Releases remains the canonical public release index.
- Deferred registry surfaces stay blocked until their specific artifacts and review paths exist.

## Required evidence

- `docs/stable-desktop-release.md`
- `docs/desktop-distribution-publication.md`
- `packaging/desktop-distribution.yaml`
- `packaging/registry-status.yaml`
- `target/release-evidence/stable-release-evidence.json`
- `target/release-evidence/desktop-distribution-publication.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/launch-qa.json`

## Registry surfaces

- GitHub Releases
- Homebrew
- Winget
- Chocolatey
- Scoop
- Flatpak
- Snap
- AUR

This contract does not claim that publication has already happened. It binds the stable release cutover and registry surfaces to the evidence and blockers already present in the repository.
