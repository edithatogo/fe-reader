#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
import tempfile
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

    source_expectations = [
        ("mcp", "apply", True, True),
        ("automation", "apply", True, True),
        ("com", "apply", True, True),
        ("applescript", "apply", True, True),
        ("dbus", "apply", True, True),
        ("android-intent", "apply", True, True),
        ("ios-app-intent", "apply", True, True),
        ("web", "apply", True, True),
        ("browser-extension", "apply", True, True),
        ("local-api", "apply", True, True),
        ("plugin", "apply", True, True),
        ("mcp", "automation", True, True),
        ("com", "automation", True, True),
        ("applescript", "automation", True, True),
        ("dbus", "automation", True, True),
        ("android-intent", "automation", True, True),
        ("ios-app-intent", "automation", True, True),
        ("browser-extension", "automation", True, True),
        ("local-api", "automation", True, True),
        ("web", "network", False, True),
        ("browser-extension", "network", False, True),
        ("local-api", "network", False, True),
        ("plugin", "plugin", False, True),
        ("automation", "external-tool", False, True),
        ("com", "external-tool", False, True),
        ("applescript", "external-tool", False, True),
        ("dbus", "external-tool", False, True),
        ("android-intent", "external-tool", False, True),
        ("ios-app-intent", "external-tool", False, True),
    ]
    for source, action, allowed, requires_review in source_expectations:
        decision = run_json(
            "cargo",
            "run",
            "-q",
            "-p",
            "fe_reader_cli",
            "--",
            "policy",
            action,
            "--source",
            source,
        )
        expect(
            decision["allowed"] is allowed,
            f"policy {source}/{action} allowed expected {allowed}, got {decision['allowed']}",
        )
        expect(
            decision["requires_review"] is requires_review,
            (
                f"policy {source}/{action} requires_review expected {requires_review}, "
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

    unknown_source = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "fe_reader_cli",
            "--",
            "policy",
            "read",
            "--source",
            "unknown-source",
        ],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    expect(unknown_source.returncode != 0, "unknown policy sources must fail closed")


def check_additive_cli_contracts() -> None:
    metadata = run_json(
        "cargo",
        "run",
        "-q",
        "-p",
        "fe_reader_cli",
        "--",
        "metadata",
        "fixtures/minimal/minimal.pdf",
        "--json",
    )
    expect(isinstance(metadata, dict), "metadata output must be a JSON object")
    expect(metadata["intent"]["risk_level"] == "read_only", "metadata must be read-only")
    expect(metadata["plan"]["write_mode"] == "no_write", "metadata must not write")
    expect(
        metadata["metadata"]["parser_error"] is None,
        "minimal metadata inspection must parse without error",
    )
    expect(
        metadata["metadata"]["xmp_streams"] == [],
        "minimal metadata fixture must report an empty XMP stream list",
    )

    search = run_json(
        "cargo",
        "run",
        "-q",
        "-p",
        "fe_reader_cli",
        "--",
        "search",
        "fixtures/corpus/basic/text-search-fixture.pdf",
        "Reader",
        "--json",
    )
    expect(isinstance(search, dict), "search output must be a JSON object")
    expect(search["intent"]["kind"] == "search", "search intent kind must be search")
    expect(search["intent"]["risk_level"] == "read_only", "search must be read-only")
    expect(search["plan"]["write_mode"] == "no_write", "search must not write")
    expect(
        search["text"]["extraction"]["precise_geometry"] is False,
        "search extraction must disclose fallback geometry",
    )
    expect(len(search["index_records"]) == 1, "text fixture search must emit one index record")
    expect(
        search["index_records"][0]["bbox"] == [0.0, 0.0, 612.0, 792.0],
        "text fixture index bbox must use non-empty page fallback geometry",
    )
    expect(len(search["hits"]) == 1, "text fixture search must find one hit")
    expect(
        search["hits"][0]["text"] == "Fe Reader Search Fixture\n",
        "text fixture search must preserve extracted text",
    )
    expect(
        search["hits"][0]["bbox"]["width"] == 612.0
        and search["hits"][0]["bbox"]["height"] == 792.0,
        "text fixture hit bbox must use non-empty page fallback geometry",
    )
    expect(search["hits"][0]["char_offset"] == 3, "text fixture hit offset must be stable")

    with tempfile.TemporaryDirectory() as tmpdir:
        journal_path = Path(tmpdir) / "journal.json"
        journal = run_json(
            "cargo",
            "run",
            "-q",
            "-p",
            "fe_reader_cli",
            "--",
            "journal",
            "plan",
            "fixtures/minimal/minimal.pdf",
            "--out",
            str(journal_path),
            "--json",
        )
        expect(journal_path.is_file(), "journal plan must persist the sidecar")
        expect(
            journal["intent"]["risk_level"] == "document_mutation",
            "journal plan must be review-gated mutation planning",
        )
        expect(
            journal["plan"]["approved_for_apply"] is False,
            "journal plan must not approve apply",
        )
        entries = journal["journal"]["entries"]
        expect(
            [entry["phase"] for entry in entries] == ["intent_received", "plan_generated"],
            "journal plan must record intent and plan phases",
        )
        persisted = json.loads(journal_path.read_text(encoding="utf-8"))
        expect(persisted == journal["journal"], "persisted sidecar must match CLI JSON")
        inspected = run_json(
            "cargo",
            "run",
            "-q",
            "-p",
            "fe_reader_cli",
            "--",
            "journal",
            "inspect",
            str(journal_path),
            "--json",
        )
        expect(
            inspected["journal"] == journal["journal"],
            "journal inspect must read the persisted sidecar",
        )
        expect(
            inspected["recovery_required"] is False,
            "planned journal sidecar must not require recovery",
        )
        expect(
            inspected["latest"]["phase"] == "plan_generated",
            "journal inspect must report the latest phase",
        )
        recoveries = run_json(
            "cargo",
            "run",
            "-q",
            "-p",
            "fe_reader_cli",
            "--",
            "journal",
            "recoveries",
            tmpdir,
            "--json",
        )
        expect(
            recoveries["recovery_required_count"] == 0,
            "planned journal directory must not report recovery work",
        )


def check_ir_schema_smoke() -> None:
    result = subprocess.run(
        ["python3", "scripts/ir_schema_smoke.py"],
        cwd=ROOT,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    expect(result.returncode == 0, f"IR schema smoke failed: {result.stderr}")


def check_contract_smokes() -> None:
    for script in (
        "scripts/job_contract_smoke.py",
        "scripts/quality_dashboard_smoke.py",
        "scripts/active_content_firewall_smoke.py",
        "scripts/visual_regression_compare.py",
    ):
        args = ["python3", script]
        if script.endswith("visual_regression_compare.py"):
            args.append("--smoke")
        result = subprocess.run(
            args,
            cwd=ROOT,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        expect(result.returncode == 0, f"{script} failed: {result.stderr}")


def main() -> None:
    check_cli_inspect_contract()
    check_policy_matrix()
    check_additive_cli_contracts()
    check_ir_schema_smoke()
    check_contract_smokes()
    print("wave0 acceptance check passed")


if __name__ == "__main__":
    main()
