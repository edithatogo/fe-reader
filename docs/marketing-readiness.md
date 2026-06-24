# Marketing Readiness

Fe Reader uses evidence-backed marketing readiness levels. Public copy must not outpace the evidence in `docs/stable-reader-readiness.md`, `docs/pdf-parity-registry.md` or the release evidence bundle.
The permitted readiness labels are technical preview, public beta, stable desktop, mature stable and v2 roadmap.
The marketing claim governance contract is documented in `docs/marketing-claim-governance-contract.md`.
The v2 roadmap implementation foundation contract is documented in `docs/v2-roadmap-implementation-foundation-contract.md`.

## Readiness Levels

| Level | Meaning | Required evidence |
| --- | --- | --- |
| Technical preview | Early evaluator build with limited support scope. | Workspace tests, preview release notes, claim scanner pass, and documented limitations. |
| Public beta | Publicly downloadable, but still support-bounded and limited. | Preview/stable reader readiness reports, release QA, screenshots, and registry-linked capability evidence. |
| Stable desktop | The desktop release line is production-oriented. | Signed artifacts, checksums, release evidence, support policy, release notes, and claim scanner pass. |
| Mature stable | Stable desktop plus broad claim confidence and support maturity. | Stable desktop evidence plus parity registry coverage, support documentation, and maintainer approval. |
| v2 roadmap | Forward-looking roadmap claims only. | Public roadmap docs and explicit non-commitment to shipping status. |

## Claim Rules

- README, docs site, release notes and package metadata must not claim stable, beta or parity status unless the matching evidence exists.
- Homepage and docs copy should use neutral capability language.
- Limitations must stay visible near any download or publication copy.
- Claims that mention PDF parity must point at `docs/pdf-parity-registry.md` or `docs/pdf-baseline-parity-matrix.md`.

## Rollback

If claims overrun the evidence, remove the unsupported wording, link the limitation, and rerun the marketing claim scanner before publishing again.
