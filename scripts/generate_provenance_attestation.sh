#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/release-evidence
SOURCE_COMMIT="$(git rev-parse --verify HEAD)"
RELEASE_ID="${FE_RELEASE_ID:-dev-smoke}"
CHANNEL="${FE_RELEASE_CHANNEL:-dev}"
python3 - "$SOURCE_COMMIT" "$RELEASE_ID" "$CHANNEL" <<'PY'
import hashlib
import json
import os
import sys
from pathlib import Path

source_commit, release_id, channel = sys.argv[1:4]
materials = []
for rel in ["Cargo.lock", "packaging/package-matrix.yaml", "packaging/release-channels.yaml"]:
    path = Path(rel)
    data = path.read_bytes()
    materials.append({"uri": rel, "digest": {"sha256": hashlib.sha256(data).hexdigest()}})

predicate = {
    "_type": "https://in-toto.io/Statement/v1",
    "subject": [{"name": release_id, "digest": {"gitCommit": source_commit}}],
    "predicateType": "https://slsa.dev/provenance/v1",
    "predicate": {
        "buildDefinition": {
            "buildType": "https://github.com/fereader/fe-reader/bootstrap-release",
            "externalParameters": {"channel": channel},
            "internalParameters": {"workflow": os.environ.get("GITHUB_WORKFLOW", "local")},
            "resolvedDependencies": materials,
        },
        "runDetails": {
            "builder": {"id": os.environ.get("GITHUB_SERVER_URL", "local")},
            "metadata": {"invocationId": os.environ.get("GITHUB_RUN_ID", "local")},
        },
    },
}
Path("target/release-evidence/provenance.json").write_text(
    json.dumps(predicate, sort_keys=True) + "\n", encoding="utf-8"
)
PY
echo "release provenance: target/release-evidence/provenance.json"
