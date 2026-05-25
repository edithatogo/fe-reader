#!/usr/bin/env bash
set -euo pipefail
mkdir -p target/frontier-reports
python3 - <<'PY'
import json
import shutil
from pathlib import Path

tools = {
    "llvm-profdata": shutil.which("llvm-profdata") is not None,
    "llvm-bolt": shutil.which("llvm-bolt") is not None,
    "perf": shutil.which("perf") is not None,
    "hyperfine": shutil.which("hyperfine") is not None,
    "sccache": shutil.which("sccache") is not None,
}
Path("target/frontier-reports/toolchain-experiment-smoke.json").write_text(
    json.dumps(
        {
            "check": "toolchain_experiment_smoke",
            "status": "advisory",
            "detail": "PGO/BOLT/build-speed tooling discovery only; not a hard gate before accepted baselines",
            "tools": tools,
        },
        sort_keys=True,
    )
    + "\n",
    encoding="utf-8",
)
PY
echo "toolchain experiment smoke: target/frontier-reports/toolchain-experiment-smoke.json"
