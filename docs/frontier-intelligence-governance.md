# Frontier Intelligence Governance

The `frontier_intelligence_preview` feature gate covers optional local ML, RAG
and intelligent assistance work after deterministic extraction, search and
workflow features are mature. It is disabled by default and does not block desktop stable launch.

This track is governance only. It does not enable model inference, RAG, local
LLM features, remote telemetry or cloud-only assistance.

## Defaults

| Control | Default |
| --- | --- |
| Feature gate | disabled |
| Network access | disabled |
| Model downloads | disabled |
| Telemetry | disabled |
| Mutation | no high-risk automatic mutation |
| Evaluation data | synthetic or public only |

no private document text, private prompts, private support bundles, credentials
or private file paths may be used in frontier evaluation fixtures.

## User Controls

Any future preview needs:

- explicit opt-in UI;
- explicit opt-in CLI;
- disable switch;
- purge control for local models;
- purge control for local indexes;
- visible model provenance before use.

The preview must fall back to deterministic extraction, deterministic search and
workflow packs when disabled.

## Promotion Evidence

Promotion beyond advisory preview requires:

- privacy review;
- security review;
- synthetic or public evaluation report;
- resource-budget report;
- explicit opt-in UI/CLI evidence;
- ADR approval;
- rollback plan.

The machine-readable governance snapshot is
[`contracts/snapshots/frontier/frontier-intelligence-governance.preview.json`](../contracts/snapshots/frontier/frontier-intelligence-governance.preview.json).
The privacy-safe eval manifest is
[`fixtures/frontier/evaluation/manifest.json`](../fixtures/frontier/evaluation/manifest.json).

Run:

```bash
python3 scripts/frontier_intelligence_governance_check.py
```

## Rollback

The rollback path is to disable `frontier_intelligence_preview`, purge local
model caches, purge local vector indexes and fall back to deterministic
extraction/search/workflows.
