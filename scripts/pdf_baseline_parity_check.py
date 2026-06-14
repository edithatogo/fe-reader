#!/usr/bin/env python3
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
MATRIX = ROOT / "docs" / "pdf-baseline-parity-matrix.json"
DOC = ROOT / "docs" / "pdf-baseline-parity-matrix.md"
OUT = ROOT / "target" / "pdf-baseline-parity-check.json"

REQUIRED_FAMILIES = {
    "reading",
    "search",
    "rendering",
    "page-organization",
    "annotations",
    "forms",
    "metadata",
    "redaction",
    "conversion",
    "signatures",
    "attachments-portfolios",
    "ocr",
}
REQUIRED_DOC_TOKENS = [
    "does not block desktop stable launch",
    "advanced_pdf_baseline",
    "OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt",
    "documented limitation",
]
MUTATING_FAMILIES = {
    "page-organization",
    "annotations",
    "forms",
    "metadata",
    "redaction",
    "conversion",
}
ALLOWED_SUPPORT_LEVELS = {
    "supported-preview",
    "plan-only",
    "plan-and-verify",
    "documented-limitation",
}


def rel(path: pathlib.Path) -> str:
    return str(path.relative_to(ROOT))


def main() -> int:
    failures: list[str] = []
    if not MATRIX.exists():
        failures.append(f"missing {rel(MATRIX)}")
        data = {}
    else:
        data = json.loads(MATRIX.read_text(encoding="utf-8"))

    doc_text = DOC.read_text(encoding="utf-8") if DOC.exists() else ""
    if not doc_text:
        failures.append(f"missing {rel(DOC)}")
    for token in REQUIRED_DOC_TOKENS:
        if token not in doc_text:
            failures.append(f"{rel(DOC)} missing token: {token}")

    if data.get("feature_gate") != "advanced_pdf_baseline":
        failures.append("matrix feature_gate must be advanced_pdf_baseline")
    if data.get("launch_blocking") is not False:
        failures.append("matrix must be non-blocking for desktop stable launch")
    governance = data.get("governance", {})
    if "OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt" not in governance.get("mutation_pipeline", ""):
        failures.append("governance must cite the mutation pipeline")

    claims = data.get("claims", [])
    if not isinstance(claims, list) or not claims:
        failures.append("matrix must contain claims")
        claims = []

    families = {claim.get("family") for claim in claims if isinstance(claim, dict)}
    missing_families = REQUIRED_FAMILIES - families
    if missing_families:
        failures.append(f"matrix missing families: {sorted(missing_families)}")

    seen_ids: set[str] = set()
    for claim in claims:
        if not isinstance(claim, dict):
            failures.append("claim entry must be an object")
            continue
        claim_id = claim.get("id")
        family = claim.get("family")
        support_level = claim.get("support_level")
        evidence = claim.get("evidence", [])
        limitation = claim.get("limitation", "")
        if not claim_id:
            failures.append("claim missing id")
        elif claim_id in seen_ids:
            failures.append(f"duplicate claim id: {claim_id}")
        else:
            seen_ids.add(claim_id)
        if support_level not in ALLOWED_SUPPORT_LEVELS:
            failures.append(f"{claim_id} has invalid support_level: {support_level}")
        if not claim.get("public_claim"):
            failures.append(f"{claim_id} missing public_claim")
        if not isinstance(evidence, list) or not evidence:
            failures.append(f"{claim_id} missing evidence")
            evidence = []
        for evidence_path in evidence:
            if not isinstance(evidence_path, str):
                failures.append(f"{claim_id} evidence path must be a string")
                continue
            if not (ROOT / evidence_path).exists():
                failures.append(f"{claim_id} evidence path missing: {evidence_path}")
        if support_level == "documented-limitation" and not limitation:
            failures.append(f"{claim_id} documented limitation must explain unsupported scope")
        if support_level != "supported-preview" and not limitation:
            failures.append(f"{claim_id} non-supported-preview claim must include limitation text")
        if family in MUTATING_FAMILIES and claim.get("requires_mutation_pipeline") is not True:
            failures.append(f"{claim_id} must require mutation pipeline")
        if claim_id and claim_id not in doc_text:
            failures.append(f"{rel(DOC)} should mention claim id {claim_id}")

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(
        json.dumps(
            {
                "check": "pdf_baseline_parity",
                "status": "fail" if failures else "pass",
                "claim_count": len(claims),
                "families": sorted(f for f in families if isinstance(f, str)),
                "failures": failures,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )

    if failures:
        print("PDF BASELINE PARITY CHECK FAILED")
        for failure in failures:
            print(f" - {failure}")
        return 1
    print("pdf baseline parity: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
