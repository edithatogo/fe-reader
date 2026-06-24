# Corpus/Oracle Evidence Factory Contract

Fe Reader meets the corpus/oracle evidence factory contract only when representative fixture classes, oracle-report generation and accepted limitation boundaries are all wired into the repository evidence path.

## Contract

- `fixtures/corpus/manifest.json` defines the accepted compatibility corpus.
- `scripts/compatibility_corpus_report.py` generates the compatibility corpus report from the manifest.
- Differential oracle reports are generated through the documented oracle smoke path.
- Corpus coverage and oracle coverage remain linked to the PDF parity registry and baseline matrix.
- Unfinished families remain documented limitations rather than silent claims.

## Required evidence

- `fixtures/corpus/manifest.json`
- `scripts/compatibility_corpus_report.py`
- `target/compatibility-corpus-report.json`
- `target/compatibility-corpus-report.md`
- `scripts/differential_oracle_smoke.sh`
- `target/oracle-reports/wave1-render-smoke.json`
- `docs/pdf-parity-registry.md`
- `docs/pdf-baseline-parity-matrix.md`

## Factory scope

The evidence factory must cover both sides of the claim boundary:

- corpus generation and classification for fixture-backed claims;
- differential/oracle comparison reports for claim validation.

This contract does not claim that every PDF family is fully supported. It requires the corpus manifest, report generator, oracle report and claim registry to stay consistent and public-doc linked.
