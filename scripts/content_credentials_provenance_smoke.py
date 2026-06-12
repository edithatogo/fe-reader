#!/usr/bin/env python3
"""Smoke-test the Content Credentials provenance scaffold.

This check intentionally verifies contract evidence only. It must not claim
that Wave 0 builds emit a signed C2PA manifest before signing credentials and
the manifest writer are available.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target/release-evidence"
PROVENANCE = EVIDENCE_DIR / "provenance.json"
REPORT = EVIDENCE_DIR / "content-credentials-provenance-smoke.json"
DOC = ROOT / "docs/reproducible-builds-provenance.md"


def fail(message: str) -> None:
    raise SystemExit(f"content credentials provenance smoke failed: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def main() -> int:
    subprocess.run(["bash", "scripts/generate_provenance_attestation.sh"], cwd=ROOT, check=True)

    require(PROVENANCE.exists(), "missing generated provenance.json")
    provenance = json.loads(PROVENANCE.read_text(encoding="utf-8"))
    require(provenance.get("_type") == "https://in-toto.io/Statement/v1", "wrong in-toto statement type")
    require(provenance.get("predicateType") == "https://slsa.dev/provenance/v1", "wrong predicate type")
    require(bool(provenance.get("subject")), "missing provenance subject list")

    subject = provenance.get("subject")
    require(isinstance(subject, list) and subject, "missing provenance subject")
    digest = subject[0].get("digest", {}) if isinstance(subject[0], dict) else {}
    require(bool(digest.get("gitCommit")), "missing source commit digest")

    predicate = provenance.get("predicate", {})
    build_definition = predicate.get("buildDefinition", {})
    require(
        build_definition.get("buildType") == "https://github.com/fereader/fe-reader/bootstrap-release",
        "unexpected provenance build type",
    )
    external_parameters = build_definition.get("externalParameters", {})
    require(external_parameters.get("channel") in {"dev", "nightly", "preview", "beta", "stable", "lts", "store_submission"}, "unexpected provenance channel")
    internal_parameters = build_definition.get("internalParameters", {})
    require("workflow" in internal_parameters, "missing provenance internal workflow parameter")
    materials = build_definition.get("resolvedDependencies", [])
    require(isinstance(materials, list) and materials, "missing hashed build materials")
    for material in materials:
        require("uri" in material, "material missing uri")
        require(bool(material.get("digest", {}).get("sha256")), f"material missing sha256 digest: {material}")

    run_details = predicate.get("runDetails", {})
    require("builder" in run_details, "missing provenance run builder details")
    require("metadata" in run_details, "missing provenance run metadata")

    docs = DOC.read_text(encoding="utf-8")
    for token in [
        "C2PA / Content Credentials",
        "contract-only",
        "not a cryptographic C2PA manifest",
        "target/release-evidence/content-credentials-provenance-smoke.json",
        "later implementation needs a feature gate",
    ]:
        require(token in docs, f"provenance doc missing token: {token}")

    report = {
        "check": "content_credentials_provenance",
        "status": "pass",
        "content_credentials_status": "contract_only",
        "cryptographic_c2pa_manifest": False,
        "provenance_path": "target/release-evidence/provenance.json",
        "materials_count": len(materials),
        "limitations": [
            "No signed C2PA manifest is emitted in Wave 0.",
            "No release artifact is embedded with Content Credentials metadata yet.",
        ],
    }
    REPORT.write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
    print(f"content credentials provenance smoke passed: {REPORT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
