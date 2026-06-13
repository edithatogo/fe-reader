#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path
import subprocess

root = Path.cwd()
target_dir = root / "target/recovery-reports"
target_dir.mkdir(parents=True, exist_ok=True)

source = root / "fixtures/corpus/basic/text-search-fixture.pdf"
redacted_pdf = target_dir / "redaction-scan-smoke.pdf"
redacted_pdf.write_bytes(source.read_bytes() + b"\n% hidden SECRET marker in redacted output\n")

receipt_path = target_dir / "redaction-scan-smoke.receipt.json"
receipt_path.write_text('{"plan_id":"wave3-redaction-scan-smoke"}\n', encoding="utf-8")

proc = subprocess.run(
    [
        "cargo",
        "run",
        "-q",
        "-p",
        "fe_reader_cli",
        "--",
        "lab",
        "redaction-scan",
        str(redacted_pdf),
        "--receipt",
        str(receipt_path),
        "--json",
    ],
    check=True,
    capture_output=True,
    text=True,
)
payload = json.loads(proc.stdout)
scan = payload["scan"]
if "SECRET" not in scan["residual_markers"]:
    raise SystemExit("redaction scan smoke missing residual marker")
if not any(f["code"] == "redaction_residual_marker_detected" for f in scan["findings"]):
    raise SystemExit("redaction scan smoke missing residual finding")

report_path = target_dir / "redaction-scan-smoke.json"
report_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(f"pdf lab redaction scan smoke: {report_path.relative_to(root)}")
PY
