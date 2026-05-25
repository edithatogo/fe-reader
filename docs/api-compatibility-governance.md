# API, ABI and Automation Compatibility Governance

## Scope

Fe Reader will expose many public surfaces. Each needs explicit compatibility rules:

```text
Rust crates
UniFFI Swift/Kotlin/Python/Ruby bindings
C# wrapper / C ABI fallback
CLI commands and JSON outputs
MCP tools/resources/prompts
WASM plugin ABI
COM automation
AppleScript / App Intents
D-Bus automation
Android intents and DocumentsProvider contracts
iOS App Intents and document contracts
Web postMessage and browser-extension contracts
```

## Compatibility levels

| Level | Meaning |
|---|---|
| `internal` | Can change at any time inside a wave. |
| `experimental` | Available behind feature flag; breaking changes allowed with changelog. |
| `preview` | Intended shape; breaking changes require explicit migration notes. |
| `stable` | SemVer/versioned compatibility commitment. |
| `lts` | Enterprise long-term support commitment. |

## Rules

- No public surface becomes `stable` until it has tests, documentation, schema examples and a versioning policy.
- CLI JSON output must be schema-backed.
- MCP tools must be versioned by capability, not just by server version.
- Plugin ABI changes require a new `fe_plugin_api` version.
- UniFFI DTOs should avoid exposing internal enums that may expand rapidly.
- C# interop must remain isolated from UniFFI first-party binding assumptions.
- The Wave 0 UniFFI facade is `preview`, read-only or plan-only, and must not expose an apply path until mutation approval, transaction journaling, verification and audit receipt contracts are available at the binding boundary.
- The Wave 0 C ABI fallback is `preview`, `extern "C"`, P/Invoke-oriented, and limited to static identity/capability and no-write plan probes until a reviewed mutation boundary exists.

## Tools

Use:

```bash
cargo semver-checks
cargo public-api
cargo doc --workspace --no-deps
fe-reader contract check schemas/
fe-reader contract diff previous-contracts/ current-contracts/
```

`cargo-semver-checks` should run in CI once public Rust crates are published or a baseline rustdoc JSON exists.

## Contract snapshot layout

```text
contracts/snapshots/
  rust-public-api/
  uniffi/
  c-abi/
  cli/
  mcp/
  plugin-abi/
  automation/
```

See `contracts/rust/api_stability.rs` and `schemas/api-compat-report.schema.json`.
