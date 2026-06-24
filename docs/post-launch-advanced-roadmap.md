# Post-launch Advanced Roadmap

Desktop stable launch remains governed by tracks AS through AW. The tracks below are post-launch continuation work and do not block desktop stable launch unless they uncover a launch-critical defect.

## Sequencing

1. `track-AY-post-launch-pdf-baseline-parity` - harden baseline PDF reading, rendering, editing, forms, metadata, redaction and conversion evidence.
2. `track-BC-rendering-performance-promotion` - promote rendering/performance work only when CPU and optional GPU evidence meets release budgets.
3. `track-AZ-mobile-public-launch` - progress Android and iOS public launch separately from desktop stable.
4. `track-BD-ecosystem-integrations-marketplace` - expand SDK, plugin and integration surfaces after safety gates remain intact.
5. `track-BB-opt-in-collaboration-sync` - explore local-first collaboration and sync only as explicit opt-in work.
6. `track-BA-frontier-intelligence-governance` - evaluate frontier intelligence after deterministic workflow features and privacy gates are mature.
7. `track-BO-v2-roadmap-implementation-foundation` - define v2 sequencing and gates only after a usable stable bleeding-edge reader release is published.

## Governance

- ML/RAG remains disabled by default.
- Frontier lanes must include a feature gate, owner, rollback plan and exit criteria.
- cloud collaboration is opt-in and must never silently upload, sync, phone home or collect analytics.
- Mobile public launch remains separate from desktop stable release readiness.
- Advanced tracks must preserve `fe_reader_core` purity and the mutation pipeline.
- v2 roadmap work must stay behind stable launch gates and marketing readiness checks.

## Validation

Run:

```bash
python3 scripts/advanced_roadmap_check.py
```

The script validates track metadata, registry entries, non-blocking launch status and governance tokens.
