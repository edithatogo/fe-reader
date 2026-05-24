# Fe Reader Execution-Ready Specification Package v6

Fe Reader is a local-first, cross-platform PDF workflow platform built around a headless Rust core. This package is designed to be extracted into a repository and handed to a coding model/agent runner as its implementation source of truth.

The product is **not** a legal-only application. Legal/affidavit automation is one workflow pack among many. The platform supports reading, editing, metadata control, conversion, workflow automation, deterministic search, accessibility, platform integration, app integration, performance engineering, release engineering, security isolation, compatibility testing, and late-stage optional local intelligence.

## How to use this package

1. Extract this folder into a new repository.
2. Read `IMPLEMENTATION_PROMPT.md`, `AGENTS.md`, `conductor/waves.yaml`, and `contracts/README.md` first.
3. Copy sample crate manifests from `crates/**/Cargo.toml.sample` into their final names when creating the workspace.
4. Install Conductor and ask the coding model to begin with Wave 0.
5. Use `scripts/conductor_phase_gate.sh --phase <phase-id> --auto-fix` after every phase.

## Core principle

Every potentially destructive operation must flow through:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

This applies equally to UI actions, CLI commands, MCP calls, plugins, COM/AppleScript/D-Bus automation, browser extension calls, and mobile intents.

## v4 additions

v4 incorporates the performance/bleeding-edge layer from v3 and adds the final operating-excellence layer:

```text
Security isolation and process sandboxing
Compatibility corpus governance and visual regression testing
Deterministic search/indexing without early ML dependency
Text, font, Unicode, CJK, RTL and accessibility contracts
Release operations, signed updates, SBOMs and provenance
Enterprise deployment policy and privacy/diagnostics governance
Developer ecosystem, SDK and API versioning
```

## Package map

```text
docs/                  Product, architecture, platform, metadata, conversion, distribution, security, performance and QA plans
contracts/             Rust traits, automation surfaces, platform contracts, MCP, CLI, web, policy and search contracts
schemas/               JSON Schemas for operations, patch plans, workflows, metadata, plugins, packaging, policy and test fixtures
conductor/             Conductor context files, waves, tracks, review policy and automation manifest
.agents/skills/        Review skill for phase gates
crates/                Cargo.toml samples for the intended Rust workspace
scripts/               Phase gate, architecture compliance, schema validation, release, security, corpus and performance checks
templates/             Workflow packs, signature templates, redaction recipes, metadata profiles and enterprise policies
packaging/             Distribution manifests/checklists for Windows, macOS, Linux, Android, iOS, NuGet and update channels
third_party/           Fork/submodule policy and library contribution strategy
benchmarks/            Performance budgets, scenarios, reports and benchmark harness plan
ci/                    Sample CI jobs including performance, compatibility, security and release readiness runs
```

## What is intentionally deferred

Local LLMs, RAG and ML-heavy NLP are not early priorities. The architecture includes late-wave contracts for local intelligence, but Waves 0-5 focus on deterministic parsing, rendering, editing, metadata, secure redaction, workflow packs, OS integration, installers, external integrations, deterministic search and release quality. Wave 6 remains optional frontier work.


## v5 additions

v5 adds the engineering-lab and governance layer that should exist before large-scale implementation begins:

```text
PDF Engineering Lab: object tree, content stream, xref, font/glyph, colour and geometry inspection
Repair and safe-open modes for malformed or hostile PDFs
Differential testing against external oracles such as qpdf, veraPDF, PDFium, Poppler, MuPDF and Ghostscript
API/ABI compatibility governance for Rust crates, UniFFI bindings, CLI, MCP, automation and plugin contracts
Reproducible-build and release-evidence planning, including SBOM, signed attestations and SLSA-aligned provenance
Advanced colour, prepress and font fidelity planning for PDF/X, ICC, overprint, separations and font subsetting
Feature-flag and runtime-capability governance for frontier features
Maintainer/RFC/ADR governance so architecture changes are intentional and reviewable
```

The v5 rule remains: stable, deterministic and fuzzable in the core; frontier and bleeding-edge work at the edges behind feature gates, benchmarks, compatibility reports and explicit owner review.


# v6 additions

Version 6 adds the final product-and-engine hardening layer before coding begins:

```text
Document IR and transformation compiler
Typed job scheduler with progress, cancellation, resumability and resource limits
Power, thermal and battery budgets for mobile and laptop use
UI design system, WCAG 2.2 accessibility and end-to-end automation strategy
PDF Time Machine and Active Content Firewall
Source pipeline / notebook mode for Markdown, Quarto, Typst, LaTeX and Pandoc workflows
Open benchmark dashboard and public quality signals
Contributor/community governance, DCO/CLA decision point and issue/PR templates
Configuration and policy engine for user, enterprise and automation policy
Docs-as-code and coding-agent evaluation harness
```

Implementation agents should now start with:

1. `docs/v6-coding-agent-start-here.md`
2. `IMPLEMENTATION_PROMPT.md`
3. `AGENTS.md`
4. `conductor/waves.yaml`
5. `conductor/tracks.md`
6. `docs/engine-ir-and-transformation-pipeline.md`
7. `docs/ui-e2e-accessibility-design-system.md`
8. `docs/job-scheduler-power-thermal.md`
9. `docs/pdf-time-machine-active-content-firewall.md`

The v6 principle is: **make the core boring, typed and testable; make the edges ambitious, observable and reversible.**


## v6 additions

v6 adds the user-facing and ecosystem-readiness layer that should guide implementation after the core operating system is in place:

```text
Human-centred UX and accessibility certification plan
Source-linked authoring workflows for Typst, Quarto, LaTeX/Tectonic and Pandoc
PDF optimisation, compression, linearisation and size/performance receipts
Content-addressed workspace, cache and offline collaboration packets
Toolchain optimisation and experimental lanes for build speed and release performance
Privacy-preserving quality signals and support diagnostics
Documentation, onboarding, community and adoption operating model
```

The v6 rule is: advanced capabilities must be discoverable, measurable and explainable. Fe Reader should not merely expose expert PDF functions; it should make those functions safe for ordinary users and scriptable for expert users.

## v7 implementation-first note

v7 adds a real Cargo workspace scaffold and minimal crate stubs. The next step is not another planning pass. The next step is Wave 0 implementation:

```bash
bash scripts/wave0_bootstrap_check.sh
cargo run -p fe_reader_cli -- doctor
```

Start with `docs/v7-coding-agent-start-here.md` and `docs/wave0-first-30-prs.md`.

## v8 implementation-first additions

v8 deliberately avoids another product-scope expansion. It makes Wave 0 more executable:

- `fe_reader_core` now defines operation intents, patch plans, write modes, transaction journals, receipts, error taxonomy, resource limits, document fingerprints and deterministic SHA-256 helpers.
- `fe_reader_pdf_model` now provides minimal PDF sniffing, page geometry contracts, text spans, page labels and document summaries.
- `fe_reader_security` now provides a small policy evaluator for read, plan, apply, export, automation, plugin and external-tool actions.
- `fe_reader_render`, `fe_reader_search`, `fe_reader_text`, `fe_reader_metadata`, `fe_reader_jobs` and `fe_reader_config` now expose concrete Wave 0 contracts instead of identity-only stubs.
- `fe-reader inspect <path> --json` now emits a concrete non-mutating document summary when given a PDF-like file.

The next coding step is to run `cargo test --workspace --all-targets` in a Rust-enabled environment, then implement the first real PDF parser/renderer adapters behind these contracts.

## v8 implementation-first additions

v8 deliberately avoids another product-scope expansion. It makes Wave 0 more executable:

- `fe_reader_core` now defines operation intents, patch plans, write modes, transaction journals, receipts, error taxonomy, resource limits, document fingerprints and deterministic SHA-256 helpers.
- `fe_reader_pdf_model` now provides minimal PDF sniffing, page geometry contracts, text spans, page labels and document summaries.
- `fe_reader_security` now provides a small policy evaluator for read, plan, apply, export, automation, plugin and external-tool actions.
- `fe_reader_render`, `fe_reader_search`, `fe_reader_text`, `fe_reader_metadata`, `fe_reader_jobs` and `fe_reader_config` now expose concrete Wave 0 contracts instead of identity-only stubs.
- `fe-reader inspect <path> --json` now emits a concrete non-mutating document summary when given a PDF-like file.

The next coding step is to run `cargo test --workspace --all-targets` in a Rust-enabled environment, then implement the first real PDF parser/renderer adapters behind these contracts.

## v9 strict contracts and repository CI/CD

v9 adds actual GitHub Actions workflows, CODEOWNERS, Dependabot/Renovate, branch-ruleset templates, strict contract manifests and CI policy checks. The implementation strategy is now:

1. Stable PR checks are hard and boring.
2. Frontier/nightly checks are aggressive but isolated.
3. Release builds emit evidence: SBOM, provenance attestation and release-readiness reports.
4. All mutating surfaces must pass OperationIntent -> PatchPlan -> TransactionJournal -> Apply -> Verify -> Receipt.

Start with `docs/v9-coding-agent-start-here.md`.
