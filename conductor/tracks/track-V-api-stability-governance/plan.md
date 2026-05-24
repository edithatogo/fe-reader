# Track V: API Stability & Contract Governance

## Mission

Keep Fe Reader's public surfaces stable, versioned and migration-friendly.

## Public surfaces

Rust crates, UniFFI, C#/C ABI, CLI, MCP, plugins, COM, AppleScript, D-Bus, Android intents, iOS App Intents, Web postMessage and browser-extension contracts.

## Phases

### V0 Stability levels

Define `internal`, `experimental`, `preview`, `stable` and `lts` in docs and contracts.

### V1 Contract snapshots

Create `contracts/snapshots/` layout and initial snapshot command.

### V2 API compatibility checks

Add `cargo-semver-checks`, CLI schema diff and MCP/plugin ABI diff to CI.

### V3 SDK release notes

Require migration notes for any breaking change to a preview/stable surface.

## Review gates

No stable surface may merge without docs, tests, examples and compatibility report.
