#!/usr/bin/env python3
"""Validate the configuration and policy engine contracts."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "docs/config-policy-engine.md"
FEATURE_DOC = ROOT / "docs/feature-flag-runtime-capabilities.md"
CONTRACT = ROOT / "contracts/rust/config_policy_engine.rs"
CONFIG_SCHEMA = ROOT / "schemas/config-policy.schema.json"
FLAG_SCHEMA = ROOT / "schemas/feature-flag.schema.json"
REPORT = ROOT / "target/release-evidence/policy-engine-smoke.json"


def fail(message: str) -> None:
    raise SystemExit(f"policy engine smoke failed: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def main() -> int:
    for path in [DOC, FEATURE_DOC, CONTRACT, CONFIG_SCHEMA, FLAG_SCHEMA]:
        require(path.exists(), f"missing required file: {path.relative_to(ROOT)}")

    doc = DOC.read_text(encoding="utf-8")
    for token in [
        "Settings, feature flags, enterprise policy, automation restrictions, plugin permissions and active-content rules",
        "Enterprise policy must override user settings for security-sensitive controls.",
        "disable MCP mutation tools",
        "disable plugins",
        "require secure redaction verification",
    ]:
        require(token in doc, f"config policy doc missing token: {token}")

    feature_doc = FEATURE_DOC.read_text(encoding="utf-8")
    for token in [
        "runtime capability discovery",
        "A disabled feature should produce a structured explanation",
        "contracts/rust/feature_flags.rs",
        "schemas/feature-flag.schema.json",
    ]:
        require(token in feature_doc, f"feature flag doc missing token: {token}")

    contract = CONTRACT.read_text(encoding="utf-8")
    for token in [
        "PolicyEvaluationRequest",
        "PolicyEvaluationResult",
        "PolicySubject",
        "ConfigPolicyEngine",
        "RequireStrongerAuth",
    ]:
        require(token in contract, f"config policy contract missing token: {token}")

    config_schema = json.loads(CONFIG_SCHEMA.read_text(encoding="utf-8"))
    flag_schema = json.loads(FLAG_SCHEMA.read_text(encoding="utf-8"))
    require(config_schema.get("title") == "Fe Reader Config Policy", "wrong config policy schema title")
    require(flag_schema.get("title") == "Fe Reader Feature Flag", "wrong feature flag schema title")
    require(config_schema.get("required") == ["policy_id", "version", "rules"], "config policy schema required fields drifted")
    require(
        flag_schema.get("properties", {}).get("risk", {}).get("enum") == ["low", "medium", "high", "critical"],
        "feature flag schema risk enum drifted",
    )

    subprocess.run(["cargo", "test", "-q", "-p", "fe_reader_config"], cwd=ROOT, check=True)

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(
        json.dumps(
            {
                "check": "policy_engine",
                "status": "pass",
                "crate": "fe_reader_config",
                "schemas": [
                    "schemas/config-policy.schema.json",
                    "schemas/feature-flag.schema.json",
                ],
                "contract": "contracts/rust/config_policy_engine.rs",
                "runtime_capability_boundary": "structured explanations for disabled features",
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"policy engine smoke: {REPORT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
