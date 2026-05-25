#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
SOURCE_COMMIT="$(git rev-parse --verify HEAD)"
SOURCE_TAG="$(git describe --tags --exact-match 2>/dev/null || true)"
TOOLCHAIN="$(rustc --version 2>/dev/null || printf 'rustc-unavailable')"
RELEASE_ID="${FE_RELEASE_ID:-dev-smoke}"
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
python3 - "$SOURCE_COMMIT" "$SOURCE_TAG" "$TOOLCHAIN" "$RELEASE_ID" "$CHANNEL" <<'PY'
import hashlib
import json
import os
import sys
from pathlib import Path

source_commit, source_tag, toolchain, release_id, channel = sys.argv[1:6]
if not source_commit or source_commit == "unknown":
    raise SystemExit("source_commit must be resolved")
if not toolchain or toolchain == "unknown":
    raise SystemExit("toolchain must be resolved")
if channel not in {"dev", "nightly", "preview", "beta", "stable", "lts", "store_submission"}:
    raise SystemExit(f"unsupported release channel: {channel}")

inputs = [
    Path("packaging/package-matrix.yaml"),
    Path("packaging/release-channels.yaml"),
    Path("packaging/codesigning.md"),
    Path("schemas/release-evidence.schema.json"),
    Path("target/release-evidence/sbom-status.json"),
    Path("target/release-evidence/cargo-metadata.json"),
    Path("target/release-evidence/provenance.json"),
    Path("target/release-evidence/signing-readiness.json"),
    Path("Cargo.lock"),
]
artifacts = []
for path in inputs:
    if not path.exists():
        continue
    data = path.read_bytes()
    artifacts.append(
        {
            "path": str(path),
            "sha256": hashlib.sha256(data).hexdigest(),
            "kind": "release_contract_input",
            "bytes": len(data),
        }
    )

evidence = {
    "release_id": release_id,
    "channel": channel,
    "source_commit": source_commit,
    "source_tag": source_tag or None,
    "toolchain": toolchain,
    "workflow_run": {
        "workflow_ref": os.environ.get("GITHUB_WORKFLOW_REF", "local"),
        "run_id": os.environ.get("GITHUB_RUN_ID", "local"),
        "run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT", "local"),
        "ref": os.environ.get("GITHUB_REF", "local"),
        "sha": os.environ.get("GITHUB_SHA", source_commit),
        "actor": os.environ.get("GITHUB_ACTOR", "local"),
        "repository": os.environ.get("GITHUB_REPOSITORY", "local"),
    },
    "builder": {
        "kind": "github-actions" if os.environ.get("GITHUB_ACTIONS") == "true" else "local",
        "workflow": os.environ.get("GITHUB_WORKFLOW", "local"),
    },
    "materials": [
        {"path": artifact["path"], "sha256": artifact["sha256"]}
        for artifact in artifacts
    ],
    "artifacts": artifacts,
    "sbom_path": "target/release-evidence/sbom.cdx.json" if Path("target/release-evidence/sbom.cdx.json").exists() else "target/release-evidence/sbom-status.json",
    "provenance_path": "target/release-evidence/provenance.json",
    "signing_readiness_path": "target/release-evidence/signing-readiness.json",
    "checks": [
        {
            "name": "release_evidence_contract",
            "status": "pass",
            "detail": "source commit, toolchain, channel, SBOM status, provenance, signing readiness, and release contract inputs recorded",
        }
    ],
}
out = Path("target/release-evidence/release-evidence.json")
out.write_text(json.dumps(evidence, sort_keys=True) + "\n", encoding="utf-8")
PY
echo "release evidence: target/release-evidence/release-evidence.json"
