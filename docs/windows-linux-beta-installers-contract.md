# Windows/Linux Beta Installers Contract

This Windows/Linux beta installers contract binds the prerelease installer path to documented evidence.

Fe Reader meets the Windows/Linux beta installer contract when the beta release path for those platforms is defined, documented and backed by prerelease installer evidence.

## Contract

- Windows beta installers are defined in the packaging matrix and release-channel model.
- Linux beta installers are defined in the packaging matrix and release-channel model.
- Beta installer publication remains prerelease-only until the public release gate is intentionally opened.
- Pre-release installer placeholders exist for the beta installer path and are recorded in release evidence.
- Public registry publication remains blocked until signed artifacts, checksums and approval exist.

## Required evidence

- `packaging/package-matrix.yaml`
- `packaging/release-channels.yaml`
- `packaging/desktop-distribution.yaml`
- `target/release-artifacts/prerelease-placeholders/fe-reader-0.1.0-preview.1-release-artifact-inventory.json`
- `target/release-evidence/release-matrix.json`
- `target/release-evidence/desktop-packaging-signing.json`
- `target/release-evidence/desktop-distribution-publication.json`
- `target/release-evidence/release-readiness.json`

## Installer surfaces

- Windows: MSI, MSIX and portable ZIP are the beta-capable desktop installer forms.
- Linux: AppImage, DEB, RPM and tarball are the beta-capable desktop installer forms.

This contract does not claim public stable release or registry publication. It binds beta installer readiness to the documented packaging matrix and prerelease installer evidence already present in the repository.
