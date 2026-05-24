# Gemini / Conductor Entry Instructions

This repository contains a Conductor-ready specification package.

Read in this order:

1. `conductor/product.md`
2. `conductor/tech-stack.md`
3. `conductor/waves.yaml`
4. `conductor/tracks.md`
5. `conductor/review-policy.md`
6. `docs/architecture.md`
7. `contracts/README.md`

Then implement Wave 0. Keep all code compileable after each phase.


## v6 implementation addendum

Before implementing feature code, review `docs/v6-coding-agent-start-here.md`. The project now requires typed Document IR, transformation passes, job scheduling, power/thermal budgets, UI accessibility/E2E checks, active-content quarantine, PDF revision/time-machine support, source pipeline workflows, and public quality signals. Do not add these as ad-hoc helpers; implement them through the contracts and Conductor tracks added in v6.

Do not enable local ML, RAG or local LLM features in early waves. Deterministic extraction, search, transformations, metadata, workflow packs, redaction verification, platform integration, testing, performance and release quality remain the first priorities.
