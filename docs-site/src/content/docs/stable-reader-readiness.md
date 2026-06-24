---
title: Stable Reader Readiness
description: Evidence-backed baseline for the stable Fe Reader reader experience.
---

Fe Reader stable reader readiness is evidence-gated. It tracks the reader-first baseline, professional workflow boundary, and the release evidence needed before broad marketing.

Broad marketing claims still require the separate marketing readiness gate in `docs/marketing-readiness.md`.

## Evidence

- `scripts/stable_reader_readiness_check.py`
- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/launch-qa.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/stable-release-evidence.json`
- `docs/pdf-parity-registry.md`
- `docs/pdf-baseline-parity-matrix.md`
- `docs/launch-limitations-support.md`

## Reader baseline

- Open local PDFs from CLI and preview entry points. See `docs/pdf-parity-registry.md` for the exact claim ids.
- Inspect metadata, search deterministically and surface safe-open diagnostics. See `docs/pdf-parity-registry.md`.
- Expose keyboard and accessibility evidence for the reader baseline.

## Boundaries

Professional workflows remain policy-gated. Stable marketing still requires signed artifacts, checksums, release evidence and registry approval.
