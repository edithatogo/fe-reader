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

The GitHub release workflow uploads the complete `target/release-evidence/**` tree as the `release-evidence` artifact, so the release record stays inspectable after the job completes.

## Docs-site relation

The public Starlight site documents the release-quality and provenance model that the workflow enforces. This keeps the external documentation in sync with the pipeline evidence contract instead of treating the site as separate marketing copy.
