#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import re
import sys
from typing import Iterable

ROOT = pathlib.Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "docs" / "pdf-parity-registry.json"
REGISTRY_DOC = ROOT / "docs" / "pdf-parity-registry.md"
BASELINE_MATRIX = ROOT / "docs" / "pdf-baseline-parity-matrix.json"
BASELINE_DOC = ROOT / "docs" / "pdf-baseline-parity-matrix.md"
REPORT = ROOT / "target" / "pdf-parity-registry-check.json"

TARGET_DOCS = [
    ROOT / "README.md",
    ROOT / "docs" / "launch-limitations-support.md",
    ROOT / "docs" / "stable-desktop-release.md",
    ROOT / "docs" / "release-notes" / "v0.1.0-preview.1.md",
    ROOT / "docs" / "pdf-baseline-parity-matrix.md",
    ROOT / "docs-site" / "src" / "content" / "docs" / "index.md",
    ROOT / "docs-site" / "src" / "content" / "docs" / "stable-reader-readiness.md",
    ROOT / "docs-site" / "src" / "content" / "docs" / "stable-desktop-release.md",
    ROOT / "docs-site" / "src" / "content" / "docs" / "pdf-baseline-parity.md",
]

NEGATIVE_CONTEXT = (
    "do not",
    "must not",
    "does not",
    "not a",
    "not an",
    "not the",
    "not yet",
    "deferred",
    "defer",
    "blocked",
    "limitation",
    "limitations",
    "remains",
    "remain",
    "gated",
)

CLAIM_KEYWORDS = re.compile(
    r"\b("
    r"open|inspect|render|search|extract|navigation|preview|support|supports|can|"
    r"forms?|annotations?|redact|repair|convert|print|export|lineariz|optimi"
    r"|ocr|accessibility|tagged|attachments?|portfolios?|layers?|multimedia|"
    r"signatures?|encryption|permissions?|pdf/a|pdf/ua|pdf/x|pdf 2\.0"
    r")\b",
    re.IGNORECASE,
)

REGISTRY_REFERENCES = (
    "pdf-parity-registry.md",
    "pdf-parity-registry.json",
    "pdf-baseline-parity-matrix.md",
    "pdf-baseline-parity-matrix.json",
)

MUTATING_FAMILIES = {
    "annotations-comments",
    "forms-active-content",
    "redaction",
    "page-organization",
    "optimization-linearization",
    "repair-incremental-updates",
    "conversion-printing-export",
}


def load_json(path: pathlib.Path) -> dict:
    if not path.exists():
        raise SystemExit(f"missing file: {path.relative_to(ROOT)}")
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise SystemExit(f"{path.relative_to(ROOT)} must contain a JSON object")
    return data


def rel(path: pathlib.Path) -> str:
    return str(path.relative_to(ROOT))


def claim_text_tokens(data: dict) -> tuple[set[str], set[str]]:
    ids: set[str] = set()
    texts: set[str] = set()
    for claim in data.get("claims", []):
        if not isinstance(claim, dict):
            continue
        claim_id = claim.get("id")
        public_claim = claim.get("public_claim")
        if isinstance(claim_id, str):
            ids.add(claim_id)
        if isinstance(public_claim, str):
            texts.add(public_claim)
    return ids, texts


def scan_docs(allowed_ids: set[str], allowed_claims: set[str]) -> list[str]:
    failures: list[str] = []
    for path in TARGET_DOCS:
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        for line_no, line in enumerate(text.splitlines(), start=1):
            lower = line.lower()
            if not lower.strip():
                continue
            if any(token in lower for token in NEGATIVE_CONTEXT):
                continue
            if not (CLAIM_KEYWORDS.search(lower) and "pdf" in lower):
                continue
            if any(reference in lower for reference in REGISTRY_REFERENCES):
                continue
            if any(claim_id in lower for claim_id in allowed_ids):
                continue
            if any(claim.lower() in lower for claim in allowed_claims):
                continue
            failures.append(
                f"{rel(path)}:{line_no} contains a PDF capability claim without registry linkage: {line.strip()}"
            )
    return failures


def main() -> int:
    failures: list[str] = []
    registry = load_json(REGISTRY)
    baseline = load_json(BASELINE_MATRIX)

    if registry.get("feature_gate") != "pdf_parity_registry":
        failures.append("registry feature_gate must be pdf_parity_registry")
    if registry.get("mutation_pipeline") != "OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt":
        failures.append("registry mutation pipeline must match the project contract")

    support_levels = registry.get("support_levels", [])
    required_support_levels = {
        "stable",
        "beta",
        "preview",
        "plan_only",
        "oracle_only",
        "blocked",
        "documented_limitation",
    }
    if set(support_levels) != required_support_levels:
        failures.append("registry support levels must enumerate the exhaustive parity states")

    families = registry.get("families", [])
    if not isinstance(families, list) or not families:
        failures.append("registry families must be a non-empty list")
        families = []
    family_ids = set()
    for family in families:
        if not isinstance(family, dict):
            failures.append("registry family entries must be objects")
            continue
        family_id = family.get("id")
        if not isinstance(family_id, str) or not family_id:
            failures.append("registry family missing id")
            continue
        if family_id in family_ids:
            failures.append(f"duplicate family id: {family_id}")
        family_ids.add(family_id)
        if family.get("default_support_level") not in required_support_levels:
            failures.append(f"family {family_id} has invalid default_support_level")

    claims = registry.get("claims", [])
    if not isinstance(claims, list) or not claims:
        failures.append("registry claims must be a non-empty list")
        claims = []
    claim_ids: set[str] = set()
    for claim in claims:
        if not isinstance(claim, dict):
            failures.append("registry claim entries must be objects")
            continue
        claim_id = claim.get("id")
        family = claim.get("family")
        support_level = claim.get("support_level")
        evidence = claim.get("evidence", [])
        limitations = claim.get("limitations", [])
        if not isinstance(claim_id, str) or not claim_id:
            failures.append("registry claim missing id")
            continue
        if claim_id in claim_ids:
            failures.append(f"duplicate claim id: {claim_id}")
        claim_ids.add(claim_id)
        if family not in family_ids:
            failures.append(f"{claim_id} references unknown family: {family}")
        if support_level not in required_support_levels:
            failures.append(f"{claim_id} has invalid support level: {support_level}")
        if not isinstance(evidence, list) or not evidence:
            failures.append(f"{claim_id} must include at least one evidence item")
        for item in evidence:
            if not isinstance(item, str):
                failures.append(f"{claim_id} evidence entries must be strings")
                continue
            if not (ROOT / item).exists():
                failures.append(f"{claim_id} evidence path missing: {item}")
        if not isinstance(limitations, list) or not limitations:
            failures.append(f"{claim_id} must include at least one limitation statement")
        if family in MUTATING_FAMILIES and claim.get("requires_mutation_pipeline") is not True:
            failures.append(f"{claim_id} must require the mutation pipeline")

    if registry.get("claims") and claim_ids:
        baseline_ids = {claim.get("id") for claim in baseline.get("claims", []) if isinstance(claim, dict)}
        missing_subset = sorted(str(claim_id) for claim_id in baseline_ids if claim_id not in claim_ids)
        if missing_subset:
            failures.append(f"baseline parity claims missing from registry: {missing_subset}")

    if baseline.get("feature_gate") != "advanced_pdf_baseline":
        failures.append("baseline matrix feature_gate must remain advanced_pdf_baseline")
    if baseline.get("launch_blocking") is not False:
        failures.append("baseline matrix must remain non-blocking")

    registry_ids, registry_claims = claim_text_tokens(registry)
    failures.extend(scan_docs(registry_ids, registry_claims))

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(
        json.dumps(
            {
                "check": "pdf_parity_registry",
                "status": "fail" if failures else "pass",
                "registry_version": registry.get("registry_version"),
                "family_count": len(family_ids),
                "claim_count": len(claim_ids),
                "baseline_claim_count": len(baseline.get("claims", [])) if isinstance(baseline.get("claims", []), list) else 0,
                "failures": failures,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )

    if failures:
        print("PDF PARITY REGISTRY CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1

    print("pdf parity registry: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
