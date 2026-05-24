# External Application Integrations

## Principles

- Integrate through local files, OS document providers, standard protocols, CLI, and safe automation first.
- Do not become a mandatory cloud-sync provider.
- Every integration maps to `ApplicationIntegrationContract` and `FeOperationIntent`.
- High-risk mutation requires review and approval.

## Integration targets

| Integration | Wave | Contract | Notes |
|---|---:|---|---|
| CLI | 0 | `contracts/cli/cli-contract.md` | Canonical test harness |
| Browser extension | 5 | `contracts/web/postmessage-contract.md` | Send current PDF/download to Fe Reader or Web Local |
| Web/PWA | 5 | Web contract | Local-only browser version, WASM where useful |
| Zotero | 5 | App integration contract | Export highlights/notes/citations, import metadata |
| Obsidian/Logseq | 4-5 | Conversion contract | Markdown notes with page/bbox links |
| LibreOffice/OnlyOffice | 4 | Conversion provider | DOCX/ODT conversion and document repair path |
| Pandoc | 4 | Conversion provider | Markdown/DOCX/HTML/LaTeX bridges |
| Quarto | 4-5 | Project integration | `.qmd` source-linked PDF workflows |
| Typst | 4-5 | Project integration | `.typ` source-linked PDF workflows |
| LaTeX | 4-5 | Project integration | `.tex` source-linked PDF workflows |
| VS Code / JetBrains | 5 | CLI/deeplink | Open page/bbox, export notes, preview docs |
| Email clients | 5 | OS share/open-with | Redact then attach, convert then attach |
| Nextcloud/WebDAV | 5 | Storage provider adapter | Prefer OS provider first |
| SharePoint/OneDrive/Google Drive/Dropbox | Later | OS provider first | Direct integration only if strong need |

## Application integration contract

See `contracts/rust/application_integration.rs` and `schemas/app-integration.schema.json`.

## Deep link format

```text
fe-reader://open?doc_sha256=<hash>&page=<zero_based_page>&bbox=x,y,w,h
fe-reader://workflow?workflow_id=<id>&document=<handle>
fe-reader://receipt?receipt_id=<id>
```

## Browser extension model

- Extension identifies embedded PDF or link to PDF.
- Extension offers “Open in Fe Reader”, “Open in Fe Reader Web Local”, “Extract metadata”, “Send to workflow”.
- Web extension cannot mutate local files directly; it must hand off to native app, web local with user-selected file, or self-hosted endpoint.
