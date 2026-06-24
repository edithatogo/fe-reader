#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE_DIR = ROOT / "target" / "release-evidence"
EVIDENCE_DIR.mkdir(parents=True, exist_ok=True)
REPORT = EVIDENCE_DIR / "marketing-readiness.json"

TARGETS = [
    "README.md",
    "docs/marketing-readiness.md",
    "docs/stable-desktop-release.md",
    "docs/launch-limitations-support.md",
    "docs/release-notes/v0.1.0-preview.1.md",
    "docs-site/src/content/docs/index.md",
    "docs-site/src/content/docs/stable-desktop-release.md",
    "docs-site/src/content/docs/stable-reader-readiness.md",
    "docs-site/package.json",
    ".github/repository-metadata.yaml",
]

SUPPORTED_PHRASES = [
    "technical preview",
    "public beta",
    "stable desktop",
    "mature stable",
    "v2 roadmap",
]

UNSUPPORTED_PHRASES = [
    "marketing ready",
    "launch ready",
    "fully stable",
    "production ready",
    "parity complete",
    "vendor clone",
]


def read_text(rel: str) -> str:
    path = ROOT / rel
    return path.read_text(encoding="utf-8", errors="replace") if path.exists() else ""


def main() -> int:
    failures: list[str] = []
    findings: list[dict[str, object]] = []
    for rel in TARGETS:
        text = read_text(rel)
        if not text:
            failures.append(f"missing file: {rel}")
            continue
        lower = text.lower()
        matches = [phrase for phrase in UNSUPPORTED_PHRASES if phrase in lower]
        supported = [phrase for phrase in SUPPORTED_PHRASES if phrase in lower]
        findings.append({"path": rel, "supported": supported, "unsupported": matches})
        if rel in {"README.md", "docs-site/src/content/docs/index.md", "docs-site/src/content/docs/stable-desktop-release.md"}:
            if "technical preview" not in lower:
                failures.append(f"{rel} must identify the current line as technical preview")
        if matches:
            failures.append(f"{rel} contains unsupported marketing phrase(s): {', '.join(matches)}")

    report = {
        "check": "marketing_claim_evidence_governance",
        "status": "fail" if failures else "pass",
        "evidence_bundle": "target/release-evidence/marketing-readiness.json",
        "targets": TARGETS,
        "findings": findings,
        "failures": failures,
        "readiness_levels": [
            "technical preview",
            "public beta",
            "stable desktop",
            "mature stable",
            "v2 roadmap",
        ],
    }
    REPORT.write_text(json.dumps(report, sort_keys=True, indent=2) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"marketing claim governance failure: {failure}")
        raise SystemExit(1)
    print("marketing claim governance: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
