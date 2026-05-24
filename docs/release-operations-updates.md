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
