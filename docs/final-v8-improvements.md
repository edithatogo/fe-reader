# Final v8 improvements

v8 shifts from planning into executable contracts. The prior package was broad and well planned, but many crates were identity stubs. v8 makes the first implementation surface concrete enough for a coding agent to compile, test and extend.

## Added implementation depth

- Core operation model: `OperationIntent`, `PatchPlan`, `PatchOperation`, `WriteMode`, `TransactionJournal`, `OperationReceipt`, `ResourceLimits`, `FeError`.
- PDF model: `PdfDocumentSummary`, `PdfHeader`, `PdfRect`, `PageIndex`, `TextSpan`, and non-mutating PDF byte/path sniffing.
- Security: default-deny policy for plugins, external tools and network access; automation mutation requires review by default.
- Rendering: tile-based `RenderBackend` contract plus `NullRenderBackend` for tests.
- Search/text: deterministic substring search and text normalisation/direction hints.
- Metadata: metadata operation planning into patch plans, without byte mutation.
- Jobs/config: cancellation, progress and feature flag scaffolding.
- CLI: `doctor`, `inspect`, `policy`, and `validate-schemas` commands.

## Recommendation

Do not add v9 scope until these v8 contracts compile and the first real parser/render adapters are implemented behind the contracts. The next useful expansion should be driven by compiler/test failures, not wishlist additions.
