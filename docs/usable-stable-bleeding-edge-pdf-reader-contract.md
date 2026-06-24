# Usable Stable Bleeding-Edge PDF Reader Contract

Fe Reader is considered to have reached the usable stable bleeding-edge PDF reader contract only when all of the following are true:

- The reader baseline is documented in `docs/stable-reader-readiness.md`.
- The stable desktop release boundary is documented in `docs/stable-desktop-release.md`.
- Launch QA passes through `scripts/launch_qa_check.py`.
- Stable reader readiness passes through `scripts/stable_reader_readiness_check.py`.
- Stable publication remains gated by signed artifacts, checksums, release evidence and registry approval.
- Public marketing claims remain constrained by `docs/marketing-readiness.md`.

## Reader capabilities

The contract covers:

- opening local PDFs from CLI and preview entry points;
- deterministic metadata and search diagnostics;
- page navigation, zoom, fit, rotate and thumbnail inspection;
- accessibility evidence for the baseline reader;
- safe-open diagnostics and documented limitations for incomplete workflows.

## Contract boundary

This contract does not claim that every PDF family is fully supported. It requires evidence-backed limitations for unfinished areas and keeps stable release, marketing readiness and post-launch parity claims separate.

## Evidence

- `target/release-evidence/stable-reader-readiness.json`
- `target/release-evidence/launch-qa.json`
- `target/release-evidence/release-readiness.json`
- `target/release-evidence/stable-release-evidence.json`
- `docs/marketing-readiness.md`
