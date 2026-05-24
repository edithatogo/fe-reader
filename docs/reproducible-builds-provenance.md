# Reproducible Builds, Release Evidence and Provenance

## Goal

Every public release should ship with evidence explaining what was built, from which source, with which dependency graph, by which workflow, and with which signing/provenance artifacts.

## Release evidence bundle

Each release should include:

```text
source commit and tag
Cargo.lock
crate versions and feature flags
platform build matrix
installer hashes
SBOM
cargo-vet/audit/deny results
license report
SLSA/in-toto provenance where available
Sigstore/Cosign attestations where available
Tauri updater signature metadata
notarization/signing receipts for macOS and Windows
store submission metadata for app stores/registries
compatibility-corpus summary
performance summary
visual-regression summary
```

## Reproducibility strategy

| Stage | Target |
|---|---|
| Dev builds | Fast, debuggable, not reproducible. |
| CI candidate builds | Locked dependencies, scripted environment, generated SBOM. |
| Release candidate | Pinned toolchain, locked features, provenance, artifact hashes. |
| LTS/enterprise | Rebuildable environment, archived dependencies, signed release evidence. |

## Commands

```bash
scripts/release_readiness_check.sh
scripts/release_evidence_check.sh
cargo cyclonedx --format json --all --output-file target/release-evidence/sbom.cdx.json
cargo vet
cargo deny check
cargo audit
cosign attest --predicate target/release-evidence/provenance.json <artifact-ref>
```

## Policy

- Local/nightly/dev builds may skip attestations.
- Public release builds must include at least SBOM, hashes and source commit.
- Store-submitted builds must preserve a release-evidence bundle even if the store modifies packaging.
- Automatic updates must never install an artifact that fails signature validation.

See `contracts/rust/release_evidence.rs` and `schemas/release-evidence.schema.json`.
