---
title: Release Quality
description: Release quality is based on evidence bundles, provenance, and contract checks.
---

Fe Reader release work is evidence-first.

The current release evidence scaffold records:

- release evidence JSON
- SBOM status or CycloneDX output
- provenance attestation scaffold
- signing readiness evidence
- release matrix and readiness reports
- schema and CI policy validation
- docs-site build and dependency-audit evidence for the public Starlight documentation surface
- release pipeline documentation that enumerates the explicit evidence artifacts

Public release channels must not silently skip provenance evidence. Development and nightly lanes may stay advisory while the tooling baseline is still being established.

## Frontier lanes

Bleeding-edge work belongs in governed lanes with feature gates, owners, rollback plans, and exit criteria. Advisory frontier checks must not become hard gates until an accepted baseline and ADR justify promotion.
