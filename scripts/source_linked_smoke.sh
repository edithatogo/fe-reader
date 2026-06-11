#!/usr/bin/env bash
set -euo pipefail

python3 - <<'PY'
from __future__ import annotations

import json
from pathlib import Path

root = Path.cwd()
schema_path = root / "schemas/source-linked-project.schema.json"
contract_path = root / "contracts/rust/source_linked_document.rs"
docs_path = root / "docs/source-linked-authoring-workflows.md"
template_path = root / "templates/source-linked/typst-project.feproj.json"
capability_path = root / "examples/source-linked-projects/typst-provider-capability.json"

for path in (schema_path, contract_path, docs_path, template_path, capability_path):
    if not path.exists():
        raise SystemExit(f"source-linked smoke missing: {path.relative_to(root)}")

schema = json.loads(schema_path.read_text(encoding="utf-8"))
template = json.loads(template_path.read_text(encoding="utf-8"))
capability = json.loads(capability_path.read_text(encoding="utf-8"))

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

if jsonschema is not None:
    jsonschema.validate(template, schema)

if template["provider"] != "typst":
    raise SystemExit("source-linked smoke expected typst template provider")
if template["permissions"].get("allow_external_build") is not False:
    raise SystemExit("source-linked template must disable external build by default")
if template["permissions"].get("allow_network") is not False:
    raise SystemExit("source-linked template must disable network by default")

discovery = capability.get("provider_capability_discovery", {})
if discovery.get("required_before_execution") is not True:
    raise SystemExit("provider capability discovery must be required before execution")
if discovery.get("external_tool_execution_default") != "disabled":
    raise SystemExit("external tool execution must default to disabled")
policy = capability.get("policy", {})
if policy.get("requires_explicit_user_action") is not True:
    raise SystemExit("source-linked provider policy must require explicit user action")
if policy.get("requires_audit_receipt") is not True:
    raise SystemExit("source-linked provider policy must require audit receipt")

contract = contract_path.read_text(encoding="utf-8")
for token in ("Typst", "Quarto", "Pandoc", "Tectonic", "CustomExternal"):
    if token not in contract:
        raise SystemExit(f"source-linked contract missing provider token: {token}")

print("source-linked smoke: ok")
PY
