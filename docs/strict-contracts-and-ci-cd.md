# Strict Contracts and CI/CD Architecture

## Contract families

Fe Reader uses contracts at multiple levels.

| Contract | Format | Enforcement |
|---|---|---|
| Core Rust contracts | Rust types/traits | compile, unit tests, architecture firewall |
| Operation contract | Rust + JSON schema | unit tests, schema validation, CLI/MCP tests |
| Public API contract | rustdoc JSON / cargo public-api snapshots | API compatibility workflow |
| CLI contract | golden JSON and stdout/stderr snapshots | PR CI |
| MCP contract | JSON schema + integration tests | automation safety workflow |
| Plugin ABI contract | WASM ABI manifest | plugin host tests |
| Platform automation contracts | IDL/SDEF/D-Bus XML/App Intents/Android intents | platform contract tests |
| Sidecar format contract | JSON schema + migration tests | migration workflow |
| Security policy contract | schema + policy tests | P0 hard gate |
| Performance contract | benchmark budgets | P1 after baseline |
| Release contract | SBOM + provenance + signatures | release hard gate |

## The invariant

All mutating paths must satisfy:

```text
OperationIntent -> PatchPlan -> TransactionJournal -> Apply -> Verify -> Receipt
```

This applies to UI, CLI, MCP, COM, AppleScript, D-Bus, Android intents, iOS App Intents, browser extensions, plugins, workflow packs and local HTTP/JSON-RPC.

## Hard PR checks

- `strict-contracts`: architecture firewall, schemas, workflow policy, v8 static checks.
- `rust-stable`: fmt, clippy, workspace tests, docs check.
- `security-supply-chain`: cargo-deny, cargo-audit, cargo-vet advisory mode during bootstrap, zizmor/actionlint, CodeQL, Scorecard.
- `api-compatibility`: public API snapshots and semver checks once stable surfaces exist.
- `cross-platform-smoke`: Linux, macOS, Windows compile/test smoke.

## Frontier checks

- Rust beta/nightly.
- Miri for core crates.
- Sanitizers on Linux where supported.
- Fuzz smoke and longer fuzz campaigns.
- PGO/BOLT and allocator experiments.
- GPU rendering/compositor experiments.
- WASM/WASI/web adapter experiments.

Frontier lanes are not release blockers until promoted by ADR.
`scripts/frontier_ci_check.py` keeps those lanes scheduled/manual, read-only,
continue-on-error, and advisory-only while still requiring smoke scripts for
Miri, sanitizers, fuzz, GPU experiments, differential oracles, performance
smoke, and PGO/BOLT/build-speed tooling discovery.
