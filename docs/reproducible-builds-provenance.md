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
C2PA / Content Credentials manifest where available
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
scripts/sbom_audit.sh
scripts/generate_provenance_attestation.sh
scripts/signing_readiness_check.sh
python3 scripts/release_provenance_check.py
python3 scripts/content_credentials_provenance_smoke.py
cargo cyclonedx --format json --all --output-file target/release-evidence/sbom.cdx.json
cargo vet
cargo deny check
cargo audit
cosign attest --predicate target/release-evidence/provenance.json <artifact-ref>
```

## Wave 0 provenance scaffold

Track AO records auditable placeholder evidence before real signing keys,
Cosign, SLSA or store credentials are available:

- `target/release-evidence/sbom-status.json` records whether CycloneDX SBOM generation ran or was skipped.
- `target/release-evidence/cargo-metadata.json` records dependency graph evidence when Cargo is available.
- `target/release-evidence/provenance.json` records source commit, workflow/run identity and hashed build materials. It is an in-toto-shaped placeholder, not a cryptographic attestation.
- `target/release-evidence/signing-readiness.json` records signing and notarization readiness without storing secrets.
- `target/release-evidence/provenance-readiness.json` records whether public-channel provenance requirements are satisfied.
- `target/release-evidence/content-credentials-provenance-smoke.json` records the contract-only C2PA / Content Credentials readiness smoke result. It validates the provenance scaffold and documents that Wave 0 does not emit a signed Content Credentials payload.
- `target/release-evidence/release-matrix.json` records the release packaging matrix validation across platforms and channels.
- `target/release-evidence/release-evidence.json` links the SBOM/status, provenance and signing-readiness evidence into the release bundle.
- `scripts/content_credentials_provenance_smoke.py` checks the provenance scaffold shape, docs boundary and release-material hashes without asserting a signed C2PA manifest.

Real public releases must replace advisory placeholders with CycloneDX SBOM,
artifact hashes, real signing/notarization receipts and provenance attestations.

## Content Credentials boundary

C2PA / Content Credentials provenance authoring is contract-only in this wave.
The current smoke evidence validates source commit, build material digests and
release documentation, but it is not a cryptographic C2PA manifest and does not
embed metadata into release artifacts. A later implementation needs a feature gate,
signing-material governance, rollback criteria and fixture coverage before it can
become release-blocking.

## Policy

- Local/nightly/dev builds may skip attestations.
- Public release builds must include at least SBOM, hashes and source commit.
- Store-submitted builds must preserve a release-evidence bundle even if the store modifies packaging.
- Automatic updates must never install an artifact that fails signature validation.

See `contracts/rust/release_evidence.rs` and `schemas/release-evidence.schema.json`.
