# Reader/Render/Search/Accessibility Parity Contract

This reader-render-search-accessibility parity contract binds the implemented baseline evidence to a named claim boundary.
reader-render-search-accessibility-parity

Fe Reader meets the reader/render/search/accessibility parity contract when the reader baseline, rendering baseline, search diagnostics and accessibility reports are all present, linked and passing.

## Contract

- The reader baseline is documented in `docs/stable-reader-readiness.md`.
- Rendering parity remains represented by the baseline visual regression path and the parity registry.
- Search parity remains represented by deterministic text diagnostics and search compatibility evidence.
- Accessibility parity remains represented by keyboard and screen-reader evidence.
- The parity boundary stays linked to the exhaustive PDF parity registry and the nested baseline matrix.

## Required evidence

- `docs/stable-reader-readiness.md`
- `docs/pdf-baseline-parity-matrix.md`
- `docs/pdf-parity-registry.md`
- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/launch-qa.json`
- `target/release-evidence/release-readiness.json`
- `target/accessibility-reports/smoke.json`
- `target/search-compatibility-report.json`
- `target/visual-regression/text-search-fixture/comparison.json`

## Coverage

- Reader: open local PDFs, inspect metadata, navigate, restore session state.
- Render: deterministic render smoke and visual regression for the baseline experience.
- Search: deterministic text/search evidence and text diagnostics.
- Accessibility: keyboard and screen-reader evidence for the reader baseline.

This contract does not claim full parity for every PDF family. It binds the implemented baseline evidence to the named reader/render/search/accessibility claim boundary.
