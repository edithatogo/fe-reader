# Desktop Release Packaging and Signing

Fe Reader desktop stable release artifacts are defined in `packaging/package-matrix.yaml`
under `desktop_artifacts`. The matrix is the source of truth for artifact kind,
path pattern, checksum path, signing policy, notarization policy and registry
targets.

## Required desktop artifacts

| Platform | Required artifact kinds | Trust evidence |
| --- | --- | --- |
| macOS | `dmg`, `pkg` | Developer ID signing and notarization |
| Windows | `msi`, `msix`, `portable_zip` | Authenticode signing |
| Linux | `appimage`, `deb`, `rpm`, `tarball` | Detached checksum and Linux signing policy |

## Local checks

```bash
python3 scripts/desktop_packaging_signing_check.py
python3 scripts/release_matrix_check.py
bash scripts/signing_readiness_check.sh
bash scripts/release_readiness_check.sh
```

Stable release checks must run with explicit desktop targets:

```bash
FE_RELEASE_CHANNEL=stable FE_RELEASE_TARGETS=macos,windows,linux bash scripts/signing_readiness_check.sh
FE_RELEASE_CHANNEL=stable FE_RELEASE_TARGETS=macos,windows,linux python3 scripts/desktop_packaging_signing_check.py
FE_RELEASE_CHANNEL=stable FE_RELEASE_TARGETS=macos,windows,linux bash scripts/release_readiness_check.sh
```

The stable signing check is expected to fail until required signing and
notarization secret references are configured outside the repository.

## Secret handling

Never commit certificates, private keys, notary credentials, passwords, upload
keys or token values. Release evidence records secret names and readiness states,
not secret values.

## Evidence

`scripts/desktop_packaging_signing_check.py` writes
`target/release-evidence/desktop-packaging-signing.json`. The release readiness
gate includes this evidence so stable desktop release cannot proceed without the
desktop artifact matrix and concrete registry readiness states.
