# Agent Instructions

## Product identity

Fe Reader is a local-first PDF workflow platform with strong privacy, verification, metadata transparency, automation safety and cross-platform native integration. It should meet industry baseline PDF capabilities without framing the product as a clone of any vendor.

## Non-negotiable architecture

```text
fe_reader_core
  pure document/workflow core
  no UI, no platform, no renderer, no AI, no MCP, no plugin runtime

adapters
  rendering, platform, app integrations, MCP, plugins, web, native shell
```

## Mutation pipeline

All writes use:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

No shortcut APIs.

## Automation safety

COM, AppleScript, D-Bus, MCP, mobile intents, browser extensions and plugins are read-only by default. Mutations require:

- document hash match
- patch plan ID
- policy evaluation
- approval token or interactive confirmation
- audit receipt emission

## Testing expectations

Every core feature needs at least one of:

- CLI golden test
- schema validation
- fuzz target
- visual regression fixture
- differential test against reference renderer/tool
- platform contract smoke test
- performance scenario budget

## Dependency rule

Use latest verified stable dependencies. Bleeding-edge or git dependencies are allowed only in frontier lanes with an owner, feature gate, rollback plan and exit criteria.


## v5 agent instructions

- Treat malformed PDFs as expected input, not an edge case. Use the safe-open and recovery contracts before attempting repair.
- Any claim that Fe Reader can read, render, convert, redact or preserve a class of PDFs must be backed by corpus fixtures, differential oracle results or a documented limitation.
- API changes to Rust crates, UniFFI, CLI, MCP, plugins, COM, AppleScript, D-Bus, Android intents or iOS App Intents require an API compatibility note and versioning decision.
- Do not add git dependencies, forks, nightly features, GPU experiments, ML experiments or native platform hooks without feature flags and governance metadata.
- Use the PDF Engineering Lab contracts to expose diagnostics; do not leak low-level object mutation APIs directly to normal workflow packs.


## v6 implementation addendum

Before implementing feature code, review `docs/v6-coding-agent-start-here.md`. The project now requires typed Document IR, transformation passes, job scheduling, power/thermal budgets, UI accessibility/E2E checks, active-content quarantine, PDF revision/time-machine support, source pipeline workflows, and public quality signals. Do not add these as ad-hoc helpers; implement them through the contracts and Conductor tracks added in v6.

Do not enable local ML, RAG or local LLM features in early waves. Deterministic extraction, search, transformations, metadata, workflow packs, redaction verification, platform integration, testing, performance and release quality remain the first priorities.


## v6 agent behaviour

- Prefer user-centred, accessible flows over hidden expert-only functionality.
- Every UI feature should have a keyboard path and, where practical, CLI/automation parity.
- Do not introduce source-linked authoring, optimisation, collaboration or cache features without adding contracts and fixtures.
- Treat cache entries, collaboration packets and quality signals as privacy-sensitive.
- Do not silently upload, sync, phone home, or collect analytics. Diagnostics and quality signals are opt-in and local-first by default.
- If a source-linked workflow uses Typst, Quarto, Tectonic, LaTeX, Pandoc or external converters, implement it as a provider with capability discovery and clear failure modes.

## v7 agent rule

This package is implementation-first. Do not broaden requirements unless a Wave 0 implementation task is blocked by a missing contract, schema or acceptance criterion. Prefer a small compiling PR over a larger speculative design update.


## v8 agent instruction

Your first task is not to add features. Your first task is to make the v8 Wave 0 workspace compile and pass its contract tests. Use `docs/v8-coding-agent-start-here.md` and `docs/wave0-contract-acceptance-tests.md`.


## v8 agent instruction

Your first task is not to add features. Your first task is to make the v8 Wave 0 workspace compile and pass its contract tests. Use `docs/v8-coding-agent-start-here.md` and `docs/wave0-contract-acceptance-tests.md`.

## v9 agent rules

- Never add UI/platform/renderer/plugin/ML dependencies to `fe_reader_core`.
- Never convert an advisory frontier lane into a hard gate without an ADR and accepted baseline.
- Never make a mutating automation surface skip transaction journaling or user/policy approval.
- Keep GitHub Actions permissions minimal and explicit.
- When creating CI workflows, include `timeout-minutes`, `concurrency`, and explicit `permissions`.
