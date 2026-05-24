# Benchmarks

Benchmarks are part of the Fe Reader product specification. Do not merge core PDF, render, redaction, metadata, conversion, or platform changes without adding or updating scenarios.

## Command tiers

```bash
# Fast smoke for local development
cargo xtask perf smoke

# Standard local benchmark suite
cargo xtask perf bench --suite standard

# Linux deterministic CI suite
cargo xtask perf callgrind --suite deterministic

# CLI-level scenarios
cargo xtask perf hyperfine --suite cli

# PGO training and optimized build
cargo xtask perf pgo-train --suite default
cargo xtask perf pgo-build
```

## Scenario naming

Use `theme.operation.fixture.scale`, for example:

```text
render.tile.legal_bundle.2x
core.extract_text.scientific_paper.single_page
workflow.affidavit.apply_100_pages
redaction.secure_rewrite.healthcare_100_pages
metadata.scrub.large_bundle
conversion.markdown.research_paper
```
