# Sidecar Format Versioning and Migrations

Fe Reader will create sidecar formats for workspace state, review packets, receipts, templates, indexes and profiles. These must be versioned from the beginning.

## Sidecar families

| Family | Extension | Purpose |
|---|---|---|
| Workspace | `.feworkspace` | local workspace/project state |
| Review packet | `.fereview` | offline comments/proposed changes |
| Receipt | `.fereceipt` | operation evidence and verification |
| Template | `.fetemplate` | workflow/signature/redaction templates |
| Index | `.feindex` | deterministic local search/cache index |
| Profile | `.feprofile` | export/preflight/optimisation profile |

## Compatibility rules

- Major version changes may require explicit migration.
- Minor version changes must be readable by newer clients.
- Patch version changes must not alter semantics.
- Unknown fields should be preserved where possible for forward compatibility.
- Downgrade support is optional but must be declared.
- Sidecar migration never mutates the original PDF.

## Migration command targets

```bash
fe-reader sidecar inspect file.feworkspace
fe-reader sidecar migrate file.feworkspace --to 1.1.0 --out migrated.feworkspace
fe-reader sidecar verify file.fereceipt
```
