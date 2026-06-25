# Tracks

| Track | Theme | Main wave(s) | Summary |
|---|---|---:|---|
| A Core Engine | Foundation | 0-3 | Core contracts, patch planning, PDF operations |
| B Rendering & Hardware | Reader | 1-6 | PDFium rendering, tiles, GPU compositor option |
| C Bindings & ABI | Native | 1-5 | UniFFI, C# wrapper, stable ABI |
| D UI & Native Shell | Reader | 1-4 | Tauri/Svelte shell, mobile wrappers, document UX |
| E Platform Integration | Native | 1-5 | Windows/macOS/Linux/Android/iOS OS integration |
| F Workflow Packs | Workflows | 2-4 | Domain workflow packs and template engine |
| G Metadata, Standards & Preflight | Pro docs | 2-5 | XMP, scrub, PDF 2.0, PDF/A/UA/X adapters |
| H Conversion & Source Pipelines | Publishing | 4-5 | Markdown/DOCX/HTML/Typst/Quarto/LaTeX/Pandoc |
| I External Integrations | Integrations | 5 | Zotero, Obsidian, browser extension, storage providers |
| J Distribution & Publishing | Release | 4-7 | installers, registries, stores, local/global install, signed updates |
| K Security & Quality | Foundation | 0-7 | fuzzing, supply chain, redaction verification, review skill |
| L Web & Browser | Integrations | 5 | Web local/PWA and browser extension |
| M Frontier Intelligence | Frontier | 6 | optional local NLP, embeddings, grounded Q&A |
| N Performance Engineering | Performance | 0-7 | budgets, benchmarks, profiling, PGO/LTO, perf gates |
| O Security Isolation & Policy | Safety | 0-7 | threat model, sandboxing, permissions, automation policy |
| P Compatibility Corpus & Visual QA | Quality | 0-7 | fixtures, visual regression, differential tests, corpus governance |
| Q Search, Text, Fonts & I18N | Reader | 1-7 | deterministic index, Unicode, CJK/RTL, font fallback, accessibility text |
| R Release Ops & Enterprise | Operations | 4-7 | signed updates, SBOM/provenance, policy templates, supportability |
| S Developer Ecosystem & SDK | Ecosystem | 5-7 | public SDK contracts, API versioning, plugin devkit, docs |

## Parallelisation rules

- A0, K0, O0 and P0 run first.
- B, C, D, E, N and Q may proceed in parallel after A0 core types compile.
- P visual-regression harness may begin as soon as B can render a fixture.
- R begins in Wave 4, but release-policy documents may be drafted earlier.
- S begins in Wave 5 after the CLI, MCP and plugin surfaces have stable contracts.
- M is optional and cannot block release hardening.

| T PDF Engineering Lab & Repair | Diagnostics | 1-7 | Object tree, content streams, safe-open, repair planning, incremental timeline |
| U Differential Oracles | Quality | 0-7 | qpdf/veraPDF/PDFium/Poppler/MuPDF/Ghostscript/Pandoc/LibreOffice comparison harness |
| V API Stability Governance | Ecosystem | 0-7 | SemVer, ABI, CLI/MCP/plugin/automation contract snapshots and diffs |
| W Reproducible Builds & Provenance | Release | 4-7 | SBOM, attestations, release evidence, reproducible build policy |
| X Advanced Prepress, Colour & Fonts | Standards | 2-7 | ICC, OutputIntents, spot colours, overprint, font diagnostics, PDF/X bridge |
| Y Maintainer RFCs & ADRs | Governance | 0-7 | ADRs, RFCs, maintainer domains, contribution gates |

Additional v5 parallelisation rules:

- Y0 and V0 can run during Wave 0 without waiting on code.
- T0 and U0 can run once A0/P0/O0 contracts exist.
- X0 can run during Wave 2 after metadata/page contracts exist.
- W starts in Wave 4, but W0 schema work can be drafted earlier.
- No public API surface can move from preview to stable without Track V review.


## v6 additional tracks

| Track | Theme | Purpose |
|---|---|---|
| AA | Document IR | Typed intermediate representation and transformation-pass compiler |
| AB | UX QA | UI design system, E2E shell tests and accessibility audit strategy |
| AC | Jobs & Power | Job scheduler, progress/cancellation, resource limits and power/thermal budgets |
| AD | Governance/Public Signals | Community governance, open quality dashboards and public benchmark reports |
| AE | PDF Safety Frontier | PDF Time Machine, revision diffing and Active Content Firewall |
| AF | Source Pipelines | Quarto, Typst, LaTeX, Markdown, Pandoc and reproducible document workspace mode |
| AG | Config & Policy | Unified settings, feature flags, enterprise policy and automation policy engine |
| AH | Agent Evaluation | Coding-agent evaluation tasks and spec-drift detection harness |

## v6 parallelisation rules

- AA0 starts in Wave 0 after A0 because IR contracts must shape patch planning.
- AC0 starts in Wave 0 because long-running work and cancellation must not be retrofitted.
- AB0 starts in Wave 1 with the first UI shell and becomes mandatory by Wave 2.
- AE0 starts in Wave 2 for detection-only work; mutating restore/export waits until Wave 5+.
- AF starts in Wave 4 with conversion providers and becomes workflow-focused in Wave 5.
- AG starts in Wave 0 and must gate automation, plugins and enterprise policy before Wave 5.
- AD/AH run continuously as project-operating-system tracks.


## v6 additional tracks

| Track | Theme | Main wave(s) | Summary |
|---|---|---:|---|
| Z UX, Accessibility & Human Factors | UX | 0-8 | Command palette, keyboard parity, WCAG-targeted web/PWA, screen-reader support and accessibility reports. |
| AA Source-Linked Authoring | Publishing | 2-6 | Typst, Quarto, LaTeX/Tectonic and Pandoc source-linked project workflows. |
| AB Cache, Workspace & Offline Collaboration | Workspace | 0-8 | Content-addressed cache, workspace catalogue, review packets and mergeable sidecars. |
| AC PDF Optimisation & Linearisation | Performance | 2-7 | Safe rewrites, compression, dedupe, linearisation and optimisation receipts. |
| AD Toolchain Optimisation & Experimental Lanes | Performance | 0-7 | Build-speed, PGO/BOLT/linker/allocator/SIMD experiments with evidence gates. |
| AE Documentation, Training & Community | Adoption | 0-8 | User/admin/developer docs, tutorials, examples, contribution and support playbooks. |

Additional v6 parallelisation rules:

- Z0, AB0, AD0 and AE0 can start in Wave 0 after core policy contracts are present.
- AA0 starts once conversion provider contracts exist; AA implementation must not execute arbitrary shell commands without explicit policy.
- AC0 starts after metadata/page-operation contracts exist and before optimisation claims are made.
- AB collaboration work can proceed without cloud sync; cloud/network collaboration remains explicitly out of scope unless separately approved.
- AE documentation work can run continuously and should update examples after each accepted contract change.

## v7 implementation-readiness tracks

- Track AF: Implementation Readiness & PR Sequencing — Wave 0 executable scaffold, first-30-PR plan, bootstrap checks and stop-expanding rules.
- Track AG: Errors, Migrations & Compatibility — stable error taxonomy, sidecar versioning and migration contracts.
- Track AH: IP, Brand & Originality Governance — neutral product positioning, dependency license hygiene and originality gates.


## v8 implementation-first tracks

| Track | Theme | Main wave(s) | Status | Summary |
|---|---|---:|---|---|
| AI | Executable Wave 0 Contracts | 0 | complete | Workspace compiles, Wave 0 contract tests pass, CLI smoke and conservative policy checks succeed. |
| AJ | CLI First Integration | 0 | complete | CLI inspect now reports parser-backed page count and diagnostics while preserving read-only planning. |
| AK | Contract Acceptance Tests | 0 | complete | Wave 0 acceptance checks now validate CLI JSON, parser schema, policy matrix, and security-policy defaults as hard gates. |

## v9 repository and CI/CD hardening tracks

| Track | Theme | Main wave(s) | Status | Summary |
|---|---|---:|---|---|
| AL | Strict Contracts | 0-7 | complete | Contract manifest, CI policy, release evidence, and CLI/MCP/plugin/platform mutation contract enforcement are hard-gated. |
| AM | Repo CI/CD | 0-7 | complete | Repository CI/CD policy, CODEOWNERS, dependency automation, branch ruleset, workflow hard gates, and release evidence smoke are enforced. |
| AN | Frontier CI | 0-7 | complete | Advisory beta/nightly, Miri, sanitizer, fuzz, GPU, performance, and PGO/BOLT experiment lanes are isolated and policy-checked. |
| AO | Release Provenance | 4-7 | complete | SBOM status, provenance, signing readiness, release evidence bundle, and public-channel provenance policy are scaffolded and policy-checked. |

v9 parallelisation rules:

- AL0 and AM0 are Wave 0 hardening tasks and should run before expanding product features.
- AN can run immediately as a non-blocking scheduled/manual lane.
- AO starts during Wave 4, but provenance and SBOM scripts can be scaffolded in Wave 0.

## Native macOS UX refinement tracks

| Track | Theme | Main wave(s) | Status | Summary |
|---|---|---:|---|---|
| AP | Native UI Wireframe Discovery | 1 | complete | Figma wireframe, shell state model, and interaction handoff for the native macOS UX are documented. |
| AQ | Native UI Shell Implementation | 1-2 | complete | Placeholder shell replaced with a native SwiftUI desktop layout, document intake, recents, metadata, status bar, and command surfaces. |
| AR | Native UI Accessibility & Polish | 2-3 | complete | Keyboard labels, accessibility names, visual polish, screenshot evidence, and native preview regression guard are in place. |

Native UX refinement rules:

- AP and its Figma file should be used as the design source of truth for AQ, AR and roadmap decisions until superseded by an approved design artifact.
- AQ must trace each implemented shell region/state back to the AP Figma roadmap and record any deviation before closing a phase.
- AQ should use a SwiftUI-first shell with AppKit interop only for platform affordances that require it.
- AQ should not continue any placeholder drawing path once the native shell is landed.
- AR should begin once AQ has a stable shell and command surface, then verify Figma-to-runtime conformance, keyboard access, accessibility and responsive behavior.

---

- [x] **Track: Desktop Packaging and Signing**
*Link: [./tracks/track-AS-desktop-packaging-signing/](./tracks/track-AS-desktop-packaging-signing/)*

---

- [x] **Track: Stable Release Evidence Gates**
*Link: [./tracks/track-AT-stable-release-evidence-gates/](./tracks/track-AT-stable-release-evidence-gates/)*

---

- [x] **Track: Desktop Distribution Publication**
*Link: [./tracks/track-AU-desktop-distribution-publication/](./tracks/track-AU-desktop-distribution-publication/)*

---

- [x] **Track: Enterprise Operations Readiness**
*Link: [./tracks/track-AV-enterprise-operations-readiness/](./tracks/track-AV-enterprise-operations-readiness/)*

---

- [x] **Track: Launch QA, Documentation and Homepage**
*Link: [./tracks/track-AW-launch-qa-docs-homepage/](./tracks/track-AW-launch-qa-docs-homepage/)*

---

- [x] **Track: Advanced Roadmap Continuation**
*Link: [./tracks/track-AX-advanced-roadmap-continuation/](./tracks/track-AX-advanced-roadmap-continuation/)*

---

- [x] **Track: Post-launch PDF Baseline Parity**
*Link: [./archive/track-AY-post-launch-pdf-baseline-parity/](./archive/track-AY-post-launch-pdf-baseline-parity/)*

---

- [x] **Track: Mobile Public Launch**
*Link: [./archive/track-AZ-mobile-public-launch/](./archive/track-AZ-mobile-public-launch/)*

---

- [x] **Track: Frontier Intelligence Governance**
*Link: [./archive/track-BA-frontier-intelligence-governance/](./archive/track-BA-frontier-intelligence-governance/)*

---

- [x] **Track: Opt-in Collaboration and Sync**
*Link: [./archive/track-BB-opt-in-collaboration-sync/](./archive/track-BB-opt-in-collaboration-sync/)*

---

- [x] **Track: Rendering Performance Promotion**
*Link: [./archive/track-BC-rendering-performance-promotion/](./archive/track-BC-rendering-performance-promotion/)*

---

- [x] **Track: Ecosystem Integrations and Marketplace**
*Link: [./archive/track-BD-ecosystem-integrations-marketplace/](./archive/track-BD-ecosystem-integrations-marketplace/)*

---



- [x] **Track: Windows and Linux Beta Installers**
*Link: [./archive/track-BG-windows-linux-beta-installers/](./archive/track-BG-windows-linux-beta-installers/)*

---

---

- [x] **Track: Professional Workflow Parity**
*Link: [./tracks/track-BK-professional-workflows-parity/](./tracks/track-BK-professional-workflows-parity/)*

---

- [x] **Track: Advanced PDF Family Parity**
*Link: [./archive/track-BL-advanced-pdf-family-parity/](./archive/track-BL-advanced-pdf-family-parity/)*

---

- [x] **Track: Marketing Claim Evidence Governance**
*Link: [./tracks/track-BM-marketing-claim-evidence-governance/](./tracks/track-BM-marketing-claim-evidence-governance/)*

---

- [x] **Track: Stable Launch Cutover and Registries**
*Link: [./archive/track-BN-stable-launch-cutover-registries/](./archive/track-BN-stable-launch-cutover-registries/)*

---

- [x] **Track: v2 Roadmap Implementation Foundation**
*Link: [./tracks/track-BO-v2-roadmap-implementation-foundation/](./tracks/track-BO-v2-roadmap-implementation-foundation/)*

---

- [~] **Track: macOS Native Functional Reader Parity**
*Link: [./tracks/track-BP-macos-native-functional-reader-parity/](./tracks/track-BP-macos-native-functional-reader-parity/)*
