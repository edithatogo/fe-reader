# Desktop Distribution Publication

Desktop publication is intentionally manual-gated. The repository can validate the release inputs and registry manifests, but it must not publish to GitHub Releases or third-party registries without maintainer approval.
The macOS public-quality signed/notarized launch contract is documented in `docs/macos-public-quality-signed-notarized-launch-contract.md`.

## Release Gate

Before any stable desktop package is published:

1. Build the desktop artifacts listed in `packaging/package-matrix.yaml`.
2. Produce detached SHA-256 files beside each artifact.
3. Run `python3 scripts/release_artifact_inventory_check.py` to record which artifacts and checksums are present.
4. Run the release workflow or the local release evidence checks.
5. Confirm `target/release-evidence/stable-release-evidence.json` passes for the release channel.
6. Confirm `target/release-evidence/desktop-distribution-publication.json` passes.
7. Obtain explicit maintainer approval for publication.

## Local Validation

```bash
python3 scripts/desktop_distribution_publication_check.py
python3 scripts/release_artifact_inventory_check.py
```

For a stable publication rehearsal:

```bash
FE_RELEASE_CHANNEL=stable python3 scripts/desktop_distribution_publication_check.py
```

Stable mode fails closed unless artifact-backed package manifests and release evidence are present.

## GitHub Release Commands

Use these only after the gate is green and the maintainer has approved publication:

```bash
gh release create v0.1.0-preview.1 \
  --repo edithatogo/fe-reader \
  --title "Fe Reader 0.1.0 Preview 1" \
  --notes-file docs/release-notes/v0.1.0-preview.1.md \
  target/release-artifacts/*
```

Manual fallback:

1. Open <https://github.com/edithatogo/fe-reader/releases/new>.
2. Select the validated tag.
3. Paste the checked release notes.
4. Upload the signed artifacts and checksums.
5. Stop before publishing until maintainer approval is recorded.

## Registry Surfaces

| Surface | Manifest | Current state |
| --- | --- | --- |
| GitHub Releases | `packaging/desktop-distribution.yaml` | blocked until signed desktop artifacts and checksums exist |
| Homebrew | `packaging/macos/homebrew/fe-reader.rb` | blocked until signed and notarized DMG exists |
| Winget | `packaging/windows/winget/FeReader.yaml` | blocked until signed Windows installer exists |
| Chocolatey | `packaging/windows/chocolatey/fe-reader.nuspec` | blocked until signed Windows artifact exists |
| Scoop | `packaging/windows/scoop/fe-reader.json` | blocked until signed portable zip and checksum exist |
| Flatpak | `packaging/linux/flatpak/org.fereader.FeReader.yml` | blocked until desktop app artifact and Flathub review exist |
| Snap | `packaging/linux/snap/snapcraft.yaml` | blocked until snap artifact and store review exist |
| AUR | `packaging/linux/aur/PKGBUILD` | blocked until source tarball and checksums exist |

No registry credentials, API tokens or store secrets belong in repository files.
