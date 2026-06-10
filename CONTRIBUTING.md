# Contributing to Fe Reader

Start with `docs/v9-coding-agent-start-here.md`, `AGENTS.md`,
`conductor/waves.yaml` and `contracts/README.md`.

## Contributor Statement

Fe Reader welcomes contributions that make local PDF workflows safer, more
private, more verifiable and more accessible. Contributions should be grounded
in reproducible evidence rather than broad claims.

By contributing, you agree that:

- You have the right to submit the contribution.
- You will not include private PDFs, credentials, support bundles, private
  paths, document text or proprietary fixtures without permission.
- You will follow `CODE_OF_CONDUCT.md` and report sensitive vulnerabilities
  through `SECURITY.md`.
- You will preserve the core/adapters architecture unless an ADR accepts a
  change.
- You will not bypass the mutation pipeline:
  `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.

## Contribution Requirements

Contributions must preserve core boundaries, include tests or evidence, and
update traceability/ADR docs when they change public contracts or architecture.

Every core feature needs at least one of:

- CLI golden test
- schema validation
- fuzz target
- visual regression fixture
- differential test against a reference renderer/tool
- platform contract smoke test
- performance scenario budget

Public API changes to Rust crates, UniFFI, CLI, MCP, plugins, COM,
AppleScript, D-Bus, Android intents, iOS App Intents or package manifests must
include a compatibility note and versioning decision.

## Developer Checks

Run the smallest relevant checks for your change, then the full gate before
release-facing work:

```bash
cargo fmt --all -- --check
cargo test --workspace --all-targets
python3 scripts/validate_schemas.py
python3 scripts/ci_policy_check.py
```

For phase-level work:

```bash
bash scripts/conductor_phase_gate.sh --phase <phase-id> --auto-fix
```

## Certificate of Origin

Fe Reader currently uses a lightweight contributor statement rather than a CLA.
Maintainers may add a Developer Certificate of Origin requirement before
accepting outside production-bound contributions.
