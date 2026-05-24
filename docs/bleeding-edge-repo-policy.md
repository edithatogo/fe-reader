# Bleeding-Edge Repository Policy

## Principle

Use bleeding-edge tooling aggressively at the edges, but keep `fe_reader_core` deterministic, fuzzable, and boring.

## Stable lane

- Rust stable pinned in `rust-toolchain.toml`.
- Workspace compile, fmt, clippy, nextest.
- Core contracts and schema validation.
- Dependency firewall.

## Beta/nightly lane

- Rust beta and nightly compile smoke.
- Miri on small pure crates.
- Sanitizers for parser/transaction code.
- `-Z minimal-versions` or direct-minimal-versions experiments where Cargo supports them.
- `cargo udeps` / unused dependency checks when reliable.

## Experimental dependency lane

Experimental dependencies require:

- ADR ID;
- owner;
- feature gate;
- crate boundary outside core;
- rollback plan;
- evidence target;
- expiry/review date.

## Promotion rule

An experiment can become default only if it improves at least one accepted budget and does not regress security, portability, accessibility, or compatibility.
