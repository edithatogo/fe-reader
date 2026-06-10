# Fe Reader Versioning

Current release line: `0.1.0-preview.1`

## Version Layers

Fe Reader uses separate but linked version layers:

- Product and release line: SemVer with prerelease identifiers, currently
  `0.1.0-preview.1`.
- Rust crate API versions: SemVer per crate, currently `0.1.0`.
- C ABI version: `0.1.0`, tracked through
  `contracts/snapshots/c-abi/fe_reader_c_abi.facade.json`.
- NuGet wrapper version: `0.1.0-preview.1`, matching the product preview line.
- Package manifests: must reference the same product release line or document a
  platform-specific version mapping.
- Contract package generation: v9, tracked through implementation docs and
  contract tests.

## Rules

1. Public Rust API changes require an API compatibility note and SemVer decision.
2. CLI, MCP, C ABI, UniFFI, COM, AppleScript, D-Bus, Android intent, iOS App
   Intent and plugin contract changes require a compatibility note.
3. Release tags use `v<product-version>`, for example `v0.1.0-preview.1`.
4. Stable releases require SBOM, provenance, signing-readiness and release
   evidence artifacts.
5. Registry publication is allowed only after the relevant package manifest,
   signing evidence and smoke test pass for that platform.

## Current Scope

`0.1.0-preview.1` is a repository and contract preview. It validates the
headless Rust workspace, CLI smoke path, platform contracts, release evidence
and packaging metadata. It is not a production application release.
