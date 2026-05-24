# Privacy-Preserving Quality Signals

Fe Reader should become reliable without collecting private documents. Diagnostics must be opt-in, minimised and locally inspectable.

## Local-first diagnostics

Default diagnostics are local files only:

```text
crash summary
feature flags
platform information
engine version
operation type
error category
performance counters
no document text
no page images
no file paths unless user permits
```

## User-visible diagnostic bundle

Users can generate a support bundle:

```bash
fe-reader diagnostics create --redact-paths --out fe-support.zip
```

The UI must show the bundle contents before export.

## Quality signal categories

| Category | Allowed default? | Notes |
|---|---:|---|
| Crash category | Local only | Upload only by explicit user action. |
| Performance counters | Local only | No document names or text. |
| Feature usage | Off | Optional product telemetry only with consent. |
| PDF compatibility failure | Local only | Include structural signatures, not content. |
| Security policy denial | Local only | Useful for debugging enterprise policy. |

## Enterprise controls

Enterprise policy can:

- disable all diagnostics export,
- force local-only diagnostics,
- set retention limits,
- prohibit path collection,
- prohibit plugin diagnostics,
- require signed support bundles.

## Implementation rule

No analytics SDK in core. If telemetry exists later, it must be an adapter behind `TelemetrySink` and disabled by default.
