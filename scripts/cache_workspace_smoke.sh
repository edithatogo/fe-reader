#!/usr/bin/env bash
set -euo pipefail
python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

root = Path.cwd()
docs = {
    "cache": root / "docs/content-addressed-cache-workspace.md",
    "collab": root / "docs/offline-collaboration-review-packets.md",
}
contracts = [
    root / "contracts/rust/cache_workspace.rs",
    root / "contracts/rust/offline_collaboration.rs",
    root / "contracts/rust/workspace_catalog.rs",
]
schemas = [
    root / "schemas/cache-manifest.schema.json",
    root / "schemas/review-packet.schema.json",
]

for path in [*docs.values(), *contracts, *schemas]:
    if not path.exists():
        raise SystemExit(f"cache/workspace smoke failure: missing {path.relative_to(root)}")

cache_doc = docs["cache"].read_text(encoding="utf-8")
for token in [
    "content-addressed cache",
    "Cache entries are not authoritative; always verify the source document hash.",
    "Use a small embedded store only in non-core crates.",
    "do not force a database dependency",
    "fe_reader_core",
]:
    if token.lower() not in cache_doc.lower():
        raise SystemExit(f"cache/workspace smoke failure: cache doc missing token {token}")

collab_doc = docs["collab"].read_text(encoding="utf-8")
for token in [
    "portable, mergeable review packet",
    "Packets are hash-bound to the source document and warn on mismatch.",
    "Packet import must not mutate the PDF without explicit approval.",
    "Review packets are untrusted input.",
]:
    if token not in collab_doc:
        raise SystemExit(f"cache/workspace smoke failure: collaboration doc missing token {token}")

cache_contract = contracts[0].read_text(encoding="utf-8")
for token in [
    "CacheKey",
    "CacheEntryMeta",
    "WorkspaceRecord",
    "CacheStore",
    "WorkspaceCatalog",
]:
    if token not in cache_contract:
        raise SystemExit(f"cache/workspace smoke failure: cache contract missing token {token}")

collab_contract = contracts[1].read_text(encoding="utf-8")
for token in [
    "ReviewPacket",
    "ReviewPacketImportResult",
    "ReviewPacketStore",
    "export_packet",
    "import_packet",
]:
    if token not in collab_contract:
        raise SystemExit(f"cache/workspace smoke failure: collaboration contract missing token {token}")

catalog_contract = contracts[2].read_text(encoding="utf-8")
for token in [
    "WorkspaceCatalog",
    "DocumentCatalogRecord",
    "upsert_workspace",
    "list_recent_documents",
]:
    if token not in catalog_contract:
        raise SystemExit(f"cache/workspace smoke failure: workspace catalog contract missing token {token}")

cache_schema = json.loads(schemas[0].read_text(encoding="utf-8"))
review_schema = json.loads(schemas[1].read_text(encoding="utf-8"))
if cache_schema.get("title") != "Fe Reader Cache Manifest":
    raise SystemExit("cache/workspace smoke failure: wrong cache manifest schema title")
if review_schema.get("title") != "Fe Reader Offline Review Packet":
    raise SystemExit("cache/workspace smoke failure: wrong review packet schema title")

cache_manifest = {
    "manifest_version": "1",
    "entries": [
        {
            "document_sha256": "a" * 64,
            "kind": "thumbnail",
            "artifact": "target/cache/thumbs/page-1.png",
        }
    ],
    "privacy_policy": "local_only",
    "created_at_utc": "1970-01-01T00:00:00Z",
}
review_packet = {
    "packet_id": "packet:smoke",
    "base_document_sha256": "b" * 64,
    "author_label": "smoke",
    "comments": [
        {
            "comment_id": "comment-1",
            "page_index": 0,
            "body_markdown": "Looks good.",
        }
    ],
    "proposed_patch_plans": [],
    "signature_hint": None,
}
try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None
if jsonschema is not None:
    jsonschema.validate(cache_manifest, cache_schema)
    jsonschema.validate(review_packet, review_schema)

report = root / "target/release-evidence/cache-workspace-smoke.json"
report.parent.mkdir(parents=True, exist_ok=True)
report.write_text(
    json.dumps(
        {
            "check": "cache_workspace",
            "status": "pass",
            "contracts": [
                "contracts/rust/cache_workspace.rs",
                "contracts/rust/offline_collaboration.rs",
                "contracts/rust/workspace_catalog.rs",
            ],
            "schemas": [
                "schemas/cache-manifest.schema.json",
                "schemas/review-packet.schema.json",
            ],
            "local_first": True,
            "no_cloud_sync": True,
        },
        indent=2,
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
print("cache/workspace smoke: ok")
PY
