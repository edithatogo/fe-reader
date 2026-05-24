#!/usr/bin/env bash
set -euo pipefail
test -f schemas/cache-manifest.schema.json
test -f schemas/review-packet.schema.json
test -f contracts/rust/cache_workspace.rs
test -f contracts/rust/offline_collaboration.rs
test -f docs/content-addressed-cache-workspace.md
test -f docs/offline-collaboration-review-packets.md
echo "cache/workspace smoke: ok"
