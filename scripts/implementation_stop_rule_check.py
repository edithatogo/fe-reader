#!/usr/bin/env python3
"""Reject ungrounded scope expansion in agent-created docs.

This lightweight check is intentionally heuristic. It reminds agents to connect new scope to
contracts, tests, CLI commands or acceptance criteria.
"""
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[1]
stop_rule = root / "docs" / "implementation-stop-rule.md"
if not stop_rule.exists():
    print("missing docs/implementation-stop-rule.md", file=sys.stderr)
    sys.exit(1)

required = ["contract", "test", "schema", "acceptance", "Wave 0"]
text = stop_rule.read_text(encoding="utf-8")
missing = [word for word in required if word not in text]
if missing:
    print(f"implementation stop rule missing anchors: {missing}", file=sys.stderr)
    sys.exit(1)
print("implementation stop rule check passed")
