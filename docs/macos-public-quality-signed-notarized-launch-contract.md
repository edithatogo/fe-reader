# macOS Public-Quality Signed/Notarized Launch Contract

Fe Reader meets the macOS public-quality signed/notarized launch contract only when the desktop release path for macOS is evidence-backed, signed, notarized and linked to the public release pipeline.

## Contract

- macOS release artifacts are signed with Developer ID credentials.
- The distributable DMG is notarized and verifiable.
- Release evidence records signing readiness and desktop packaging state.
- Public release documentation points to the same macOS launch evidence.
- Public publication remains blocked until release artifacts and registry approval exist.

## Required evidence

- `target/release-evidence/desktop-packaging-signing.json`
- `target/release-evidence/signing-readiness.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/desktop-distribution-publication.json`
- `target/release-evidence/launch-qa.json`

## Evidence-backed launch surface

The macOS public-quality launch path is documented in:

- `docs/stable-desktop-release.md`
- `docs/desktop-distribution-publication.md`
- `packaging/macos/notarization.md`
- `packaging/desktop-distribution.yaml`

This contract does not claim that public registry publication has already happened. It requires the signed/notarized launch path, release evidence and documentation boundary to be present and consistent.
