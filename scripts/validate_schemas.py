#!/usr/bin/env python3
import json
import pathlib
import sys

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

root = pathlib.Path(__file__).resolve().parents[1]
failures = []
for path in sorted((root / "schemas").glob("*.schema.json")):
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
        if jsonschema is not None:
            jsonschema.validators.validator_for(data).check_schema(data)
    except Exception as exc:
        failures.append((path, exc))

if failures:
    for path, exc in failures:
        print(f"schema failure: {path}: {exc}", file=sys.stderr)
    raise SystemExit(1)
print("schema validation: ok")
