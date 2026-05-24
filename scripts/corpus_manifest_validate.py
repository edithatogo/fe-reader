#!/usr/bin/env python3
from __future__ import annotations
import json, pathlib, sys
root = pathlib.Path.cwd()
manifest = root / "fixtures" / "corpus" / "manifest.json"
if not manifest.exists():
    print("corpus manifest missing; creating directories is required before Wave 0 exits", file=sys.stderr)
    sys.exit(0)  # advisory until fixtures are materialised
try:
    data = json.loads(manifest.read_text())
except Exception as e:
    print(f"invalid corpus manifest: {e}", file=sys.stderr)
    sys.exit(1)
for item in data.get("fixtures", []):
    if item.get("redistribution") == "forbidden":
        print(f"forbidden fixture must not be committed: {item.get('fixture_id')}", file=sys.stderr)
        sys.exit(1)
print("corpus manifest check passed")
