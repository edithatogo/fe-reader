# Implementation plan: Serde/YAML qualification

- [ ] Freeze the exact current `Cargo.lock`, `BudgetFile` type, all budget YAML fixtures and expected validation outcomes; report active ownership/PR conflicts before editing manifests.
- [ ] Inspect maintained alternatives' source, Rust MSRV, licence, security advisories and versions; measure transitive dependencies and compile cost. Record a go/no-go ADR.
- [ ] Build an isolated differential harness using valid/invalid budgets: normal records, missing/extra fields, duplicate keys, aliases/anchors, excessive nesting, YAML tags, number/null coercions and malformed UTF-8.
- [ ] If the replacement passes, change *only* `crates/xtask` dependency and call sites; prove unchanged budgets, errors (or documented improvement), performance guard decisions and all feature configurations. Do not touch `fe_reader_core`.
- [ ] Assess stable `serde_json::Value` uses separately: keep open extension/document metadata dynamic; prefer typed DTOs only for a versioned stable public wire contract and retain unknown-data preservation where specified.
- [ ] Run affected tests, workspace formatting/lints, supply-chain and perf gates on the exact PR head. Record command, environment, SHA and outcome; no acceptance until real test results exist.
- [ ] Have the existing Fe Reader integration owner review, integrate and close according to `AGENTS.md` including Conductor review, notes and archive rules. Do not claim migration completed in this planning PR.
