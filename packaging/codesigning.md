# Code Signing and Trust

## Windows

- Sign MSI/MSIX/NSIS/portable binaries.
- Keep per-user and global installers separate.
- COM registration strategy must be explicit: per-user HKCU or global HKLM.

## macOS

- Sign app bundle.
- Notarize Developer ID distribution.
- Use separate entitlement profiles for direct distribution and Mac App Store.

## Linux

- Sign release artifacts/checksums.
- Flatpak/Snap/distro registries have their own trust pipelines.

## Android/iOS

- Use platform release signing.
- Store upload/signing keys securely.

## Signing Readiness Evidence

Release evidence records signing readiness without storing secrets:

- platform and artifact kind;
- required credential class;
- expected CI secret name or secure-store reference;
- current readiness status;
- signing, notarization or store receipt path when available;
- blocker reason when a required credential or receipt is absent.

Real private keys, signing certificates, notarization credentials and upload
keys must never be committed to the repository.
