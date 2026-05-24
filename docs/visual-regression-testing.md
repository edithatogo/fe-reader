# Visual Regression Testing

## Goal

Visual rendering must be measured, not assumed. Tests should catch rendering, annotation overlay and redaction visual failures.

## Baseline artefacts

```text
fixtures/expected/rendered/<fixture-id>/page-0001@1x.png
fixtures/expected/rendered/<fixture-id>/page-0001@2x.png
fixtures/expected/rendered/<fixture-id>/metadata.json
```

## Comparison method

- Normalise output to a defined colour space.
- Compare dimensions, crop boxes and page rotation first.
- Compare pixels with configurable tolerance.
- Report bounding boxes of differences.
- Save diff images under `target/visual-regression/`.

## Special cases

- Font fallback can vary by platform; use packaged fonts in deterministic tests.
- Antialiasing differs between renderers; use tolerance bands and structural checks.
- Annotation overlays should be tested separately from base rendering.
- Redaction tests require both visual black-box verification and content absence verification.

## CI policy

- Smoke visual tests run in phase gates once rendering exists.
- Full visual tests run nightly and before release.
- Accepted baseline changes require review notes explaining why the visual change is correct.
