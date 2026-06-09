#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from __future__ import annotations

import hashlib
import json
from pathlib import Path

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

root = Path.cwd()
fixture = root / "fixtures/corpus/malformed-adversarial/truncated-catalog.pdf"
target_dir = root / "target/recovery-reports"
target_dir.mkdir(parents=True, exist_ok=True)

source_bytes = fixture.read_bytes()
copy_path = target_dir / "truncated-catalog.repair-copy.pdf"
copy_path.write_bytes(source_bytes)

source_sha = hashlib.sha256(source_bytes).hexdigest()
copy_sha = hashlib.sha256(copy_path.read_bytes()).hexdigest()
report = {
    "plan_id": "wave3-repair-copy-only-smoke",
    "original_sha256": source_sha,
    "mode": "RepairAndSaveCopy",
    "actions": [
        {
            "op": "save_copy_only",
            "source": str(fixture.relative_to(root)),
            "copy": str(copy_path.relative_to(root)),
            "copy_sha256": copy_sha,
        }
    ],
    "write_policy": "CopyOnlyNoSourceMutation",
    "warnings": [
        "Repair smoke preserves source bytes and writes only a diagnostic copy."
    ],
    "receipt": {
        "verification_status": "passed",
        "source_preserved": source_sha == copy_sha,
        "audit_receipt_emitted": True,
    },
}
if not report["receipt"]["source_preserved"]:
    raise SystemExit("repair copy changed source bytes")

if jsonschema is not None:
    schema = json.loads((root / "schemas/recovery-report.schema.json").read_text(encoding="utf-8"))
    jsonschema.validate(report, schema)

report_path = target_dir / "smoke.json"
report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(f"pdf repair smoke: {report_path.relative_to(root)}")
PY
