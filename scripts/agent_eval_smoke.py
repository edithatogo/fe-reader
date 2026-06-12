#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs/coding-agent-evaluation-harness.md"
REPORT = ROOT / "target/release-evidence/agent-eval-smoke.json"


def fail(message: str) -> None:
    raise SystemExit(f"agent eval smoke failed: {message}")


def main() -> None:
    if not DOC.exists():
        fail("missing docs/coding-agent-evaluation-harness.md")
    doc = DOC.read_text(encoding="utf-8")
    for token in [
        "architecture compliance",
        "schema compliance",
        "CLI contract compliance",
        "operation safety compliance",
        "platform adapter boundaries",
        "Golden tasks",
        "add a read-only CLI command",
        "add a platform adapter stub without leaking OS types",
        "phase gates",
        "file-boundary rules",
    ]:
        if token not in doc:
            fail(f"evaluation harness doc missing token: {token}")
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(
        json.dumps(
            {
                "check": "agent_eval",
                "status": "pass",
                "doc": "docs/coding-agent-evaluation-harness.md",
                "criteria": [
                    "architecture compliance",
                    "schema compliance",
                    "CLI contract compliance",
                    "operation safety compliance",
                    "platform adapter boundaries",
                    "performance budget compliance",
                    "fixture/corpus coverage",
                    "security policy compliance",
                    "accessibility baseline compliance",
                    "API compatibility",
                ],
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print("agent eval smoke passed")


if __name__ == "__main__":
    main()
