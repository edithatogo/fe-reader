# v6 Coding Agent Start Here

Read this after `IMPLEMENTATION_PROMPT.md`.

## Do first

1. Materialise sample crate manifests.
2. Run schema validation.
3. Create the initial CLI skeleton.
4. Create the operation/patch/transaction contracts.
5. Create cache/workspace/accessibility/source-linked/optimisation contracts as stubs.
6. Run the phase gate.

## Do not do early

- Do not implement local LLM/RAG features before deterministic search and redaction work.
- Do not implement source builds as arbitrary shell command execution.
- Do not make cache data authoritative.
- Do not make review packets mutate PDFs on import.
- Do not enable optimisation that invalidates signatures without a receipt.
- Do not make GPU rendering mandatory.

## Initial implementation order

```text
A0 core contracts
K/O security contracts
P corpus scaffold
N performance budgets
V API stability
Z accessibility command registry
AB cache/workspace manifest
AC optimisation plan schema
AA source-linked project schema
AD experiment registry
AE docs skeleton
```
