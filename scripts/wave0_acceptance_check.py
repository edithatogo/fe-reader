#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

try:
    import jsonschema  # type: ignore
except Exception:  # pragma: no cover - optional local dependency
    jsonschema = None

ROOT = Path(__file__).resolve().parents[1]


def run_json(*args: str) -> object:
    result = subprocess.run(
        args,
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    return json.loads(result.stdout)


def fail(message: str) -> None:
    print(f"wave0 acceptance failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def expect(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def validate_summary_schema(summary: object) -> None:
    schema_path = ROOT / "schemas/pdf-document-summary.schema.json"
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    if jsonschema is not None:
        jsonschema.validate(summary, schema)


def check_cli_inspect_contract() -> None:
    payload = run_json(
        "cargo",
        "run",
        "-q",
        "-p",
        "fe_reader_cli",
        "--",
        "inspect",
        "fixtures/minimal/minimal.pdf",
        "--json",
    )
    expect(isinstance(payload, dict), "inspect output must be a JSON object")
    for key in ("intent", "plan", "summary"):
        expect(key in payload, f"inspect output missing {key}")

    plan = payload["plan"]
    expect(plan["write_mode"] == "no_write", "inspect plan must be no_write")
    expect(plan["approved_for_apply"] is False, "inspect plan must not be approved for apply")
    expect(plan["operations"] == [{"op": "noop"}], "inspect plan must be a no-op plan")

    summary = payload["summary"]
    validate_summary_schema(summary)
    parser = summary["parser"]
    expect(parser["adapter"] == "lopdf", "inspect parser adapter must be lopdf")
    expect(parser["page_count"] == 1, "minimal fixture must report one page")
    expect(parser["encrypted"] is False, "minimal fixture must not be encrypted")
    expect(parser["error"] is None, "minimal fixture must parse without error")


def check_policy_matrix() -> None:
    expectations = {
        "read": (True, False),
        "plan": (True, True),
        "apply": (True, True),
        "export": (True, True),
        "automation": (True, True),
        "external-tool": (False, True),
        "plugin": (False, True),
        "network": (False, True),
    }
    for action, (allowed, requires_review) in expectations.items():
        decision = run_json("cargo", "run", "-q", "-p", "fe_reader_cli", "--", "policy", action)
        expect(
            decision["allowed"] is allowed,
            f"policy {action} allowed expected {allowed}, got {decision['allowed']}",
        )
        expect(
            decision["requires_review"] is requires_review,
            (
                f"policy {action} requires_review expected {requires_review}, "
                f"got {decision['requires_review']}"
            ),
        )

    unknown = subprocess.run(
        ["cargo", "run", "-q", "-p", "fe_reader_cli", "--", "policy", "unknown-action"],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    expect(unknown.returncode != 0, "unknown policy actions must fail closed")


def main() -> None:
    check_cli_inspect_contract()
    check_policy_matrix()
    print("wave0 acceptance check passed")


if __name__ == "__main__":
    main()
