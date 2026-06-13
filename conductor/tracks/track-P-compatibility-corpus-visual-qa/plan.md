# Track P Plan

## P0 Wave 0

- [x] Create corpus directory skeleton.
- [x] Add `schemas/test-fixture.schema.json`.
- [x] Add corpus manifest validator.
- Evidence: `fixtures/corpus/manifest.json`, `schemas/test-fixture.schema.json`, `scripts/corpus_manifest_validate.py`.

## P1 Wave 1

- [x] Add first basic PDF fixture and render baseline placeholders.
- [x] Add text-extraction golden file format.
- Evidence: `fixtures/corpus/basic/text-search-fixture.pdf`, `fixtures/expected/rendered/text-search-fixture/metadata.json`, `fixtures/expected/search/text-search-fixture.search-index.json`, `scripts/visual_regression_compare.py`.

## P2 Wave 2

- [x] Implement PNG comparison and diff output.
- [x] Add CJK/RTL/font fixtures.
- Evidence: `scripts/visual_regression_compare.py`, `target/visual-regression/text-search-fixture/comparison.json`, `target/visual-regression/text-search-fixture/comparison.diff.png`, `fixtures/corpus/rtl-cjk-complex-text/`.

## P3 Wave 3

- [x] Add redaction, signed-document and incremental-update fixtures.
- Evidence: `fixtures/corpus/redaction/secure-redaction-smoke.recipe.json`, `fixtures/corpus/signed/README.md`, `fixtures/corpus/incremental-updates/incremental-update-smoke.txt`, `fixtures/corpus/manifest.json`.

## P4 Wave 7

- [x] Produce release compatibility report covering accepted fixture classes.
- Evidence: `target/compatibility-corpus-report.json`, `target/compatibility-corpus-report.md`, `scripts/compatibility_corpus_report.py`, `scripts/wave7_release_hardening_smoke.py`.
