# Installer and Publishing Plan

## Install modes

| Platform | Local/no-admin | Global/admin | Portable | Store/registry |
|---|---|---|---|---|
| Windows | NSIS per-user, Scoop, portable zip | MSI/MSIX, winget, Chocolatey | zip | Microsoft Store, winget, Scoop, Chocolatey, NuGet for bindings |
| macOS | `~/Applications`, Homebrew Cask | `/Applications` drag/install | zip/dmg | Mac App Store, Homebrew Cask |
| Linux | AppImage, Flatpak user, `~/.local` | deb/rpm, Flatpak system, Snap, distro packages | AppImage/tarball | Flathub, Snap Store, AUR, distro registries |
| Android | N/A | N/A | APK side-load dev | Google Play, evaluate F-Droid |
| iOS/iPadOS | N/A | N/A | TestFlight/dev only | App Store |

## Windows

Artifacts:

- `.msi`
- `.msix`
- NSIS `.exe`
- portable `.zip`
- Scoop manifest
- winget manifest
- Chocolatey package
- NuGet package for native/.NET wrapper. Wave 0 keeps `FeReader.Native` at `0.1.0-preview.1` without native runtime assets until platform packaging, signing and smoke tests are complete.

Per-user install should avoid admin rights and write to user locations. COM registration should support per-user registration where possible. Global install uses machine-wide registration and requires admin privileges.

## macOS

Artifacts:

- signed `.app`
- signed/notarized `.dmg`
- Homebrew Cask
- Mac App Store sandbox build
- optional CLI installer script

Local install: `~/Applications` and `~/.local/bin` or Homebrew user prefix. Global install: `/Applications` and `/usr/local/bin`/Homebrew managed path.

## Linux

Artifacts:

- AppImage
- Flatpak
- Snap
- `.deb`
- `.rpm`
- AUR PKGBUILD
- tarball

Local install: AppImage, Flatpak user, tarball to `~/.local`. Global install: system Flatpak, package manager install, Snap, deb/rpm.

## Android

- Debug APK.
- Release APK for internal testing.
- AAB for Google Play.
- Reproducible FOSS variant evaluation for F-Droid.

## iOS/iPadOS

- Development archive.
- TestFlight.
- App Store archive.
- Privacy manifest and sandboxed document access.

## Release channels

- `nightly`: unsigned/dev only where appropriate.
- `preview`: signed where possible, prerelease registries.
- `stable`: signed, notarized, registry/store published.

See `packaging/package-matrix.yaml` and `packaging/release-channels.yaml`.
