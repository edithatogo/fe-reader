# Rust dependency rationalisation — fe-reader (2026-11-13)

## Repository-specific decisions

- Verified: broad workspace already centralises Anyhow, Thiserror, Serde, Tracing, Tokio, Rayon, Criterion and backend-specific types; do not force additional dependencies into the model/IR core.
- Evaluate existing CPU/GPU/Rayon scheduling and Bytes buffer reuse on memory and PDF evidence-integrity benchmarks; all rendering/prepress/transform kernels require pixel/document parity.
- Extend Proptest for PDF object-span, form field, annotation, incremental update and redaction no-residual-content invariants; Insta only for compact redacted contract snapshots.
- Retain each PDF engine and binding with explicit backend ownership; PyO3 only if a separately tested Python adapter is approved; Syn/Quote only for real procedural macros.
- Treat redaction, PII, untrusted documents, font embed policy, parser resource limits and atomic writes as hard safety gates; tracing must not record document content.

## Evidence and implementation gates

- Status: **planned / scope registered**; no runtime substitution or CI/benchmark success is asserted by this file.
- Reconcile existing Conductor tracks before new work; preserve domain-specific APIs, schema versions, privacy and public CLI behaviour.
- Write failing contract tests before code changes, then run native targeted/full tests, fmt/clippy and relevant integration/supply-chain checks at exact SHA.
- Use Cargo feature/duplicate inventories plus security, licence and MSRV assessments, and fixed-fixture performance baselines before adoption.
- Record adopt / retain / reject / defer decisions and rollback; only close after verified evidence.
