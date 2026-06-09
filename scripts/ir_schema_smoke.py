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
    print(f"ir schema smoke failure: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> None:
    result = subprocess.run(
        ["cargo", "run", "-q", "-p", "xtask", "--", "ir-smoke"],
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    payload = json.loads(result.stdout)
    if set(payload) != {"document_ir", "transformation_graph"}:
        fail("xtask ir-smoke emitted unexpected top-level keys")

    document_ir = payload["document_ir"]
    graph = payload["transformation_graph"]
    if document_ir["source_sha256"] != graph["input_document_sha256"]:
        fail("document IR and graph must use the same input hash")
    if graph["passes"][0]["policy_risk"] != "ReadOnly":
        fail("smoke graph must stay read-only")

    if jsonschema is not None:
        document_schema = json.loads((ROOT / "schemas/document-ir.schema.json").read_text())
        graph_schema = json.loads((ROOT / "schemas/transformation-pass.schema.json").read_text())
        jsonschema.validate(document_ir, document_schema)
        jsonschema.validate(graph, graph_schema)

    print("ir schema smoke passed")


if __name__ == "__main__":
    main()
