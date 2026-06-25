#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys

try:
    import jsonschema  # type: ignore
except Exception:
    jsonschema = None

ROOT = pathlib.Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "platform-parity-matrix.json"
SCHEMA = ROOT / "schemas" / "platform-parity-matrix.schema.json"
REPORT = ROOT / "target" / "platform-parity-matrix-check.json"

PLATFORMS = {
    "macos",
    "windows",
    "linux",
    "android",
    "ios",
    "web_local_pwa",
    "browser_extension",
}
ALLOWED_STATUS = {"supported", "plan-only", "documented-limitation", "external-gate"}
REQUIRED_CAPABILITIES = {
    "open-local-pdf",
    "navigate-zoom-search",
    "inspect-validate-accessibility",
    "workflow-planning-surfaces",
    "approved-mutation-apply",
    "buildable-release-artifact",
}
REQUIRED_EVIDENCE = [
    ROOT / "native" / "macos" / "FeReaderNativeApp.swift",
    ROOT / "contracts" / "web" / "postmessage-contract.md",
    ROOT / "contracts" / "platform" / "android-intents" / "AndroidManifest.contract.xml",
    ROOT / "contracts" / "platform" / "ios-appintents" / "FeReaderAppIntents.swift",
    ROOT / "contracts" / "platform" / "windows-com" / "FeReaderAutomation.idl",
    ROOT / "contracts" / "platform" / "linux-dbus" / "org.fereader.FeReader1.xml",
]


def rel(path: pathlib.Path) -> str:
    return str(path.relative_to(ROOT))


def main() -> int:
    failures: list[str] = []
    if not SCHEMA.exists():
        failures.append(f"missing {rel(SCHEMA)}")
    for path in REQUIRED_EVIDENCE:
        if not path.exists():
            failures.append(f"missing platform evidence path: {rel(path)}")

    if not MATRIX.exists():
        failures.append(f"missing {rel(MATRIX)}")
        data: dict = {}
    else:
        data = json.loads(MATRIX.read_text(encoding="utf-8"))
        if jsonschema is not None and SCHEMA.exists():
            schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
            try:
                jsonschema.validate(instance=data, schema=schema)
            except jsonschema.ValidationError as exc:
                failures.append(f"matrix does not match schema: {exc.message}")

    if data.get("feature_gate") != "cross_platform_full_workflow_parity":
        failures.append("matrix feature_gate must be cross_platform_full_workflow_parity")
    if data.get("launch_blocking") is not False:
        failures.append("platform parity matrix must not block launch by itself")

    governance = data.get("governance", {})
    if governance.get("mutation_pipeline") != "OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt":
        failures.append("governance must cite the canonical mutation pipeline")
    if "fe_reader_core remains pure" not in governance.get("adapter_boundary", ""):
        failures.append("governance must keep fe_reader_core pure")
    if "external gates" not in governance.get("release_policy", ""):
        failures.append("release policy must keep signing and registries as external gates")
    if "supported evidence" not in governance.get("claim_policy", ""):
        failures.append("claim policy must bind parity claims to evidence")

    platforms = set(data.get("platforms", []))
    if platforms != PLATFORMS:
        failures.append(f"platform set mismatch: expected {sorted(PLATFORMS)}, got {sorted(platforms)}")

    capabilities = data.get("capabilities", [])
    if not isinstance(capabilities, list) or not capabilities:
        failures.append("capabilities must be a non-empty list")
        capabilities = []

    seen_ids: set[str] = set()
    mutating_count = 0
    external_gate_count = 0
    for capability in capabilities:
        if not isinstance(capability, dict):
            failures.append("capability entries must be objects")
            continue
        capability_id = capability.get("id")
        if not isinstance(capability_id, str) or not capability_id:
            failures.append("capability missing id")
            continue
        if capability_id in seen_ids:
            failures.append(f"duplicate capability id: {capability_id}")
        seen_ids.add(capability_id)
        if not capability.get("summary"):
            failures.append(f"{capability_id} missing summary")
        mutating = capability.get("mutating") is True
        if mutating:
            mutating_count += 1
            if capability.get("mutation_pipeline_required") is not True:
                failures.append(f"{capability_id} must require mutation_pipeline_required")

        support_by_platform = capability.get("support_by_platform", {})
        if set(support_by_platform.keys()) != PLATFORMS:
            failures.append(f"{capability_id} support_by_platform must cover every target platform")
            continue
        for platform, support in support_by_platform.items():
            if not isinstance(support, dict):
                failures.append(f"{capability_id}/{platform} support entry must be an object")
                continue
            status = support.get("status")
            if status not in ALLOWED_STATUS:
                failures.append(f"{capability_id}/{platform} invalid status: {status}")
            evidence = support.get("evidence", [])
            if not isinstance(evidence, list):
                failures.append(f"{capability_id}/{platform} evidence must be a list")
                evidence = []
            if status == "supported" and not evidence:
                failures.append(f"{capability_id}/{platform} supported status requires evidence")
            if status in {"plan-only", "documented-limitation"} and not support.get("limitation"):
                failures.append(f"{capability_id}/{platform} {status} status requires limitation text")
            if status == "external-gate":
                external_gate_count += 1
                if not support.get("external_gate"):
                    failures.append(f"{capability_id}/{platform} external-gate status requires external_gate text")
            for item in evidence:
                if not isinstance(item, str):
                    failures.append(f"{capability_id}/{platform} evidence item must be a string")
                    continue
                if not (ROOT / item).exists():
                    failures.append(f"{capability_id}/{platform} evidence path missing: {item}")

    missing_capabilities = REQUIRED_CAPABILITIES - seen_ids
    if missing_capabilities:
        failures.append(f"matrix missing required capabilities: {sorted(missing_capabilities)}")
    if mutating_count == 0:
        failures.append("matrix must include at least one mutating capability")
    if external_gate_count == 0:
        failures.append("matrix must include explicit external release gates")

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(
        json.dumps(
            {
                "check": "platform_parity_matrix",
                "status": "fail" if failures else "pass",
                "platforms": sorted(platforms),
                "capability_count": len(seen_ids),
                "mutating_capability_count": mutating_count,
                "external_gate_count": external_gate_count,
                "failures": failures,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )

    if failures:
        print("PLATFORM PARITY MATRIX CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1
    print("platform parity matrix: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
