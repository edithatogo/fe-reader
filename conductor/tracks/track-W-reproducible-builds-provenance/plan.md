# Track W: Reproducible Builds & Release Evidence

## Mission

Ensure release artifacts have evidence: source, hashes, SBOM, provenance, signatures, audit reports, performance and compatibility summaries.

## Phases

### W0 Evidence schema

Deliver `schemas/release-evidence.schema.json`, `contracts/rust/release_evidence.rs` and `scripts/release_evidence_check.sh`.

### W1 SBOM and dependency evidence

Generate CycloneDX SBOM and cargo-vet/audit/deny reports.

### W2 Artifact signing and attestations

Add placeholders for Cosign/in-toto/SLSA-aligned attestations and Tauri update signatures.

### W3 Rebuildability

Document toolchain, environment and installer reproducibility requirements.

### W4 Release packet

Bundle all release evidence into `target/release-evidence/`.
