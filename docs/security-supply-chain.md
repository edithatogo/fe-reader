# Security and Supply Chain Plan

## Security principles

- Treat all PDFs as hostile input.
- Keep parsers and renderers isolated behind small interfaces.
- Never let automation bypass review for destructive operations.
- Disable PDF JavaScript/RichMedia execution by default.
- Prefer deterministic verification for redaction.
- Keep plugin execution sandboxed.
- Store secrets with platform facilities.

## Supply-chain checks

Use:

- `cargo deny check` for advisories/licenses/bans;
- `cargo audit` for vulnerability advisories;
- `cargo vet` for dependency audits;
- `cargo metadata --locked` for lockfile consistency;
- SBOM generation in release workflows.

## Fuzzing

Initial targets:

```text
fuzz_parse_document
fuzz_extract_text_spans
fuzz_page_tree_walk
fuzz_parse_xref
fuzz_plan_redaction
fuzz_apply_workflow_template
fuzz_metadata_scrub
fuzz_conversion_job_schema
```

## High-risk operation gates

Operations that require human/policy approval:

- secure redaction;
- metadata scrub;
- delete/extract pages from source;
- export with irreversible conversion;
- apply workflow to multiple documents;
- execute plugin proposal;
- save over original;
- invalidate existing signatures;
- use external conversion provider;
- expose document text to MCP client.

## v4 release security additions

- Generate CycloneDX SBOM for each release artifact.
- SBOM audit should verify CycloneDX structure, metadata component identity and component coverage before release evidence is accepted.
- Generate provenance/attestation artifacts for CI-built binaries.
- Validate signed update manifests before publication.
- Keep renderer/converter/helper-process crash failures non-mutating.
- Add explicit policy checks for PDF JavaScript, RichMedia, launch actions, embedded files, plugins and all automation surfaces.

## Dependency tiers

```text
Tier 0: core document mutation dependencies; require human review and fuzz/compat tests.
Tier 1: render/conversion adapters; require isolation and visual/compat tests.
Tier 2: UI/platform integrations; require platform permission tests.
Tier 3: dev tooling; can update more freely but still checked by cargo-deny/audit.
Tier 4: frontier experiments; feature-gated, removable, never default without evidence.
```
