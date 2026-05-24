# Local Intelligence Future Roadmap

Local ML, local LLMs, and RAG are deliberately **not early priorities**. Fe Reader should be valuable without them.

## Wave 0-5: prepare only

- Define `EntityDetector` trait.
- Define `SearchIndex` trait.
- Define evidence objects: page, span, bounding box, confidence, source detector.
- Keep deterministic detectors first.
- Do not add Candle/Burn dependencies to `fe_reader_core`.

## Wave 6+ options

| Feature | Description | Guardrail |
|---|---|---|
| Local NER redaction assist | Detect likely names/orgs/addresses in extracted spans | Human review required |
| Local embeddings | Semantic search over local document spans | No cloud dependency |
| Grounded Q&A | Answers cite page/bbox evidence | Read-only by default |
| Workflow suggestion | Suggest matching workflow pack/template | No automatic mutation |
| Conversion repair | Suggest fixes to Markdown/DOCX tables | User accepts diff |

## Optional crates

```text
fe_reader_intelligence
fe_reader_nlp_candle
fe_reader_embeddings
fe_reader_local_qa
```

## Model policy

- Models are optional downloads.
- Model provenance must be displayed.
- Model outputs are suggestions, not authority.
- High-risk suggestions must produce patch plans and require approval.
