# Summary

Describe the change and the wave or track it advances.

# Contracts and Safety

- [ ] Core remains free of UI, platform, renderer, AI, MCP and plugin runtime dependencies.
- [ ] Mutating flows still use OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt.
- [ ] Automation surfaces remain read-only or plan-only unless all mutation guards are present.
- [ ] Public API changes include a compatibility note and versioning decision.

# Evidence

- [ ] Tests, schema validation or smoke checks were run.
- [ ] Fixture, corpus, visual, performance or differential evidence was added where claims depend on PDF behaviour.
- [ ] Release, privacy, diagnostics or accessibility docs were updated if public behaviour changed.
