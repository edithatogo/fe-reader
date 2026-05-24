# Final v6 Improvements

v6 adds the product-readiness layer: user experience, accessibility, source-linked authoring, PDF optimisation, content-addressed workspaces, offline collaboration, quality signals and adoption operations.

## Why these improvements matter

The earlier package versions define a powerful engine. v6 makes the engine usable, supportable and differentiable:

- **Usability:** expert PDF operations need safe defaults, previews, warnings and receipts.
- **Accessibility:** Fe Reader must be accessible as an app, not merely capable of inspecting accessible PDFs.
- **Authoring:** PDFs increasingly originate from Markdown, Typst, Quarto, LaTeX and DOCX. Fe Reader should understand source-linked projects.
- **Optimisation:** many PDFs are bloated, non-linearised, image-heavy or slow to open. Fe Reader should explain and improve them.
- **Collaboration:** not all collaboration requires cloud sync. Fe Reader can exchange review packets, sidecars and signed comments offline.
- **Quality signals:** diagnostics, crash reports and performance data must be privacy-preserving, opt-in and useful for support.
- **Adoption:** implementation needs documentation, examples, tutorials and community process from the beginning.

## New tracks

| Track | Name | Purpose |
|---|---|---|
| Z | UX, Accessibility & Human Factors | Make workflows discoverable, keyboard-accessible, screen-reader compatible and user-tested. |
| AA | Source-Linked Authoring | Integrate with Typst, Quarto, LaTeX/Tectonic and Pandoc without putting converters in core. |
| AB | Cache, Workspace & Offline Collaboration | Content-addressed cache, workspace catalogue, annotation packets and mergeable review. |
| AC | PDF Optimisation & Linearisation | Size reduction, linearisation, dedupe, image/font optimisation and optimisation receipts. |
| AD | Toolchain Optimisation & Experimental Lanes | Build speed, release performance, PGO/BOLT, linkers, allocators and safe experimentation. |
| AE | Documentation, Training & Community | Developer docs, workflow docs, tutorials, governance and support playbooks. |

## v6 core rule

Every capability must have:

```text
contract -> implementation path -> CLI/test path -> UX path -> policy/performance note -> release evidence
```

Advanced features can be expert-only, but they cannot be invisible, unsafe or untestable.
