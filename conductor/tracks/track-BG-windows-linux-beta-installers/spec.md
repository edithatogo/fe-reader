# Track BG: Windows and Linux Beta Installers

## Overview

Make Windows and Linux genuinely installable and smoke-tested, while clearly labeling them beta-quality until they reach the macOS public-quality bar.

## Functional Requirements

- Produce Windows MSI/MSIX and portable ZIP artifacts with checksums.
- Produce Linux AppImage, tarball and package artifacts where supported by local/GitHub build environments.
- Validate install, launch, open-file and CLI smoke paths for each platform.
- Keep registry manifests blocked until artifact URLs, checksums, signing state and maintainer approval exist.

## Non-Functional Requirements

- Windows signing is required before stable-quality claims; unsigned beta builds must be clearly labeled.
- Linux package claims must match actual artifact formats and tested environments.
- GitHub Actions permissions remain minimal and explicit.

## Acceptance Criteria

- Windows and Linux `NOT_AN_INSTALLER` placeholders are superseded by real beta artifacts or documented as still blocked.
- `release_artifact_inventory` records artifact presence and checksum matches.
- Registry status distinguishes beta artifact availability from stable registry publication.

## Out of Scope

- Microsoft Store, Flathub, Snap Store, AUR or distro repository publication before external review gates.
