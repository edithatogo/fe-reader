# Track Q: Search, Text, Fonts & I18N Spec

Track Q owns deterministic search, text diagnostics, Unicode, CJK/RTL, font fallback and accessibility text inspection. It explicitly avoids early ML/RAG.

## Deliverables

- `fe_reader_search` crate.
- `fe_reader_text` crate.
- in-memory search provider for tests.
- optional Tantivy provider.
- CJK/RTL/complex text fixtures.
- reading-order diagnostic UI/CLI outputs.

## Dependencies

- A text-span model.
- B rendering/text-layer alignment.
- G accessibility/tag-tree inspection.
