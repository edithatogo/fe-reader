# Final v7 Improvements

v7 is intentionally not another scope expansion. It converts Fe Reader from a comprehensive architecture/specification package into an implementation-first handoff package.

## v7 additions

1. Real root `Cargo.toml` workspace.
2. Real crate `Cargo.toml` files copied from the previous samples.
3. Minimal `src/lib.rs` or `src/main.rs` stubs for every crate so the coding agent has concrete files to replace rather than empty placeholders.
4. Wave 0 bootstrap script and first-30-PR plan.
5. Error taxonomy contracts for consistent UX, CLI, MCP, local API and platform automation responses.
6. Operation transaction/journal contract to make mutating operations crash-safe from the first implementation pass.
7. Sidecar format versioning and migration contracts for `.feworkspace`, `.fereview`, `.fereceipt`, `.fetemplate`, `.feindex` and future sidecar formats.
8. IP/brand/trade-dress safety guardrails to keep the project an original local-first workflow platform rather than a vendor clone.
9. Exit criteria that tell agents when not to add more specification and to begin coding.

## v7 rule

Do not add another feature wave until Wave 0 produces:

- a compiling workspace,
- a working CLI `doctor` command,
- schema validation,
- a transaction journal prototype,
- a redaction-safe write-mode policy stub,
- first compatibility fixtures,
- phase gates passing in CI.

## Additional recommendations still worth considering after Wave 0

These are explicitly deferred until implementation feedback exists:

- renderer strategy shootout using real fixtures and measured first-page latency;
- PDFium versus alternate renderer adapter evaluation;
- verified qpdf/veraPDF oracle integration on generated PDFs;
- actual OS-specific installer signing dry-runs;
- a tiny web/PWA viewer spike using the same command contracts;
- local intelligence only after deterministic extraction/search/redaction work is mature.
