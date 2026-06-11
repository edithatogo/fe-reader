# Documentation, Training and Community Plan

Implementation quality depends on documentation quality. Fe Reader needs user docs, developer docs, contributor docs and workflow documentation from the start.

## Documentation surfaces

| Surface | Content |
|---|---|
| User guide | Reading, annotation, metadata, safe-share, redaction, workflows. |
| Admin guide | Enterprise policy, installers, updates, registry distribution, diagnostics. |
| Developer guide | Core architecture, contracts, plugins, CLI, MCP, SDKs, testing. |
| Workflow cookbook | Healthcare de-identification, FOI, publishing preflight, research notes, engineering markup. |
| API docs | Rust docs, UniFFI bindings, CLI reference, MCP tools, local API, plugin ABI. |
| Security docs | Threat model, responsible disclosure, sandboxing, supply chain. |

## Examples repository structure

```text
examples/
  cli/
  mcp/
  plugins/
  workflows/
  source-linked-projects/
  metadata-scrub/
  redaction-verification/
  platform-automation/
```

## Contributor onboarding

- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `GOVERNANCE.md`
- `docs/architecture-decision-records.md`
- issue templates for bug/feature/security/performance/corpus fixture
- pull-request template requiring contracts/tests/fixtures/performance notes where relevant

## Training packs

Create small tutorials:

1. Build the CLI.
2. Open and inspect a PDF.
3. Render a page tile.
4. Create a metadata scrub plan.
5. Apply a safe annotation patch.
6. Create a review packet.
7. Build a Typst-linked project.
8. Add a workflow pack.
9. Add a plugin proposal.
10. Run a corpus/visual regression test.

## Community metrics

Prefer maintainability signals over vanity metrics:

```text
time to first successful build
number of passing fixture classes
API stability warnings
review latency
security response time
performance budget regressions
corpus coverage
accessibility audit pass rate
```

## Evidence and release notes

- The current adoption smoke evidence is `target/release-evidence/wave8-adoption-ecosystem.json`.
- User, admin and developer guides should be updated in lockstep when a contract or workflow changes.
- Training examples should stay small, runnable and directly tied to the supported contracts instead of drifting into tutorial fiction.
