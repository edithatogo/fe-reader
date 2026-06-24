# Launch Limitations and Support

This document keeps public launch claims aligned with implemented evidence and `docs/marketing-readiness.md`.

## Known Launch Limitations

- Stable reader marketing remains gated by `scripts/stable_reader_readiness_check.py`; broad marketing claims must not outrun reader navigation, accessibility, search, performance and release evidence.
- The usable stable bleeding-edge PDF reader contract is documented in `docs/usable-stable-bleeding-edge-pdf-reader-contract.md`.
- Stable desktop release remains blocked until signed artifacts, checksums and release evidence exist for each target platform.
- The exhaustive PDF parity taxonomy and contracts are documented in `docs/exhaustive-pdf-parity-taxonomy-contract.md`.
- The corpus/oracle evidence factory is documented in `docs/corpus-oracle-evidence-factory-contract.md`.
- The marketing claim governance contract is documented in `docs/marketing-claim-governance-contract.md`.
- The stable release cutover and registries contract is documented in `docs/stable-release-cutover-registries-contract.md`.
- The v2 roadmap implementation foundation contract is documented in `docs/v2-roadmap-implementation-foundation-contract.md`.
- Mobile support is advisory: Android emulator and iOS simulator/binding checks exist, but mobile store packages are not launch-ready. The `mobile_public_launch` feature gate and [`docs/mobile-public-launch-readiness.md`](mobile-public-launch-readiness.md) track this status separately from desktop release readiness.
- ML/RAG features are deferred. Early waves intentionally keep local ML, RAG and local LLM features disabled. The `frontier_intelligence_preview` gate and [`docs/frontier-intelligence-governance.md`](frontier-intelligence-governance.md) define the future opt-in, privacy and promotion rules.
- cloud collaboration is deferred. Fe Reader remains local-first and must not silently upload, sync or collect analytics. The `opt_in_collaboration_sync` gate and [`docs/opt-in-collaboration-sync.md`](opt-in-collaboration-sync.md) define the future explicit opt-in, provider discovery, rollback and support-bundle exclusion rules.
- GPU and expanded rendering performance claims remain advisory. The `rendering_performance_promotion` gate and [`docs/rendering-performance-promotion.md`](rendering-performance-promotion.md) require CPU fallback, visual regression, differential oracle, budget and platform evidence before promotion.
- SDK, plugin, workflow-pack and marketplace publication remain deferred. The `ecosystem_integrations_marketplace` gate and [`docs/ecosystem-integrations-marketplace.md`](ecosystem-integrations-marketplace.md) require compatibility snapshots, provenance, support/security links and maintainer approval before publication claims.
- Registry manifests are present for several package surfaces, but package publication is deferred until credentials, artifacts, signatures and maintainer approval are available.
- v2 roadmap work is deferred until the stable reader baseline, marketing readiness and installable release evidence are all present.

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

Post-launch PDF parity is tracked in `docs/pdf-parity-registry.md` and the nested `docs/pdf-baseline-parity-matrix.md`, and validated by:

```bash
python3 scripts/pdf_parity_registry_check.py
python3 scripts/pdf_baseline_parity_check.py
```

Stable reader readiness is tracked in `docs/stable-reader-readiness.md` and validated by:

```bash
python3 scripts/stable_reader_readiness_check.py
```

The stable-reader readiness gate is the reader baseline checkpoint for public release claims.

Opt-in collaboration and sync governance is checked with:

```bash
python3 scripts/opt_in_collaboration_sync_check.py
```

Rendering performance promotion governance is checked with:

```bash
python3 scripts/rendering_performance_promotion_check.py
```

Ecosystem integration and marketplace governance is checked with:

```bash
python3 scripts/ecosystem_integrations_marketplace_check.py
```

For stable release readiness, run:

```bash
python3 scripts/launch_qa_check.py
```

The v2 roadmap implementation foundation is validated by:

```bash
python3 scripts/v2_roadmap_implementation_foundation_check.py
```
