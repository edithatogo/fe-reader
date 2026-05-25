# Libraries, Forks, Submodules, and Contribution Strategy

## Default policy

Prefer upstream crates and contribute fixes upstream. Fork only when:

- a blocking bug exists;
- upstream is inactive or cannot accept needed changes;
- security patching requires immediate control;
- a private branch is needed temporarily while an upstream PR is pending.

Every fork must have:

- upstream URL;
- reason;
- owner;
- divergence policy;
- rebase cadence;
- exit criteria.

## Core library choices

| Area | Library | Version target | Usage | Fork? |
|---|---|---:|---|---|
| Low-level PDF manipulation | `lopdf` | `0.40.0` | Object-level edits, page ops, low-level preservation | No by default |
| PDF object writing | `pdf-writer` | `0.14.0` | Structured writing of new objects/content | No by default |
| Text/layout extraction | `pdf_oxide` | `0.3.47` target | Extraction, markdown/HTML experiments, validation ideas | No by default; evaluate as dependency and contribution target |
| Rendering | `pdfium-render` | `0.9.1` | PDFium adapter | No; isolate FFI |
| GPU/compositor | `wgpu` | `29.0.3` target | Optional GPU compositor | No |
| Vector compositor | `vello` | `0.8.0` target | Overlay/vector experiment | No |
| Skia binding | `skia-safe` | `0.93.1` target | Optional experiment only | No |
| Bindings | `uniffi` | `0.31.1` target | Swift/Kotlin/Python/Ruby; C# via third-party/fallback | No |
| MCP | `rmcp` | `1.6.0` target | MCP server | No |
| Plugins | `extism` | deferred | WASM plugin MVP once a non-vulnerable Wasmtime-backed release is available | No |
| Preflight | `veraPDF` | external adapter | PDF/A/PDF/UA validation | No submodule; CLI adapter |
| Conversion | `pandoc` | external provider | Markdown/DOCX/HTML/LaTeX conversion | No submodule |
| Office conversion | LibreOffice | external provider | DOCX/ODT conversion fallback | No submodule |

## Libraries to study, not fork initially

- Existing Tauri/Svelte PDF annotation apps: useful UI references, not core foundation.
- Rust PDF viewers using Iced/Dioxus: useful rendering/UI ideas, not default path.
- Existing PDF CLI tools: useful for fixtures and expected behaviours.

## C# strategy

UniFFI is first-party for Swift/Kotlin/Python/Ruby. C# must be treated separately:

1. evaluate `uniffi-bindgen-cs` or equivalent third-party generator;
2. maintain a preview `extern "C"` ABI fallback in `fe_reader_c_abi`, with C# wrappers using P/Invoke rather than UniFFI assumptions;
3. publish a `.NET` wrapper through NuGet in Wave 4-5.

## Fork manifest

See `third_party/fork-policy.yaml`.

## v4 additional library choices

| Area | Library | Version target | Usage | Fork? |
|---|---|---:|---|---|
| Local deterministic search | `tantivy` | `0.26.1` | Optional local full-text index outside core | No |
| Rust text layout | `cosmic-text` | `0.19.0` | Text overlay/i18n experiments, diagnostics | No |
| Text shaping | `rustybuzz` | `0.20.1` | HarfBuzz-compatible shaping when needed | No |
| Secure local secrets | `keyring` | `4.0.1` | Platform keychain adapters only | No |
| Linux/BSD/mac notifications | `notify-rust` | `4.17.0` | Desktop notifications adapter | No |
| Linux D-Bus | `zbus` | `5.15.0` | D-Bus automation adapter | No |
| Tauri signed updater | `tauri-plugin-updater` | `2.10.1` | Desktop update manifest integration | No |
| SBOM | `cargo-cyclonedx` | tool | Release SBOM generation | No |
| Supply-chain audits | `cargo-vet`, `cargo-deny`, `cargo-audit` | tools | Release and CI gates | No |

## Git dependency policy

Git dependencies are allowed only for frontier experiments or urgent unreleased fixes. They must include:

- pinned revision hash;
- owner;
- reason;
- expected upstream release or PR;
- removal deadline;
- feature gate;
- tests proving behaviour.

Git dependencies are disallowed in `fe_reader_core` except under an explicit emergency security waiver.


## v6 additions

- Add `tokio = 1.51.0` only in job, automation, local API and platform crates; never in the pure PDF model unless a specific async boundary is required.
- Keep `schemars` on the stable line until v1 stabilisation is validated in the workspace; use `jsonschema` as an external validation crate/tool for schemas.
- Add `tracing-opentelemetry` only to diagnostics/export adapters. Local diagnostics remain opt-in and privacy preserving.
- Do not add a direct dependency on Tauri, PDFium, wgpu, vello, MCP, Extism, Wasmtime, Candle or Burn to `fe_reader_core`.
