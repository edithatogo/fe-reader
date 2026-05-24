# Platform Integration Plan

## Cross-platform contract

All platforms implement `contracts/rust/platform_integration.rs`. The core sees only `PlatformIntegration`, not OS APIs.

## Windows 11+

### Wave 1

- File association for `application/pdf`.
- Recent documents via Shell recent-doc integration.
- Open-with and drag/drop.
- Native print handoff.
- Local notification for long operations.
- Secure storage using DPAPI or Windows credential APIs through platform adapter.

### Wave 4

- NSIS per-user install, MSI/MSIX global install, portable zip.
- Winget, Scoop, Chocolatey, NuGet native bindings.
- Code signing.

### Wave 5

- COM Automation, out-of-process server preferred.
- PowerShell module wrapper around CLI and/or COM.
- Optional Windows Search IFilter for text/metadata indexing.
- Explorer preview/thumbnail handler evaluation.

### Automation contract

Use `contracts/platform/windows-com/FeReaderAutomation.idl`.

High-risk COM methods return `PatchPlanId` and require explicit approval token before apply.

## macOS

### Wave 1

- Document app behaviours: Open Recent, drag/drop, file association.
- Sandbox-aware file access with security-scoped bookmarks.
- Native notifications.
- Keychain for secrets.
- Printing.

### Wave 4

- Signed, notarized `.app` and `.dmg`.
- Homebrew Cask.
- Mac App Store sandbox variant.
- Per-user `~/Applications` install and global `/Applications` install.

### Wave 5

- AppleScript dictionary (`.sdef`).
- App Intents / Shortcuts for modern automation.
- Spotlight indexing with page/annotation deep links.
- Quick Look previews for Fe workflow/receipt/template files.

### Automation contract

Use `contracts/platform/macos-applescript/FeReader.sdef` and `contracts/platform/ios-appintents/FeReaderAppIntents.swift` as model contracts.

## Linux

### Wave 1

- XDG MIME association and `.desktop` file.
- Recent documents through freedesktop bookmark/recent files.
- Portal-first file picker for sandboxed environments.
- Native notifications via DBus/portals.
- Printing via CUPS/portal path.

### Wave 4

- AppImage portable build.
- Flatpak and Flathub.
- Snap.
- `.deb`, `.rpm`, and AUR packaging.
- User-local install into `~/.local/bin` and `~/.local/share/applications`.

### Wave 5

- D-Bus automation service.
- Optional Tracker/Baloo indexing adapter.
- Thumbnailer integration.

### Automation contract

Use `contracts/platform/linux-dbus/org.fereader.FeReader1.xml`.

## Android

### Wave 1

- Storage Access Framework open/save.
- Persistable URI permissions.
- Share target for PDFs.
- Open-with PDF intent filters.
- Notification permission flow.
- Printing.

### Wave 4

- Google Play distribution.
- F-Droid evaluation for fully FOSS build variant.
- APK/AAB release signing.

### Wave 5

- `DocumentsProvider` for Fe-managed local workspace.
- AppSearch indexing of titles, tags, annotations, extracted text, workflow states.
- Android intents for automation-style actions.
- Work profile support.

### Automation contract

Use `contracts/platform/android-intents/AndroidManifest.contract.xml`.

## iOS / iPadOS

### Wave 1

- Document browser shell.
- Files app integration.
- Share/open-in support.
- Security-scoped file access via document architecture.
- Printing/share sheet.

### Wave 2-3

- PencilKit adapter for high-quality ink/signature capture.
- Keychain for secrets.

### Wave 4

- App Store distribution.
- TestFlight.
- Signed app archive and privacy manifest.

### Wave 5

- App Intents/Shortcuts.
- Spotlight indexing with page/annotation deep links.
- Share extension.
- Quick Look / preview integration where appropriate.

### Automation contract

Use `contracts/platform/ios-appintents/FeReaderAppIntents.swift`.
