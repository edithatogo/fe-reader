# v2 Roadmap Implementation Foundation Contract

Fe Reader meets the v2 roadmap implementation foundation contract only after a usable, stable, bleeding-edge PDF reader release has been published and the post-launch v2 sequencing remains evidence-linked.

## Contract

- v2 work begins only after the stable reader baseline, marketing readiness and release evidence remain green.
- The v2 roadmap foundation stays documented in `docs/v2-roadmap-foundation.md`.
- The post-launch sequencing remains documented in `docs/post-launch-advanced-roadmap.md`.
- The v2 foundation track `track-BO-v2-roadmap-implementation-foundation` stays defined as post-launch work and does not weaken the stable launch baseline.
- The first v2 implementation tranche must preserve the core architecture, mutation safety, evidence-first release gates and feature-gated rollout discipline.

## Required evidence

- `docs/v2-roadmap-foundation.md`
- `docs/post-launch-advanced-roadmap.md`
- `docs/stable-reader-readiness.md`
- `docs/stable-desktop-release.md`
- `docs/marketing-readiness.md`
- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/stable-release-evidence.json`
- `target/release-evidence/launch-qa.json`
- `target/release-evidence/advanced-roadmap.json`

## Implementation foundation

- Keep v2 sequencing behind stable launch gates.
- Use contracts, fixtures and release evidence before promoting any v2 tranche.
- Preserve `fe_reader_core` purity and the mutation pipeline.
- Keep v2 expansion feature-gated until the next tranche is explicitly approved.

This contract does not claim that v2 feature work is already complete. It only establishes the post-launch foundation and the evidence boundary that allows v2 implementation to proceed safely.
