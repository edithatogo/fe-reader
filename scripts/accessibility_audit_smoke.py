#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
evidence_dir = root / "target" / "accessibility-reports"
report_path = evidence_dir / "smoke.json"
required = [
    root / "schemas" / "accessibility-audit.schema.json",
    root / "contracts" / "rust" / "accessibility.rs",
    root / "contracts" / "rust" / "accessibility_audit.rs",
    root / "docs" / "ux-accessibility-human-factors.md",
    root / "crates" / "fe_reader_accessibility" / "src" / "lib.rs",
]
missing = [str(p) for p in required if not p.exists()]
if missing:
    print("accessibility smoke missing:", missing)
    raise SystemExit(1)

report = {
    "check": "accessibility_audit_smoke",
    "status": "pass",
    "required": [str(p.relative_to(root)) for p in required],
}
evidence_dir.mkdir(parents=True, exist_ok=True)
report_path.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
print("accessibility smoke: ok")
