# Open Quality Signals Dashboard

## Purpose

A serious open PDF platform should publish evidence, not claims. v6 adds public quality signals that can be generated from CI and release candidates.

## Dashboards

```text
compatibility corpus coverage
visual regression status
performance budgets
security posture
supply-chain posture
accessibility coverage
installer/signing readiness
API compatibility
workflow pack validation
```

## Output files

```text
target/fe-quality/compatibility.json
target/fe-quality/performance.json
target/fe-quality/accessibility.json
target/fe-quality/security.json
target/fe-quality/release-readiness.json
target/fe-quality/dashboard.html
```

## Rule

No headline feature claim is accepted unless it links to a fixture, test, benchmark, compatibility report, or release note.

## Evidence

- `scripts/quality_dashboard_smoke.py` validates the public dashboard contract, schema, crate and privacy-preserving documentation.
- Dashboard output stays local-first and uses local artifacts only.
