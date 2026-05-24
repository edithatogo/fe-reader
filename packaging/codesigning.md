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
