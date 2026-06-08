#!/usr/bin/env python3
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "target" / "release-evidence"
EVIDENCE.mkdir(parents=True, exist_ok=True)


def read(rel):
    return (ROOT / rel).read_text(encoding="utf-8")


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def require(condition, message, failures):
    if not condition:
        failures.append(message)


def required_file(rel, failures):
    path = ROOT / rel
    require(path.exists(), f"missing {rel}", failures)
    return path


def check_release_evidence(failures):
    evidence = EVIDENCE / "release-evidence.json"
    readiness = EVIDENCE / "release-readiness.json"
    provenance = EVIDENCE / "provenance.json"
    signing = EVIDENCE / "signing-readiness.json"
    for path in [evidence, readiness, provenance, signing]:
        require(path.exists(), f"missing release evidence output {path}", failures)
        if path.exists():
            load_json(path)
    if evidence.exists():
        data = load_json(evidence)
        for field in ["release_id", "channel", "source_commit", "toolchain", "artifacts", "sbom_path", "provenance_path"]:
            require(field in data, f"release evidence missing {field}", failures)


def check_docs_and_templates(failures):
    for rel in [
        "fixtures/corpus/manifest.json",
        "benchmarks/budgets/performance-budgets.yaml",
        "templates/accessibility/checklists/app-accessibility-smoke.md",
        "templates/policy/default-enterprise-policy.json",
        "schemas/api-compat-report.schema.json",
        "schemas/accessibility-audit.schema.json",
        "schemas/cache-manifest.schema.json",
        "schemas/review-packet.schema.json",
        "schemas/pdf-optimization-plan.schema.json",
    ]:
        required_file(rel, failures)

    doc_tokens = {
        "docs/release-operations-updates.md": [
            "signed update manifest",
            "SBOM",
            "provenance/attestation artifact",
            "compatibility report",
            "accessibility report",
        ],
        "docs/enterprise-deployment-policy.md": ["Windows", "macOS", "Linux", "disable MCP server"],
        "docs/privacy-diagnostics-observability.md": ["local by default", "explicit user action", "privacy"],
        "docs/api-compatibility-governance.md": ["contracts/snapshots/", "MCP", "plugin"],
        "docs/differential-testing-oracles.md": ["Wave 7", "accepted corpus coverage", "documented limitation"],
        "docs/pdf-optimization-linearization-compression.md": ["receipt", "signature", "safe"],
        "docs/content-addressed-cache-workspace.md": ["privacy-sensitive", "not authoritative"],
    }
    for rel, tokens in doc_tokens.items():
        text = read(rel)
        for token in tokens:
            require(token in text, f"{rel} missing release-hardening token {token}", failures)


def check_snapshots_and_reports(failures):
    for rel in [
        "contracts/snapshots/rust-public-api/fe_reader_core.operation_intent.preview.json",
        "contracts/snapshots/rust-public-api/fe_reader_core.patch_plan.preview.json",
        "contracts/snapshots/rust-public-api/fe_reader_core.audit_receipt.preview.json",
        "contracts/snapshots/cli/fe_reader_cli.commands.preview.json",
        "contracts/snapshots/mcp/fe_reader_mcp.tools.preview.json",
        "contracts/snapshots/plugin-abi/fe_reader_plugin_host.preview.json",
    ]:
        load_json(required_file(rel, failures))

    oracle = ROOT / "target" / "oracle-reports" / "wave1-render-smoke.json"
    require(oracle.exists(), "missing differential oracle report output", failures)
    if oracle.exists():
        data = load_json(oracle)
        require(data.get("fixture_id") == "basic-text-search-fixture", "oracle report must cover accepted fixture", failures)

    for rel in [
        "target/frontier-reports/gpu-frontier-smoke.json",
        "target/frontier-reports/toolchain-experiment-smoke.json",
    ]:
        path = ROOT / rel
        require(path.exists(), f"missing frontier report {rel}", failures)
        if path.exists():
            load_json(path)


def main():
    failures = []
    try:
        check_release_evidence(failures)
        check_docs_and_templates(failures)
        check_snapshots_and_reports(failures)
    except Exception as exc:
        failures.append(str(exc))

    report = {
        "check": "wave7_release_hardening",
        "status": "fail" if failures else "pass",
        "criteria": [
            "release evidence bundle",
            "compatibility corpus evidence",
            "performance budget registry",
            "accessibility report contract",
            "enterprise policy template",
            "privacy diagnostics policy",
            "API compatibility snapshots",
            "differential oracle report",
            "optimization and cache privacy controls",
        ],
        "failures": failures,
    }
    (EVIDENCE / "wave7-release-hardening.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"wave7 release hardening failure: {failure}", file=sys.stderr)
        raise SystemExit(1)
    print("wave7 release hardening smoke: ok")


if __name__ == "__main__":
    main()
