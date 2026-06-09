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

Strict release checks are scoped by `FE_RELEASE_TARGETS`, a comma-separated
platform list such as `linux,android`. Use `all` only when Windows, macOS,
Linux, Android and iOS credentials are all present. Apple notarization and iOS
distribution may remain deferred while Developer Program/App Store Connect
verification is pending, but those platforms must not be included in a public
release target list until their credentials and receipts exist.

Real private keys, signing certificates, notarization credentials and upload
keys must never be committed to the repository.
