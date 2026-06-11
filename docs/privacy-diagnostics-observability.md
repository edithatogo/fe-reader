# Privacy, Diagnostics and Observability

## Principle

Diagnostics are local-first. Fe Reader must be useful to debug without silently exporting private document content.
The default privacy posture is local inspection first, with explicit user or managed-policy action before any export.

## Allowed local diagnostics

- timing spans
- operation IDs
- error classes
- feature flags
- dependency versions
- crash stack traces with document paths redacted by default
- performance counters

## Disallowed by default

- document text
- rendered page images
- extracted entities
- file paths containing names or sensitive folders
- raw PDFs
- search index contents
- redaction candidates

## Diagnostic bundle

Users may explicitly generate a support bundle. It must show a preview of what will be included and offer redaction of paths, usernames and document titles.

## Observability contract

Use `tracing` internally, with a policy-controlled export layer. Logs are local by default. Remote upload requires explicit user action or managed enterprise policy.

## Evidence

- `scripts/quality_dashboard_smoke.py` confirms the dashboard and privacy docs remain aligned with local-first observability.
- No document text, rendered page images or search index contents should enter the default diagnostic path.
