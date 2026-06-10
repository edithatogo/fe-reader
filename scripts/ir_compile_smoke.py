#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None


def fail(message: str) -> None:
    print(f"ir compile smoke failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    result = subprocess.run(
        ["cargo", "run", "-q", "-p", "xtask", "--", "ir-compile-smoke"],
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    payload = json.loads(result.stdout)
    if set(payload) != {"transformation_graph", "pass_registry", "compilation_report"}:
        fail("xtask ir-compile-smoke emitted unexpected top-level keys")

    graph = payload["transformation_graph"]
    registry = payload["pass_registry"]
    report = payload["compilation_report"]

    if report["graph_id"] != graph["graph_id"]:
        fail("compile report graph_id must match graph")
    if report["input_document_sha256"] != graph["input_document_sha256"]:
        fail("compile report input hash must match graph")
    if report["mutation_policy"] != "passive_compile_only_no_execution_or_patch_plan":
        fail("compile report must stay passive and compile-only")
    if report["expected_write_mode"] != graph["expected_write_mode"]:
        fail("compile report write mode must mirror graph declaration")

    accepted = report["accepted_passes"]
    if len(accepted) != len(graph["passes"]):
        fail("compile report must bind every graph pass")
    if not accepted:
        fail("compile report must include at least one accepted pass")

    first_pass = graph["passes"][0]
    first_accepted = accepted[0]
    definitions = registry["definitions"]
    if first_pass["pass_type"] not in definitions:
        fail("registry must define the graph pass type")
    if first_accepted["policy_risk"] != "ReadOnly":
        fail("AA1 smoke pass must remain read-only")
    if first_accepted["pass_type"] != first_pass["pass_type"]:
        fail("accepted pass type must match graph pass type")
    if first_accepted["inputs"] != first_pass["inputs"]:
        fail("accepted pass inputs must match graph pass inputs")
    if first_accepted["outputs"] != first_pass["outputs"]:
        fail("accepted pass outputs must match graph pass outputs")

    if jsonschema is not None:
        schema = json.loads(
            (ROOT / "schemas/transformation-compile-report.schema.json").read_text()
        )
        jsonschema.validate(report, schema)

    print("ir compile smoke passed")


if __name__ == "__main__":
    main()
