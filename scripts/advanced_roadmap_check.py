#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
REPORT = EVIDENCE_DIR / "advanced-roadmap.json"

REQUIRED = [
    "docs/v2-roadmap-foundation.md",
    "docs/post-launch-advanced-roadmap.md",
    "docs/v2-roadmap-implementation-foundation-contract.md",
    "docs/stable-reader-readiness.md",
    "docs/marketing-readiness.md",
    "docs/pdf-parity-registry.md",
    "scripts/stable_reader_readiness_check.py",
    "scripts/marketing_claim_evidence_governance_check.py",
]

TOKENS = {
    "docs/v2-roadmap-foundation.md": ["Entry Gates", "v2 Themes", "Exit Criteria", "feature-gated"],
    "docs/post-launch-advanced-roadmap.md": ["track-BA-frontier-intelligence-governance", "track-BB-opt-in-collaboration-sync", "track-BO-v2-roadmap-implementation-foundation"],
    "docs/v2-roadmap-implementation-foundation-contract.md": ["v2 Roadmap Implementation Foundation Contract", "docs/post-launch-advanced-roadmap.md", "target/release-evidence/advanced-roadmap.json"],
    "docs/stable-reader-readiness.md": ["Stable publication still requires signed artifacts", "release evidence"],
    "docs/marketing-readiness.md": ["technical preview", "stable desktop", "v2 roadmap"],
}


def read_text(rel: str) -> str:
    path = ROOT / rel
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def main() -> int:
    failures: list[str] = []
    findings: list[dict[str, object]] = []
    for rel in REQUIRED:
        path = ROOT / rel
        if not path.exists():
            failures.append(f"missing roadmap file: {rel}")
            findings.append({"path": rel, "status": "missing"})
            continue
        text = read_text(rel)
        missing_tokens = [token for token in TOKENS.get(rel, []) if token not in text]
        if missing_tokens:
            failures.append(f"{rel} missing tokens: {missing_tokens}")
        findings.append({"path": rel, "status": "pass", "missing_tokens": missing_tokens})

    report = {
        "check": "advanced_roadmap",
        "status": "fail" if failures else "pass",
        "required_files": REQUIRED,
        "findings": findings,
        "failures": failures,
    }
    REPORT.write_text(json.dumps(report, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"advanced roadmap failure: {failure}")
        raise SystemExit(1)
    print("advanced roadmap: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
