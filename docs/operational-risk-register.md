# Operational Risk Register

| Risk | Impact | Mitigation |
|---|---|---|
| PDF engine scope explosion | high | contracts, waves, baseline parity matrix, corpus gates |
| Rendering mismatch across platforms | high | PDFium baseline, visual regression, platform tolerance policy |
| Redaction false confidence | very high | secure rewrite, verification receipt, no annotation-only secure redaction |
| Dependency supply-chain issue | high | cargo-deny, cargo-vet, SBOM, provenance, fork policy |
| Automation API abuse | high | read-only default, approval tokens, policy engine |
| Poor mobile file handling | medium | SAF/document-browser contracts, persistent grants, no broad storage |
| Performance regressions | high | budgets, profiling, phase gates, release reports |
| PDF standard nonconformance | medium | veraPDF adapter, metadata/preflight track |
| Product identity too narrow | medium | workflow packs, domain-neutral positioning |
| ML distracts from core | medium | Wave 6 optional only, deterministic-first rule |
