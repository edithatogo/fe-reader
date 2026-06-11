#!/usr/bin/env python3
"""Validate the Wave 0 first-30 PR sequencing plan."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PLAN = ROOT / "docs/wave0-first-30-prs.md"
V9 = ROOT / "docs/v9-coding-agent-start-here.md"
REPORT = ROOT / "target/release-evidence/wave0-first-30-prs-check.json"


def fail(message: str) -> None:
    raise SystemExit(f"wave0 first-30 PR check failed: {message}")


def main() -> int:
    if not PLAN.exists():
        fail("missing docs/wave0-first-30-prs.md")
    if not V9.exists():
        fail("missing docs/v9-coding-agent-start-here.md")

    rows = []
    for line in PLAN.read_text(encoding="utf-8").splitlines():
        if not line.startswith("| ") or line.startswith("| PR "):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) != 4:
            continue
        if not cells[0].isdigit():
            continue
        rows.append(cells)

    if len(rows) != 30:
        fail(f"expected 30 PR rows, found {len(rows)}")

    pr_numbers = [int(row[0]) for row in rows]
    if pr_numbers != list(range(1, 31)):
        fail("PR rows must be numbered 1 through 30 in order")

    titles = [row[2] for row in rows]
    if len(set(titles)) != len(titles):
        fail("PR titles must be unique")

    owners = [row[1] for row in rows]
    if len(set(owners)) < 10:
        fail("plan should span multiple owner tracks for parallel work")

    text = PLAN.read_text(encoding="utf-8")
    for token in [
        "These PRs are intentionally small",
        "Do not combine unrelated PRs",
        "split it",
    ]:
        if token not in text:
            fail(f"plan missing guidance token: {token}")

    v9 = V9.read_text(encoding="utf-8")
    for token in [
        "Required first PRs after v9 extraction",
        "Make `strict_contract_check.py` and `repository_ci_cd_check.py` required PR statuses.",
        "Do not add new product features until:",
    ]:
        if token not in v9:
            fail(f"v9 start page missing token: {token}")

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(
        json.dumps(
            {
                "check": "wave0_first_30_prs",
                "status": "pass",
                "first_pr": rows[0],
                "last_pr": rows[-1],
                "owner_count": len(set(owners)),
                "titles_count": len(set(titles)),
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"wave0 first-30 PR check: {REPORT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
