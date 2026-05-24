# Content-Addressed Cache and Workspace

Fe Reader should feel instant after the first open. A content-addressed cache enables fast resume, thumbnails, search indexes, text layers, receipts and source-linked project metadata without mutating source PDFs.

## Cache principles

- Cache keys are derived from document bytes, operation version, renderer version and relevant settings.
- Cache data is local-first and privacy-sensitive.
- Cache may be disabled by enterprise policy.
- Cache can be cleared without losing source documents.
- Cache entries must never be trusted as a substitute for verifying the source document hash.

## Cache domains

| Domain | Examples |
|---|---|
| Rendering | tiles, thumbnails, page raster previews. |
| Text | text spans, word boxes, reading-order guesses. |
| Search | deterministic index shards and query statistics. |
| Metadata | parsed document info, XMP snapshots, standards reports. |
| Workflows | patch-plan drafts, receipts, template matches. |
| Authoring | source-linked build maps and output hashes. |
| Diagnostics | performance traces, crash recovery snapshots if opt-in. |

## Workspace catalogue

The workspace catalogue is local state, not cloud sync:

```text
recent documents
pinned documents
document hashes
sidecar locations
cache entries
workflow receipts
source-linked projects
review packets
```

## Storage strategy

- Use a small embedded store only in non-core crates.
- Core receives cache data as explicit inputs; it does not query the database directly.
- Consider `redb` or `sqlite` adapters later; do not force a database dependency into `fe_reader_core`.
- Use OS-provided app data directories.
- Use platform secure storage only for secrets, not bulk cache.

## Cache invalidation

A cache entry must include:

```text
input document hash
input document length
engine version
renderer version
operation version
feature flags
platform profile where relevant
```

## CLI examples

```bash
fe-reader cache status
fe-reader cache warm input.pdf --thumbnails --text --search
fe-reader cache clear --document input.pdf
fe-reader workspace list
fe-reader workspace forget input.pdf
```
