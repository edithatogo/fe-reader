---
title: Release Pipeline
description: The release pipeline emits explicit evidence artifacts and publishes them as immutable release inputs.
---

Fe Reader release work is evidence-first. The pipeline is designed to make every release claim traceable to machine-readable artifacts under `target/release-evidence/`.

## Pipeline stages

1. Build and validate the release inputs.
2. Generate SBOM, provenance, signing readiness, matrix, and readiness evidence.
3. Upload the evidence bundle as a release artifact.
4. Use the bundle as the release record for downstream packaging or store submission.

## Explicit evidence artifacts

- `target/release-evidence/release-evidence.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/release-matrix.json`
- `target/release-evidence/sbom.cdx.json` or `target/release-evidence/sbom-status.json`
- `target/release-evidence/cargo-metadata.json`
- `target/release-evidence/provenance.json`
- `target/release-evidence/provenance-readiness.json`
- `target/release-evidence/signing-readiness.json`
- `target/release-evidence/desktop-distribution-publication.json`
- `target/release-evidence/enterprise-operations-readiness.json`
- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/launch-qa.json`

The GitHub release workflow uploads the complete `target/release-evidence/**` tree as the `release-evidence` artifact, so the release record stays inspectable after the job completes.

## Desktop distribution

Desktop package publication is represented by `packaging/desktop-distribution.yaml` and checked by `scripts/desktop_distribution_publication_check.py`. The gate keeps GitHub Releases, Homebrew, Winget, Chocolatey, Scoop, Flatpak, Snap and AUR in one of three explicit states: ready pending approval, published, or blocked with an exact reason.

The public release index remains <https://github.com/edithatogo/fe-reader/releases> until signed desktop artifacts are available. Registry credentials are never stored in the repository.

## Launch QA

Launch QA is checked by `scripts/launch_qa_check.py`. The command runs the focused release readiness checks, validates desktop install and registry documentation, confirms homepage metadata, checks support/security links and writes `target/release-evidence/launch-qa.json`.

## Docs-site relation

The public Starlight site documents the release-quality and provenance model that the workflow enforces. This keeps the external documentation in sync with the pipeline evidence contract instead of treating the site as separate marketing copy.
