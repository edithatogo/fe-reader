# Track BU: Web/PWA and Browser Extension Parity

## Overview

Add web-local/PWA and browser-extension parity surfaces that use explicit user grants and the postMessage safety contract.

## Functional Requirements

- Add a web-local/PWA reader surface for open, inspect, search, validate and plan-only workflows.
- Add a browser-extension handoff surface for embedded PDF/link inspection and plan handoff.
- Upgrade the postMessage contract compatibly while preserving current safety rules.
- Add Playwright or equivalent smoke coverage for web/PWA and browser-extension contract behavior.

## Non-Functional Requirements

- No hidden uploads, analytics, background sync or persistent local access without user grant.
- Browser extension direct local mutation remains out of scope.

## Acceptance Criteria

- Web/PWA smoke validates open/search/inspect/validate on the baseline fixture.
- Browser-extension smoke validates inspect and plan handoff without direct mutation.
- `scripts/web_postmessage_contract_smoke.py` and `scripts/browser_extension_contract_smoke.py` pass.

## Out of Scope

- Cloud hosting as a default path.
- Extension store publication.
