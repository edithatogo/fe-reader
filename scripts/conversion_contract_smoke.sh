#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; skipping conversion contract smoke in this environment"
  exit 0
fi

cargo test -q -p fe_reader_conversion

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

root = Path.cwd()
schema = json.loads((root / "schemas/conversion-job.schema.json").read_text(encoding="utf-8"))

jobs = [
    {
        "job_id": "job-pdf-markdown-smoke",
        "input_kind": "pdf",
        "output_kind": "markdown",
        "input_uri": "fixtures/minimal/minimal.pdf",
        "output_uri": "target/conversion/minimal.md",
        "preserve_metadata": False,
        "create_source_map": False,
        "provider_hint": "builtin_pdf_export_contract",
        "options": {},
    },
    {
        "job_id": "job-docx-pdf-smoke",
        "input_kind": "docx",
        "output_kind": "pdf",
        "input_uri": "examples/source-linked-projects/example.docx",
        "output_uri": "target/conversion/example.pdf",
        "preserve_metadata": True,
        "create_source_map": True,
        "provider_hint": "source_pipeline_contract",
        "options": {"provider_family": "libreoffice"},
    },
]

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

if jsonschema is not None:
    for job in jobs:
        jsonschema.validate(job, schema)

supported_pairs = {
    ("pdf", "markdown"),
    ("pdf", "html"),
    ("pdf", "text"),
    ("pdf", "json"),
    ("docx", "pdf"),
    ("markdown", "pdf"),
    ("markdown", "html"),
    ("quarto", "pdf"),
    ("typst", "pdf"),
    ("latex", "pdf"),
}
for job in jobs:
    pair = (job["input_kind"], job["output_kind"])
    if pair not in supported_pairs:
        raise SystemExit(f"conversion smoke unsupported pair: {pair}")

report_path = root / "target/conversion-reports/conversion-contract-smoke.json"
report_path.parent.mkdir(parents=True, exist_ok=True)
report_path.write_text(
    json.dumps(
        {
            "check": "conversion_contract_smoke",
            "status": "pass",
            "jobs": jobs,
            "policy": "plan-only; execution requires policy and user approval",
        },
        indent=2,
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
print(f"conversion contract smoke: {report_path.relative_to(root)}")
PY
