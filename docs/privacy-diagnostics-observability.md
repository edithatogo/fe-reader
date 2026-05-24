# Privacy, Diagnostics and Observability

## Principle

Diagnostics are local-first. Fe Reader must be useful to debug without silently exporting private document content.

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
