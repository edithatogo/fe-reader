# Phase Gates

## Gate 0 — Contract compile

- Core contracts compile.
- Schemas validate.
- No forbidden core dependencies.

## Gate 1 — Reader smoke

- Render fixture tile.
- Extract page text spans.
- CLI inspect command returns JSON.

## Gate 2 — Operation plan safety

- Page ops and metadata ops return patch plans.
- Destructive operations require approval.

## Gate 3 — Redaction safety

- Secure redaction uses full sanitising rewrite.
- Verification checks run.
- Audit receipt generated.

## Gate 4 — Packaging safety

- Package matrix exists.
- Code-signing/notarisation checklist complete.
- Per-user/global install behaviour documented.

## Gate 5 — Automation safety

- Automation surfaces read-only by default.
- High-risk calls return patch plans.
- Approval tokens required before apply.

## Gate 6 — Intelligence safety

- Local intelligence optional.
- Evidence citations required.
- No model-driven automatic mutation.
