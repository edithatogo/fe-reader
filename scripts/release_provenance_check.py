#!/usr/bin/env python3
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
failures: list[str] = []
PUBLIC_CHANNELS = {"preview", "beta", "stable", "lts", "store_submission"}


def read(path: str) -> str:
    full = ROOT / path
    if not full.exists():
        failures.append(f"missing release provenance file: {path}")
        return ""
    return full.read_text(encoding="utf-8", errors="replace")


workflow = read(".github/workflows/07-release.yml")
if workflow:
    for command in [
        "bash scripts/sbom_audit.sh",
        "bash scripts/generate_provenance_attestation.sh",
        "bash scripts/signing_readiness_check.sh",
        "python3 scripts/desktop_packaging_signing_check.py",
        "python3 scripts/stable_release_evidence_check.py",
        "python3 scripts/desktop_distribution_publication_check.py",
        "python3 scripts/enterprise_operations_readiness_check.py",
        "bash scripts/release_evidence_check.sh",
        "python3 scripts/release_provenance_check.py",
        "python3 scripts/release_matrix_check.py",
        "bash scripts/release_readiness_check.sh",
    ]:
        if command not in workflow:
            failures.append(f"release workflow missing provenance command: {command}")
    for token in ["target/release-evidence/**", "if-no-files-found: error", "retention-days:"]:
        if token not in workflow:
            failures.append(f"release workflow missing evidence upload token: {token}")

for script in [
    "scripts/sbom_audit.sh",
    "scripts/generate_provenance_attestation.sh",
    "scripts/signing_readiness_check.sh",
    "scripts/desktop_packaging_signing_check.py",
    "scripts/stable_release_evidence_check.py",
    "scripts/desktop_distribution_publication_check.py",
    "scripts/enterprise_operations_readiness_check.py",
    "scripts/release_evidence_check.sh",
    "scripts/release_readiness_check.sh",
]:
    text = read(script)
    if text and "target/release-evidence" not in text:
        failures.append(f"{script} must write release evidence under target/release-evidence")

schema = read("schemas/release-evidence.schema.json")
if schema:
    try:
        data = json.loads(schema)
    except json.JSONDecodeError as exc:
        failures.append(f"release evidence schema invalid JSON: {exc}")
    else:
        props = data.get("properties", {})
        for field in ["sbom_path", "provenance_path", "signing_readiness_path"]:
            if field not in props:
                failures.append(f"release evidence schema missing {field}")
        for field in ["source_tag", "workflow_run", "builder", "materials"]:
            if field not in props:
                failures.append(f"release evidence schema missing provenance context field: {field}")

readiness_schema = read("schemas/release-evidence.schema.json")
if readiness_schema:
    try:
        readiness_schema_data = json.loads(readiness_schema)
    except json.JSONDecodeError as exc:
        failures.append(f"release readiness schema invalid JSON: {exc}")
    else:
        if readiness_schema_data.get("title") != "ReleaseEvidenceBundle":
            failures.append("release readiness schema title mismatch")

matrix = read("contracts/ci/contract-test-matrix.yaml")
if matrix:
    for token in [
        "release_provenance",
        "sbom_audit",
        "signing_readiness",
        "bash scripts/generate_provenance_attestation.sh",
        "bash scripts/sbom_audit.sh",
        "bash scripts/signing_readiness_check.sh",
    ]:
        if token not in matrix:
            failures.append(f"contract test matrix missing release provenance token: {token}")

docs = read("docs/reproducible-builds-provenance.md")
if docs:
    for token in [
        "scripts/generate_provenance_attestation.sh",
        "scripts/signing_readiness_check.sh",
        "target/release-evidence/provenance.json",
        "target/release-evidence/signing-readiness.json",
    ]:
        if token not in docs:
            failures.append(f"reproducible builds doc missing token: {token}")

evidence_dir = ROOT / "target/release-evidence"
if evidence_dir.exists():
    import os

    release_channel = os.environ.get("FE_RELEASE_CHANNEL", "dev")
    sbom = evidence_dir / "sbom.cdx.json"
    provenance = evidence_dir / "provenance.json"
    signing = evidence_dir / "signing-readiness.json"
    readiness = {
        "check": "release_provenance",
        "channel": release_channel,
        "status": "pass",
        "required_for_public_release": sorted(PUBLIC_CHANNELS),
        "sbom_path": str(sbom) if sbom.exists() else None,
        "provenance_path": str(provenance) if provenance.exists() else None,
        "signing_readiness_path": str(signing) if signing.exists() else None,
    }
    if release_channel in PUBLIC_CHANNELS:
        missing = []
        if not sbom.exists():
            missing.append(str(sbom))
        if not provenance.exists():
            missing.append(str(provenance))
        if not signing.exists():
            missing.append(str(signing))
        if missing:
            failures.append(f"public release channel missing provenance evidence: {', '.join(missing)}")
    (evidence_dir / "provenance-readiness.json").write_text(
        json.dumps(readiness, sort_keys=True) + "\n", encoding="utf-8"
    )
    if (evidence_dir / "release-readiness.json").exists():
        try:
            release_readiness = json.loads((evidence_dir / "release-readiness.json").read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            failures.append(f"release readiness artifact invalid JSON: {exc}")
        else:
            checks = release_readiness.get("checks", [])
            names = {check.get("name") for check in checks if isinstance(check, dict)}
            for check_name in [
                "required_release_files",
                "sbom_presence",
                "provenance_attestation",
                "signing_readiness",
                "desktop_packaging_signing",
                "stable_release_evidence",
                "desktop_distribution_publication",
                "enterprise_operations_readiness",
            ]:
                if check_name not in names:
                    failures.append(f"release readiness missing check: {check_name}")
            if release_readiness.get("channel") != release_channel:
                failures.append("release readiness channel mismatch")

if failures:
    print("RELEASE PROVENANCE CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("release provenance check passed")
