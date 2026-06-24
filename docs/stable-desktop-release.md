# Stable Desktop Release

Fe Reader desktop release documentation is evidence-first. The stable desktop channel must not be treated as ready until signed artifacts, checksums, release evidence and maintainer approval exist for the target platform.
Marketing readiness is tracked in `docs/marketing-readiness.md`.
The usable stable bleeding-edge PDF reader contract is documented in `docs/usable-stable-bleeding-edge-pdf-reader-contract.md`.
The macOS public-quality signed/notarized launch contract is documented in `docs/macos-public-quality-signed-notarized-launch-contract.md`.
The Windows/Linux beta installers contract is documented in `docs/windows-linux-beta-installers-contract.md`.
The exhaustive PDF parity taxonomy and contracts are documented in `docs/exhaustive-pdf-parity-taxonomy-contract.md`.
The corpus/oracle evidence factory is documented in `docs/corpus-oracle-evidence-factory-contract.md`.

## Release Index

The canonical public release index is GitHub Releases:

- <https://github.com/edithatogo/fe-reader/releases>

Each desktop release should publish:

- platform artifacts for macOS, Windows and Linux;
- `SHA256SUMS` and signatures for every artifact;
- `target/release-evidence/release-artifact-inventory.json` proving expected artifacts and checksums exist;
- `target/release-evidence/stable-reader-readiness.json` proving the reader baseline matches the release claims;
- release notes;
- the `release-evidence` bundle from GitHub Actions.

## macOS

Expected stable artifacts:

- signed and notarized `.dmg`;
- optional signed `.pkg`;
- Homebrew Cask update after the signed artifact and checksum are available.

Verification:

```bash
shasum -a 256 -c SHA256SUMS
spctl --assess --type open --context context:primary-signature FeReader.dmg
```

## Windows

Expected stable artifacts:

- signed MSI or MSIX installer;
- optional signed portable zip;
- Winget, Chocolatey and Scoop updates after checksums are final.

Verification:

```powershell
Get-FileHash .\FeReader.msi -Algorithm SHA256
Get-AuthenticodeSignature .\FeReader.msi
```

## Linux

Expected stable artifacts:

- AppImage or tarball;
- package manifests for Flatpak, Snap, AUR or distro repositories when the corresponding review path is ready.

Verification:

```bash
sha256sum -c SHA256SUMS
```

## Launch QA

Run the launch QA aggregator before stable publication:

```bash
python3 scripts/launch_qa_check.py
python3 scripts/stable_reader_readiness_check.py
```

The command writes `target/release-evidence/launch-qa.json` and summarizes desktop packaging, registry publication state, enterprise operations readiness, release matrix checks, release readiness, accessibility smoke evidence, stable-reader readiness and public documentation alignment.

## PDF Capability Claims

Post-launch PDF parity is governed by `advanced_pdf_baseline` and does not block desktop stable launch. Capability claims must stay aligned with `docs/pdf-parity-registry.md`, `docs/pdf-parity-registry.json`, `docs/pdf-baseline-parity-matrix.md` and `docs/pdf-baseline-parity-matrix.json`.

Validate the claim boundary with:

```bash
python3 scripts/pdf_baseline_parity_check.py
```

## Related Evidence

- `docs/desktop-release-packaging.md`
- `docs/desktop-distribution-publication.md`
- `docs/enterprise-operations-readiness.md`
- `docs/pdf-baseline-parity-matrix.md`
- `packaging/registry-status.yaml`
- `packaging/desktop-distribution.yaml`
- `target/release-evidence/release-artifact-inventory.json`
- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/launch-qa.json`
- `docs/macos-public-quality-signed-notarized-launch-contract.md`
- `docs/windows-linux-beta-installers-contract.md`
- `docs/exhaustive-pdf-parity-taxonomy-contract.md`
- `docs/corpus-oracle-evidence-factory-contract.md`
