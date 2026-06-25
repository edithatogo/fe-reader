# Track BP: macOS Native Functional Reader Parity Plan

## Phase BP1 - Track setup and lifecycle checkpoint

- [x] Task: Create macOS native functional reader parity track.
    - [x] Add specification, implementation plan, metadata, and index.
    - [x] Register the track in `conductor/tracks.md`.
    - [x] Commit with Conductor trailers and git note.

## Phase BP2 - Native reader canvas and command wiring

- [ ] Task: Replace preview-only canvas with a PDFKit reader surface.
    - [ ] Render opened PDFs with `PDFView`.
    - [ ] Add page navigation, current page display, zoom controls, fit width, and actual size.
    - [ ] Preserve keyboard paths for open, close, inspect, validate, search, zoom, and page navigation.
- [ ] Task: Wire core/CLI-backed reader actions.
    - [ ] Bundle or resolve the `fe-reader` CLI.
    - [ ] Connect Inspect, Metadata, Search, Accessibility, and Validate to deterministic CLI JSON when available.
    - [ ] Show clear fallback status when the CLI is unavailable.
- [ ] Task: Keep mutation and frontier boundaries intact.
    - [ ] Keep Redact disabled or plan-only.
    - [ ] Do not enable ML, RAG, telemetry, cloud sync, or hidden network behavior.
    - [ ] Keep all native behavior outside `fe_reader_core`.

## Phase BP3 - macOS verification and evidence

- [ ] Task: Harden native verification mode.
    - [ ] Write screenshots to `target/native-preview/` by default.
    - [ ] Add `--fixture` support for open-document smoke.
    - [ ] Avoid overwriting tracked screenshots unless explicitly requested.
- [ ] Task: Validate and review.
    - [ ] Run empty-state and fixture-open native UI smoke.
    - [ ] Run lifecycle, launch QA, stable reader readiness, and PDF parity checks.
    - [ ] Run `conductor-review` and apply fixes.

## Phase BP4 - Archive and closeout

- [ ] Task: Archive track after validation.
    - [ ] Mark all BP plan tasks complete.
    - [ ] Move track to `conductor/archive/track-BP-macos-native-functional-reader-parity/`.
    - [ ] Update `conductor/tracks.md`.
    - [ ] Commit, add git note, push commits and notes.
    - [ ] Confirm GitHub Actions pass for the final SHA.
