# Opt-in Collaboration and Sync

Fe Reader remains local-first. The `opt_in_collaboration_sync` feature gate is disabled by default, does not block desktop stable launch and must never enable silent upload, background sync, analytics or phone-home behavior.

## Contract Boundary

Collaboration starts with portable review packets, workspace sidecars and local cache metadata. These artifacts are privacy-sensitive and are treated as untrusted input on import.

| Artifact | Boundary | Default behavior |
|---|---|---|
| Review packets | Comments, annotations, proposed patch plan IDs and provenance hints | Local export/import only; no automatic apply |
| Cache entries | Derived thumbnails, indexes and diagnostics | Local-only, content-addressed and removable |
| Quality signals | Local evidence about rendering, parsing or workflow quality | Opt-in, local-first and excluded from support bundles unless explicitly included |
| Sync metadata | Provider capability results, conflict state and remote path hints | Disabled until a user explicitly connects a provider |

All mutating collaboration flows still use `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`. Review packets can carry proposed patch plans, but packet import cannot mutate a PDF without document hash match, policy evaluation, approval and audit receipt emission.

## Provider Discovery

Sync providers are capability-discovered, not assumed. A provider starts unavailable until the user explicitly opts in from UI or CLI.

| Provider | Status | Failure modes |
|---|---|---|
| `local_folder` | Preview planning only | Missing path, permission denied, hash mismatch |
| `git_folder` | Preview planning only | Git unavailable, dirty worktree, merge conflict |
| `webdav_nextcloud_future` | Future network lane | Offline, auth required, quota exceeded, conflict requiring manual packet merge |

Network providers require a separate ADR before promotion. Their default upload mode is `never`; connection, upload and token storage all require explicit user action.

## Privacy And Support

support bundles exclude document bytes, document text, review-packet bodies, cache payloads, provider tokens, provider account IDs and sync remote paths by default. A support workflow may report that a sync provider is unavailable, but it must not include private packet contents or account identifiers.

## Rollback

Rollback disables sync providers and retains local workspaces, review packets and audit receipts. It purges provider tokens, sync metadata and remote path hints. Local-only mode remains the default before, during and after rollback.

## Evidence

The preview contract is machine-readable at `contracts/snapshots/collaboration/opt-in-collaboration-sync.preview.json`.

Run:

```bash
python3 scripts/opt_in_collaboration_sync_check.py
```
