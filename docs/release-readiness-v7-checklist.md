# Release Readiness v7 Checklist

This is not for public release. It is for deciding whether Wave 0 is ready to hand to feature-building agents.

- [ ] `cargo metadata` succeeds.
- [ ] `cargo test --workspace --all-targets` succeeds or failures are documented as dependency/toolchain setup issues.
- [ ] `fe-reader doctor` works.
- [ ] `fe-reader inspect --json` emits a non-mutating plan stub.
- [ ] Schema validation passes.
- [ ] Architecture compliance check confirms core dependency firewall.
- [ ] Error taxonomy and operation transaction contracts are present.
- [ ] First compatibility fixture manifest exists.
- [ ] First 30 PRs are represented as issues/tasks.
- [ ] No local AI/ML/RAG dependency is added to Wave 0.
- [ ] Automation surfaces remain read-only by default.
