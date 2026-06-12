# Release Operations and Signed Updates

## Release channels

```text
nightly
alpha
beta
stable
lts-enterprise
```

## Artifacts

Each release produces:

- platform installers and portable packages
- detached checksums
- signed update manifest
- SBOM
- provenance/attestation artifact
- compatibility report
- performance report
- accessibility report
- security/dependency report

## Signed update contract

The app must never accept unsigned update metadata. Update manifests bind:

- channel
- version
- platform
- architecture
- artifact URL
- artifact SHA-256
- signature
- minimum compatible app version
- migration notes
- rollback status

## Rollout policy

- Nightly can auto-update within the nightly channel.
- Beta and stable require signed manifests.
- Enterprise/LTS can disable auto-update and use managed deployment.
- Rollback manifests are allowed only if explicitly signed and marked as rollback.

## Release readiness gate

A release is not ready unless:

- packaging matrix is complete for the target channel
- code signing/notarisation state is documented
- update manifest validates
- SBOM and provenance generated
- critical supply-chain checks pass
- P0/P1 performance budgets pass or have accepted waivers
- accepted corpus compatibility has no untriaged regression

## Wave 0 evidence definition

During bootstrap, "generated" SBOM/provenance means the release workflow emits
machine-readable evidence under `target/release-evidence/`. Dev and nightly
channels may use advisory SBOM status plus placeholder provenance and signing
readiness records. Preview, beta, stable, LTS and store-submission channels
must have real SBOM output and provenance/signing-readiness evidence before the
release readiness check passes.

## Evidence

- `scripts/wave7_release_hardening_smoke.py` validates the release evidence bundle, packaging templates and compatibility snapshots.
- `scripts/release_evidence_check.sh` validates the release evidence bundle shape, update-manifest schema and contract-input digests.
- `scripts/release_readiness_check.sh` validates the release readiness bundle against the release evidence schema and packaging/channel definitions.
- `scripts/wave4_distribution_smoke.py` validates packaging matrix structure, manifest syntax and release evidence shape.
- Release claims should stay bound to generated evidence under `target/release-evidence/`.
- Generated release evidence should include the release-evidence bundle, not just the update manifest.
