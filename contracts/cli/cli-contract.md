# CLI Contract

The CLI is both a user tool and the canonical implementation test harness.

## Commands

```bash
fe-reader inspect input.pdf --json
fe-reader render input.pdf --page 5 --scale 2.0 --out page-5.png
fe-reader extract-text input.pdf --page 5 --bbox --json
fe-reader metadata input.pdf --json
fe-reader metadata-diff before.pdf after.pdf --json
fe-reader metadata-scrub input.pdf --profile clean-share --plan-only
fe-reader organise input.pdf --delete 3,4 --out output.pdf
fe-reader plan-workflow input.pdf --workflow healthcare.deidentify.basic --params params.json
fe-reader plan-redaction input.pdf --recipe redaction.json
fe-reader apply-plan input.pdf plan.json --approval approval.json --out output.pdf
fe-reader verify-redaction output.pdf --receipt receipt.json
fe-reader convert input.pdf --to markdown --out output.md
fe-reader validate input.pdf --profile syntax
fe-reader platform self-test --json
```

## Output rules

- `--json` output must be stable and schema-backed.
- `inspect --json` must emit `intent`, a non-mutating `plan`, and `summary.parser` with parser adapter, page count when safely available, encryption state when safely available, trailer keys, and non-fatal parser error details.
- Destructive commands must support `--plan-only`.
- Applying a plan requires matching input document hash.
- Commands must return non-zero on verification failure.
