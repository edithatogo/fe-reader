# Marketing Claim Governance Contract

Fe Reader meets the marketing claim governance contract when public copy stays inside the evidence-backed readiness levels and the claim scanner passes.

## Contract

- README, docs site, release notes and package metadata must not claim stable, beta or parity status unless the matching evidence exists.
- Public copy must stay aligned with `docs/marketing-readiness.md`.
- PDF capability claims must keep pointing to the parity registry or baseline matrix.
- The current release line remains a technical preview until the separate marketing readiness gate passes.

## Required evidence

- `docs/marketing-readiness.md`
- `scripts/marketing_claim_evidence_governance_check.py`
- `target/release-evidence/marketing-readiness.json`
- `docs/stable-reader-readiness.md`
- `docs/pdf-parity-registry.md`
- `docs/pdf-baseline-parity-matrix.md`

## Readiness levels

- technical preview
- public beta
- stable desktop
- mature stable
- v2 roadmap

This contract does not claim the product is marketing-ready. It binds the current public copy to the evidence model and claim scanner already in the repository.
