# Track AV: Enterprise Operations Readiness

## Overview

Make the desktop stable release enterprise-ready with offline install, per-user/global install validation, managed deployment policy templates, signed update/rollback validation, support bundle generation and diagnostics review.

## Scope

- Offline installer workflows.
- Per-user and global install modes.
- Enterprise deployment policy templates.
- Signed update and rollback validation.
- Support bundle and diagnostics evidence.

## Functional Requirements

- Validate offline installation for each desktop platform or record exact platform blockers.
- Validate enterprise policy loading and precedence.
- Validate signed update and rollback manifests.
- Add support bundle smoke evidence.
- Add diagnostics review documentation that avoids collecting secrets or private document content.

## Non-Functional Requirements

- Enterprise diagnostics are opt-in and local-first.
- Support bundles must not include document contents by default.
- Policy precedence must be deterministic and test-covered.

## Acceptance Criteria

- Enterprise readiness smoke passes locally and in CI.
- Offline install and rollback/update workflows have evidence.
- Deployment templates are documented and validated.
- Support bundle output is privacy-reviewed.

## Out of Scope

- Mandatory cloud management.
- Remote analytics.
- Mobile device management launch.

