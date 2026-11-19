# K2 runtime follow-up — 2026-11-20

The historical K2 completion above qualified its contract skeleton. Inspection
found `fuzz_smoke.sh` reporting pass when cargo-fuzz was merely installed, with no
campaign execution. Current `fe_reader_pdf_model` has real lopdf byte parsers;
PDFium remains a stub. This follow-up does not promote renderer readiness.

- [x] Add shared corpus/mutation invariants and a real `pdf_bytes` libFuzzer target.
- [x] Add synthetic seeds, exact tool/dependency pins, limits and reproduction.
- [x] Correct smoke reporting: pass requires execution; a failing run returns failure.
- [x] Add an isolated advisory PR/weekly workflow with read-only permissions/artifacts.
- [x] Run the two focused native corpus/mutation regression tests.
- [x] Review the source against the read-only parser and advisory-frontier contracts.
- [ ] Complete sanitizer execution and the full required validation gates before admission.
- [ ] Inspect final-revision hosted CI; promotion to a required gate needs the existing ADR/baseline review.

See `../../../fuzz/README.md`. This follow-up keeps other phases and ownership intact.

## Source review and executed evidence

Base: `dffebe059b8df8dab8ecc14bf1eb3612af947787`. Both new native regression
tests passed on Rust 1.99.1 during implementation. The later full workspace
`cargo +1.99.1 test --locked --workspace --all-features` attempt was killed
while compiling `syn`, before executing tests; no full-suite pass is claimed.
A local instrumented build was started but has no completed campaign receipt.

Schema validation, architecture compliance and the Conductor lifecycle check
passed. Actionlint 1.7.9 syntax/expression validation passed. The source review
confirmed that all four byte APIs document the same PDF-header error boundary,
that parser operations remain read-only and that PDFium is not represented as
qualified. The new campaign remains advisory; its workflow's overall conclusion
must not be used as an execution pass without checking the campaign step/log.

Full conductor-review/supply-chain validation, remote Conductor note delivery,
required hosted checks and default-sanitizer qualification remain admission
gates. This follow-up does not archive or newly complete Track K.
