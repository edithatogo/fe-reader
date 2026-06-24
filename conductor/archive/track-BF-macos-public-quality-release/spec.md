# Track BF: macOS Public Quality Release

## Overview

Make macOS the first public-quality launch platform. The product may still label Windows and Linux as beta-quality, but macOS must be polished, signed, notarized and installable before mature marketing.

## Functional Requirements

- Build a macOS app bundle with stable reader baseline functionality.
- Produce signed and notarized DMG and optional PKG artifacts.
- Support local installation, app launch, file open, file association strategy, recent files and privacy-safe permissions.
- Run macOS UI smoke tests and screenshot checks against the stable reader baseline.
- Publish checksum, signing, notarization and release evidence.

## Non-Functional Requirements

- No Apple credentials, private keys or notarization secrets may be committed.
- macOS automation and AppleScript remain read-only by default unless mutation approval contracts are satisfied.
- The release must pass accessibility, keyboard and privacy review.

## Acceptance Criteria

- `target/release-artifacts/` contains signed/notarized macOS release artifacts and checksums for the release version.
- GitHub Release and Homebrew status are updated only after artifact and approval evidence exist.
- Placeholder macOS prerelease assets are removed or superseded by real artifacts.

## Out of Scope

- Mac App Store publication unless Developer Program and App Store Connect gates are resolved.
- iOS public launch.
