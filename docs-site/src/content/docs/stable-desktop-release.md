---
title: Stable Desktop Release
description: Install, verification, launch QA and known limitation guidance for desktop releases.
---

Fe Reader stable desktop releases are evidence-gated. A release is not stable-ready until signed artifacts, checksums, release evidence, stable-reader readiness evidence and maintainer approval exist for each target platform. PDF capability claims remain tied to `docs/pdf-parity-registry.md`.
The usable stable bleeding-edge PDF reader contract is documented in `docs/usable-stable-bleeding-edge-pdf-reader-contract.md`.
The macOS public-quality signed/notarized launch contract is documented in `docs/macos-public-quality-signed-notarized-launch-contract.md`.
The Windows/Linux beta installers contract is documented in `docs/windows-linux-beta-installers-contract.md`.
The exhaustive PDF parity taxonomy and contracts are documented in `docs/exhaustive-pdf-parity-taxonomy-contract.md`.
The corpus/oracle evidence factory is documented in `docs/corpus-oracle-evidence-factory-contract.md`.
The reader/render/search/accessibility parity contract is documented in `docs/reader-render-search-accessibility-parity-contract.md`.
The advanced PDF family parity contract is documented in `docs/advanced-pdf-family-parity-contract.md`.
The marketing claim governance contract is documented in `docs/marketing-claim-governance-contract.md`.
The stable release cutover and registries contract is documented in `docs/stable-release-cutover-registries-contract.md`.

Marketing claims for this line are governed by `docs/marketing-readiness.md` and should remain aligned with the technical preview, public beta or stable desktop evidence level that actually exists.

## Install Sources

- GitHub Releases: <https://github.com/edithatogo/fe-reader/releases>
- Registry status: `packaging/registry-status.yaml`
- Desktop publication gate: `packaging/desktop-distribution.yaml`

Package registries become authoritative only after their signed artifact and checksum are published.

## Verification

Every desktop release should include `SHA256SUMS`, signatures and a `release-evidence` artifact.

```bash
sha256sum -c SHA256SUMS
python3 scripts/launch_qa_check.py
python3 scripts/stable_reader_readiness_check.py
```

macOS releases also require Developer ID signing and notarization evidence. Windows releases require Authenticode evidence. Linux package channels require the relevant artifact checksum and repository or store review evidence.

## Known launch limitations

- Mobile store packages are deferred.
- ML/RAG features are deferred.
- Cloud collaboration is deferred.
- Reader baseline launch claims require `target/release-evidence/stable-reader-readiness.json`.
- Registry publication remains blocked until signed artifacts, checksums, credentials and maintainer approval are available.

## Support and Security

Use `SUPPORT.md` for support routing and `SECURITY.md` for vulnerability reporting. Do not post private PDFs, document text, credentials or support bundles in public issues.
