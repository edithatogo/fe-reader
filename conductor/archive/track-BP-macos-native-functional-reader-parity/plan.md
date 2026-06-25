# Track BP: macOS Native Functional Reader Parity Plan

## Phase BP1 - Track setup and lifecycle checkpoint

- [x] Task: Create macOS native functional reader parity track.
    - [x] Add specification, implementation plan, metadata, and index.
    - [x] Register the track in `conductor/tracks.md`.
    - [x] Commit with Conductor trailers and git note.

## Phase BP2 - Native reader canvas and command wiring

- [x] Task: Replace preview-only canvas with a PDFKit reader surface.
    - [x] Render opened PDFs with `PDFView`.
    - [x] Add page navigation, current page display, zoom controls, fit width, and actual size.
    - [x] Preserve keyboard paths for open, close, inspect, validate, search, zoom, and page navigation.
- [x] Task: Wire core/CLI-backed reader actions.
    - [x] Bundle or resolve the `fe-reader` CLI.
    - [x] Connect Inspect, Metadata, Search, Accessibility, and Validate to deterministic CLI JSON when available.
    - [x] Show clear fallback status when the CLI is unavailable.
- [x] Task: Keep mutation and frontier boundaries intact.
    - [x] Keep Redact disabled or plan-only.
    - [x] Do not enable ML, RAG, telemetry, cloud sync, or hidden network behavior.
    - [x] Keep all native behavior outside `fe_reader_core`.

## Phase BP3 - macOS verification and evidence

- [x] Task: Harden native verification mode.
    - [x] Write screenshots to `target/native-preview/` by default.
    - [x] Add `--fixture` support for open-document smoke.
    - [x] Avoid overwriting tracked screenshots unless explicitly requested.
- [x] Task: Validate and review.
    - [x] Run empty-state and fixture-open native UI smoke.
    - [x] Run lifecycle, launch QA, stable reader readiness, and PDF parity checks.
    - [x] Run `conductor-review` and apply fixes.

## Phase BP4 - Archive and closeout

- [x] Task: Archive track after validation.
    - [x] Mark all BP plan tasks complete.
    - [x] Move track to `conductor/archive/track-BP-macos-native-functional-reader-parity/`.
    - [x] Update `conductor/tracks.md`.
    - [x] Commit, add git note, push commits and notes.
    - [x] Push the final SHA and monitor GitHub Actions to completion.
