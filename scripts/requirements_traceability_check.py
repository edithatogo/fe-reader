#!/usr/bin/env python3
"""Validate MoSCoW requirements traceability."""

from __future__ import annotations

import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MOSCOW = ROOT / "docs/requirements-moscow.md"
TRACE = ROOT / "docs/requirements-traceability.md"
MATRIX = ROOT / "contracts/ci/contract-test-matrix.yaml"

STATUS = {"Planned", "In Progress", "Implemented", "Blocked", "Deferred"}
ID_PREFIX = {
    "Must": "MUST",
    "Should": "SHOULD",
    "Could": "COULD",
    "Won't, initially": "WONT",
}


def fail(message: str) -> None:
    raise SystemExit(f"requirements traceability check failed: {message}")


def bullets_by_section() -> dict[str, list[str]]:
    section = None
    bullets: dict[str, list[str]] = {key: [] for key in ID_PREFIX}
    for line in MOSCOW.read_text(encoding="utf-8").splitlines():
        if line.startswith("## "):
            candidate = line.removeprefix("## ").strip()
            section = candidate if candidate in bullets else None
            continue
        if section and line.startswith("- "):
            bullets[section].append(line.removeprefix("- ").strip())
    return bullets


def parse_rows() -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for line in TRACE.read_text(encoding="utf-8").splitlines():
        if not line.startswith("| "):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if not cells or cells[0] in {"Req ID", "---"}:
            continue
        if len(cells) != 7:
            fail(f"traceability row has {len(cells)} cells, expected 7: {line}")
        rows.append(
            {
                "id": cells[0],
                "moscow": cells[1],
                "requirement": cells[2],
                "track": cells[3],
                "diagram": cells[4],
                "evidence": cells[5],
                "status": cells[6],
            }
        )
    return rows


def code_paths(cell: str) -> list[str]:
    return re.findall(r"`([^`]+)`", cell)


def require_path(path_text: str, kind: str) -> None:
    if not (ROOT / path_text).exists():
        fail(f"{kind} path does not exist: {path_text}")


def main() -> int:
    bullets = bullets_by_section()
    rows = parse_rows()
    if not rows:
        fail("traceability table has no data rows")

    seen_ids: set[str] = set()
    rows_by_requirement = {row["requirement"]: row for row in rows}

    expected_total = sum(len(items) for items in bullets.values())
    if len(rows) != expected_total:
        fail(f"expected {expected_total} traceability rows, found {len(rows)}")

    for section, items in bullets.items():
        prefix = ID_PREFIX[section]
        for index, requirement in enumerate(items, start=1):
            row = rows_by_requirement.get(requirement)
            if row is None:
                fail(f"missing traceability row for requirement: {requirement}")
            expected_id = f"{prefix}-{index:03d}"
            if row["id"] != expected_id:
                fail(f"{requirement!r} expected id {expected_id}, found {row['id']}")
            if row["moscow"] != section:
                fail(f"{row['id']} expected MoSCoW {section}, found {row['moscow']}")

    for row in rows:
        if row["id"] in seen_ids:
            fail(f"duplicate requirement id: {row['id']}")
        seen_ids.add(row["id"])
        if row["status"] not in STATUS:
            fail(f"{row['id']} has invalid status: {row['status']}")

        track_paths = code_paths(row["track"])
        diagram_paths = code_paths(row["diagram"])
        evidence_paths = code_paths(row["evidence"])
        if not track_paths:
            fail(f"{row['id']} has no linked track path")
        if not diagram_paths:
            fail(f"{row['id']} has no linked Mermaid path")
        if not evidence_paths:
            fail(f"{row['id']} has no linked evidence path")
        for path_text in track_paths:
            if not path_text.startswith("conductor/tracks/"):
                fail(f"{row['id']} track link is outside conductor/tracks: {path_text}")
            require_path(path_text, "track")
        for path_text in diagram_paths:
            if not path_text.startswith("docs/mermaid/") or not path_text.endswith(".mmd"):
                fail(f"{row['id']} diagram link is not a Mermaid file: {path_text}")
            require_path(path_text, "diagram")
        for path_text in evidence_paths:
            require_path(path_text, "evidence")
        if row["status"] == "Implemented" and not any(
            path.startswith("scripts/")
            or path.startswith("schemas/")
            or path.startswith("contracts/")
            or path.startswith("crates/")
            for path in evidence_paths
        ):
            fail(f"{row['id']} is Implemented but lacks executable or contract evidence")

    matrix_text = MATRIX.read_text(encoding="utf-8")
    if "requirements_traceability:" not in matrix_text:
        fail("contract matrix missing requirements_traceability entry")
    if "python3 scripts/requirements_traceability_check.py" not in matrix_text:
        fail("contract matrix missing requirements traceability command")

    print("requirements traceability check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
