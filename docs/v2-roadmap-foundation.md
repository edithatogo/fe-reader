# v2 Roadmap Foundation

Fe Reader v2 work begins only after a usable, stable, bleeding-edge PDF reader release is published. The purpose of this document is to keep v2 expansion from weakening the stable launch baseline and to keep the post-launch advanced roadmap aligned with that boundary.

## Entry Gates

- Stable reader baseline evidence passes.
- A real public-quality installable artifact exists for at least one desktop platform.
- Marketing readiness and parity governance pass.
- Stable release support and regression gates remain active.

## v2 Themes

| Theme | Focus | Default gate |
| --- | --- | --- |
| Advanced PDF parity expansion | Finish post-launch families that remain limited or oracle-backed. | feature-gated |
| Workflow packs | Extend domain workflows without weakening mutation safety. | feature-gated |
| Source-linked authoring | Build provider-backed pipelines for Typst, Quarto, LaTeX and Pandoc. | feature-gated |
| Opt-in collaboration | Keep cloud sync, review packets and sharing local-first and explicit. | feature-gated |
| Marketplace | Expand SDK, plugin and publication surfaces after safety review. | feature-gated |
| Local intelligence | Keep embeddings, extraction assistance and grounded Q&A optional. | feature-gated |
| Enterprise operations | Strengthen policy, deployment and supportability without broadening defaults. | feature-gated |
| Mobile maturity | Continue Android and iOS maturity behind their own gates. | feature-gated |

## Sequencing

1. Lock stable support gates and regressions first.
2. Publish the roadmap and evidence links.
3. Implement the highest-value stable-adjacent tranche as a small, reviewable track.
4. Keep unsupported claims out of release notes and homepage copy.

## Exit Criteria

- v2 roadmap is public and evidence-linked.
- At least one implementation tranche has contracts, fixtures, tests and release evidence.
- Stable maintenance still passes the release gates.

## Related Evidence

- `docs/stable-reader-readiness.md`
- `docs/marketing-readiness.md`
- `docs/pdf-parity-registry.md`
- `docs/post-launch-advanced-roadmap.md`
- `docs/v2-roadmap-implementation-foundation-contract.md`
- `scripts/advanced_roadmap_check.py`
