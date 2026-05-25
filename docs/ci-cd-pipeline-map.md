# CI/CD Pipeline Map

## PR pipelines

1. Contract policy.
2. Rust stable build/test.
3. Security/supply chain.
4. Cross-platform smoke.
5. API compatibility when surfaces are stable.

## Nightly pipelines

1. Frontier Rust beta/nightly.
2. Miri and sanitizers.
3. Longer fuzzing.
4. Differential oracle checks.
5. Performance benchmarking.

## Release pipelines

1. Version and changelog verification.
2. Build platform artifacts.
3. Generate SBOM.
4. Generate provenance attestation.
5. Sign/notarize where applicable.
6. Publish release evidence bundle.
7. Submit package-manager manifests through separate human-approved steps.

## Required generated artifacts

- `release-evidence.json`.
- SBOM in CycloneDX and/or SPDX format.
- Build provenance attestation.
- Signing readiness or signing receipt evidence.
- API compatibility report.
- Dependency audit/vet report.
- Performance report.
- Compatibility corpus report.
- Accessibility report.
- Installer/package manifest report.
