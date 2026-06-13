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
timeline_pdf = target_dir / "timeline-smoke.pdf"
timeline_pdf.write_bytes(source.read_bytes() + b"\n% incremental update smoke\nstartxref\n12345\n/Prev 6789\n%%EOF\n")

proc = subprocess.run(
    ["cargo", "run", "-q", "-p", "fe_reader_cli", "--", "lab", "timeline", str(timeline_pdf), "--json"],
    check=True,
    capture_output=True,
    text=True,
)
payload = json.loads(proc.stdout)
timeline = payload["timeline"]
if not timeline["revisions"]:
    raise SystemExit("timeline smoke missing revisions")
if not any(f["code"] == "incremental_revision_timeline_smoke" for f in timeline["findings"]):
    raise SystemExit("timeline smoke missing incremental revision finding")

report_path = target_dir / "timeline-smoke.json"
report_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(f"pdf lab timeline smoke: {report_path.relative_to(root)}")
PY
