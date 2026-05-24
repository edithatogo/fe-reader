# macOS Notarization Notes

- Direct-distribution `.app`/`.dmg` must be signed and notarized.
- Mac App Store build uses a stricter sandbox entitlement set.
- Homebrew Cask should point to the notarized DMG.
- AppleScript automation requires explicit entitlements and user consent.
