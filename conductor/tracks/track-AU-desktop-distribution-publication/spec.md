# Track AU: Desktop Distribution Publication

## Overview

Prepare the public desktop distribution surfaces for stable launch: GitHub Releases, Homebrew, Winget, Chocolatey, Scoop, Flatpak, Snap and AUR. Publication must remain blocked unless signed artifacts, checksums, release evidence and maintainer approval are present.

## Scope

- GitHub Release process.
- Changelog and release notes.
- Desktop registry manifests.
- Homepage and repository package links.
- Publication blocker tracking.

## Functional Requirements

- Validate desktop registry manifests against current version, artifact names and checksums.
- Add a release publication checklist with explicit maintainer approval.
- Update `packaging/registry-status.yaml` from generic deferred states to ready/published/external-blocked states.
- Document GitHub Release creation and registry submission commands.
- Ensure package links are discoverable from README/docs/homepage.

## Non-Functional Requirements

- Do not publish automatically without explicit approval.
- Do not store registry credentials in the repository.
- Every public listing must match product identity and license policy.

## Acceptance Criteria

- Registry manifests validate against release artifacts.
- GitHub Release process is scriptable and documented.
- Desktop package status is concrete per registry.
- Docs and repository homepage point at release/package locations.

## Out of Scope

- Mobile store launch.
- Cloud-hosted update service beyond signed manifest publication.

