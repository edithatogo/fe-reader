# Web Version Plan

## Products

| Product | Description | Priority |
|---|---|---|
| Fe Reader Web Viewer | Browser reader/annotation preview/search | Should |
| Fe Reader Web Local | Local-only PWA using file APIs and WASM where available | Should |
| Fe Reader Browser Extension | Open web PDFs in Fe Reader, extract metadata, hand off workflows | Could |
| Fe Reader Self-hosted | Optional server for teams/institutions | Later |

## Web constraints

- Browser file access requires user action and varies by browser.
- Web version must never imply it can persist arbitrary local access without user grant.
- Browser version can use PDF.js for rendering while using Rust/WASM for selected parsing/workflow logic.
- Browser extension must not bypass the operation-intent safety model.

## Web contract

See `contracts/web/postmessage-contract.md`.

## Web acceleration

- Early: browser Canvas/WebGL/WebGPU compositing experiments only.
- Later: WASM + WebGPU for local OCR/vector/annotation compositing where useful.
