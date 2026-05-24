# Implementation Prompt for Coding Agent

You are implementing Fe Reader, a local-first cross-platform PDF workflow platform. You must treat this repository as an execution-ready specification package, not as a loose set of ideas.

## First files to read

Read these in order:

1. `README.md`
2. `AGENTS.md`
3. `conductor/waves.yaml`
4. `conductor/tracks.md`
5. `contracts/README.md`
6. `docs/architecture.md`
7. `docs/security-threat-model.md`
8. `docs/performance-engineering.md`
9. `docs/compatibility-corpus-governance.md`
10. `docs/release-operations-updates.md`

## Implementation rules

- Do not put UI, Tauri, PDFium, MCP, platform APIs, Extism, Wasmtime, Candle, Burn, COM, AppleScript, D-Bus, or web APIs into `fe_reader_core`.
- Implement deterministic functionality before ML. ML, local LLMs, embeddings and RAG are Wave 6+ only.
- Implement a CLI path as early as possible so every operation can be tested outside the UI.
- Treat all external automation surfaces as untrusted. Read-only by default; mutations require `PatchPlan` and explicit approval.
- Secure redaction must use full sanitising rewrite and verification. Do not implement secure redaction as an annotation-only black box.
- Renderer crashes must not corrupt documents. Rendering and mutation are separate concerns.
- The project is local-first. Do not add cloud sync. External storage integrations must be explicit providers or OS-level document providers.
- Use upstream libraries first; fork only according to `third_party/fork-policy.yaml`.
- Performance budgets are product requirements, not optional polish.

## Starting command

Begin with:

```text
/conductor:status
/conductor:implement Wave 0 Foundation Contracts, CLI Harness, Security Policy and Corpus Baseline
```

## Phase exit rule

At the end of every phase, run:

```bash
scripts/conductor_phase_gate.sh --phase <phase-id> --auto-fix
```

Formatting and simple lint fixes may be automatic. Security, architecture, redaction, policy, release-signing, compatibility and accepted performance failures must not be auto-bypassed.


## v5 implementation priority

After reading the Wave 0 foundation files, also read:

```text
docs/pdf-engineering-lab.md
docs/pdf-repair-recovery.md
docs/differential-testing-oracles.md
docs/api-compatibility-governance.md
docs/reproducible-builds-provenance.md
docs/color-prepress-fonts-advanced.md
docs/maintainership-rfc-governance.md
```

Do not implement high-risk PDF mutation, public SDK changes, signing/export features, renderer changes, or installer release steps without creating or updating the matching contract, test oracle, compatibility report and phase-gate entry.

Prefer existing upstream projects first. Fork only under `third_party/fork-policy.yaml`, and only with a named owner, rebase cadence, exit criteria and upstream contribution plan.


## v6 implementation addendum

Before implementing feature code, review `docs/v6-coding-agent-start-here.md`. The project now requires typed Document IR, transformation passes, job scheduling, power/thermal budgets, UI accessibility/E2E checks, active-content quarantine, PDF revision/time-machine support, source pipeline workflows, and public quality signals. Do not add these as ad-hoc helpers; implement them through the contracts and Conductor tracks added in v6.

Do not enable local ML, RAG or local LLM features in early waves. Deterministic extraction, search, transformations, metadata, workflow packs, redaction verification, platform integration, testing, performance and release quality remain the first priorities.


## v6 implementation priority

After the v5 foundation files, read the v6 usability, authoring, optimisation and collaboration files:

```text
docs/final-v6-improvements.md
docs/ux-accessibility-human-factors.md
docs/source-linked-authoring-workflows.md
docs/pdf-optimization-linearization-compression.md
docs/content-addressed-cache-workspace.md
docs/offline-collaboration-review-packets.md
docs/toolchain-optimization-experimental-lanes.md
docs/privacy-preserving-quality-signals.md
docs/user-docs-training-community.md
```

Do not ship a feature solely because it compiles. Each feature needs a UX path, a CLI or automation path where appropriate, a corpus fixture or test strategy, and a privacy/security/performance statement. Advanced expert tools must have safe defaults, warning copy, undo/transaction support and receipts where the operation changes a document.

## v7 instruction

Before adding new features, make the Wave 0 scaffold compile and pass bootstrap checks. Prioritise `Cargo.toml`, crate boundaries, schema validation, transaction journal, error taxonomy and CLI doctor/inspect commands. Use `docs/implementation-stop-rule.md` to reject ungrounded scope expansion.

## v8 execution instruction

Do not expand product scope unless a Wave 0 contract is missing. Start by compiling and testing the concrete v8 workspace contracts. Preserve these invariants:

1. `fe_reader_core` remains free of PDF parser, renderer, UI, platform, plugin, MCP, and ML dependencies.
2. Every mutating route must pass through `OperationIntent -> PatchPlan -> TransactionJournal -> Apply -> Verify -> Receipt`.
3. Redaction, signing, export, plugin execution, external-tool execution, and automation mutation are high-risk actions unless a stricter policy says otherwise.
4. The CLI is the first executable integration surface; it must stay useful for tests and coding agents.
5. If a new feature is requested, add or update a contract/test first, not UI code.

## v8 execution instruction

Do not expand product scope unless a Wave 0 contract is missing. Start by compiling and testing the concrete v8 workspace contracts. Preserve these invariants:

1. `fe_reader_core` remains free of PDF parser, renderer, UI, platform, plugin, MCP, and ML dependencies.
2. Every mutating route must pass through `OperationIntent -> PatchPlan -> TransactionJournal -> Apply -> Verify -> Receipt`.
3. Redaction, signing, export, plugin execution, external-tool execution, and automation mutation are high-risk actions unless a stricter policy says otherwise.
4. The CLI is the first executable integration surface; it must stay useful for tests and coding agents.
5. If a new feature is requested, add or update a contract/test first, not UI code.

## v9 repository enforcement addendum

Before feature implementation, enforce the v9 contracts:

- Run `python3 scripts/strict_contract_check.py` and `python3 scripts/ci_policy_check.py`.
- Treat `.github/workflows/*.yml` as source-controlled policy, not examples.
- Do not bypass OperationIntent -> PatchPlan -> TransactionJournal -> Apply -> Verify -> Receipt.
- Keep bleeding-edge experiments in frontier lanes and outside `fe_reader_core`.
- Release artifacts must include SBOM/provenance evidence before public distribution.
