# Launch Limitations and Support

This document keeps public launch claims aligned with implemented evidence.

## Known Launch Limitations

- Stable desktop release remains blocked until signed artifacts, checksums and release evidence exist for each target platform.
- Mobile support is advisory: Android emulator and iOS simulator/binding checks exist, but mobile store packages are not launch-ready. The `mobile_public_launch` feature gate and [`docs/mobile-public-launch-readiness.md`](mobile-public-launch-readiness.md) track this status separately from desktop release readiness.
- ML/RAG features are deferred. Early waves intentionally keep local ML, RAG and local LLM features disabled. The `frontier_intelligence_preview` gate and [`docs/frontier-intelligence-governance.md`](frontier-intelligence-governance.md) define the future opt-in, privacy and promotion rules.
- cloud collaboration is deferred. Fe Reader remains local-first and must not silently upload, sync or collect analytics. The `opt_in_collaboration_sync` gate and [`docs/opt-in-collaboration-sync.md`](opt-in-collaboration-sync.md) define the future explicit opt-in, provider discovery, rollback and support-bundle exclusion rules.
- Registry manifests are present for several package surfaces, but package publication is deferred until credentials, artifacts, signatures and maintainer approval are available.

## Support Route

Use `SUPPORT.md` for normal support questions and `SECURITY.md` for vulnerabilities. Do not attach private PDFs, document text, credentials, support bundles, crash dumps with document paths, or exploit fixtures to public issues.

## Evidence Boundary

Capability claims must point to at least one of:

- CLI golden tests;
- schema validation;
- fuzz targets;
- visual regression fixtures;
- differential oracle results;
- platform contract smoke tests;
- performance scenario budgets;
- documented limitations.

Post-launch PDF baseline parity is tracked in `docs/pdf-baseline-parity-matrix.md` and validated by:

```bash
python3 scripts/pdf_baseline_parity_check.py
```

Opt-in collaboration and sync governance is checked with:

```bash
python3 scripts/opt_in_collaboration_sync_check.py
```

For stable release readiness, run:

```bash
python3 scripts/launch_qa_check.py
```
