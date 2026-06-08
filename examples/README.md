# Fe Reader Examples

These examples are declarative contract samples. They do not mutate documents directly and must not bypass OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt.

## Coverage

- `cli/inspect-minimal.sh` shows a read-only CLI inspection path.
- `workflows/legal-deidentify.plan.json` shows a workflow pack request that remains plan-only.
- `source-linked-projects/typst-provider-capability.json` shows provider capability discovery before external source tooling.
- `review-packets/local-review-packet.json` shows a local-first review packet.
- `plugins/read-only-plugin-manifest.json` shows a proposal-only plugin stub.
