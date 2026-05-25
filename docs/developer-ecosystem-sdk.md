# Developer Ecosystem and SDK Plan

## Surfaces

- Rust crate APIs for internal workspace.
- CLI for automation and tests.
- MCP server for AI-agent read/plan workflows.
- UniFFI bindings for Swift/Kotlin/Python and C# fallback strategy.
- Web postMessage contract for web/PWA/browser extension.
- Plugin SDK for proposal-only workflow extensions.
- Optional NuGet package for native Windows/.NET integration.

## API versioning

All public surfaces have semantic versions:

```text
core contract version
patch plan schema version
workflow pack schema version
plugin ABI version
MCP tool manifest version
CLI contract version
platform automation version
C ABI fallback version
```

## Stability tiers

```text
internal
experimental
preview
stable
deprecated
removed
```

## SDK deliverables

- examples directory
- contract tests
- schema snapshots
- sample workflow pack
- sample read-only plugin
- sample MCP client script
- sample .NET/Swift/Kotlin calls

## .NET preview wrapper

`FeReader.Native` is a preview NuGet wrapper over the C ABI fallback. In Wave 0 it exposes managed contract metadata and P/Invoke declarations for identity, capability and no-write plan probes only. It does not ship native runtime assets or expose apply operations.

## Rule

SDK examples must never bypass `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
