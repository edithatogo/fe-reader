---
title: Opt-in Collaboration and Sync
description: Local-first collaboration and sync governance for Fe Reader.
---

Fe Reader keeps collaboration local-first. The `opt_in_collaboration_sync` gate is disabled by default and does not block desktop stable launch.

The preview contract requires explicit opt-in, provider capability discovery, local-only defaults, support bundle exclusions and rollback that preserves local workspaces while purging provider tokens and sync metadata.

Cloud sync, background upload, analytics and phone-home behavior are not enabled by this track.

See the repository documentation for the full contract: [`docs/opt-in-collaboration-sync.md`](https://github.com/edithatogo/fe-reader/blob/main/docs/opt-in-collaboration-sync.md).
