# Reference Notes for Maintainers

This file records source categories used when creating this package. Check current upstream docs before locking versions or publishing.

- Tauri v2 docs: desktop/mobile support, webview architecture, bundling.
- UniFFI docs: language support and version compatibility.
- Conductor docs: context/spec/plan/implement workflow and review capability.
- PDF Association resources: PDF 2.0, PDF/A, PDF/UA, Arlington PDF Model.
- veraPDF resources: PDF/A and PDF/UA validation.
- Platform vendor docs: Windows Shell/COM/App SDK, Apple AppKit/UIKit/App Intents/PencilKit, Android SAF/AppSearch/DocumentsProvider, XDG portals.
- Packaging docs: winget, Scoop, Homebrew, Flathub, Snapcraft, App Store Connect, Google Play Publishing API.
- Rust security tooling: cargo-deny, cargo-vet, cargo-fuzz, OSS-Fuzz.

## v4 reference additions

- Tauri updater plugin: signed update manifests and artifact signatures.
- cargo-vet: human dependency audit evidence.
- cargo-deny: license, advisory and duplicate dependency gates.
- GitHub artifact attestations / SLSA provenance: release provenance.
- Tantivy: deterministic local full-text indexing.
- cosmic-text and rustybuzz: complex text layout and shaping.
- PDF/UA-2 and PDF/A-4 resources: accessibility and preservation targets.


## v5 references to verify during implementation

- qpdf manual and `qpdf --check` for structural PDF sanity checks.
- veraPDF for PDF/A and PDF/UA validation.
- cargo-semver-checks for Rust public API compatibility checks.
- SLSA and in-toto/Cosign for build provenance and release attestations.
- CycloneDX/cargo-cyclonedx for SBOM generation.
- memmap2 for optional memory-mapped I/O behind strict resource limits.


## v6 references to verify during implementation

- qpdf documentation for linearisation and content-preserving PDF transformations.
- W3C WCAG 2.2 for web/PWA accessibility targets.
- Typst documentation and CLI for source-linked authoring provider behaviour.
- Tectonic documentation for self-contained TeX/LaTeX provider behaviour.
- Pandoc documentation for conversion provider capabilities.
- Tauri updater documentation for signed updater requirements.
- Rust release/changelog sources for pinned toolchain decisions.
