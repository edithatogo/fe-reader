# Deterministic Search and Indexing Plan

## Goal

Provide excellent search and document-library features without early ML dependency.

## Architecture

```text
Text extraction spans
  -> normalisation/tokenisation
  -> local index writer
  -> encrypted/local workspace storage option
  -> search query parser
  -> page + bounding-box results
```

## Initial search features

- Exact text search.
- Case-insensitive search.
- Regex search for CLI/workflow recipes.
- Whole-word search.
- Phrase search.
- Metadata search.
- Annotation search.
- Attachment filename search.
- Page labels and outline search.

## Index features

Use a provider trait:

```rust
SearchIndexProvider
  - in_memory for tests
  - tantivy for local desktop/server index
  - platform index adapters later
```

The default early search path can be in-memory; `tantivy` is the long-lived local index candidate.

## Privacy rules

- No cloud index.
- User can disable indexing.
- User can purge per-document and per-workspace index records.
- Redacted documents must invalidate stale index entries for the original content.
- Workspaces may mark documents as `indexing_disallowed`.

## Later frontier

ML embeddings and semantic search may be added in Wave 6, but must be optional and use the same result evidence model: every result links to page, span and bounding box.
