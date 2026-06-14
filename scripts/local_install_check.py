#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(rel: str) -> str:
    path = ROOT / rel
    if not path.exists():
        raise SystemExit(f"missing install helper: {rel}")
    return path.read_text(encoding="utf-8", errors="replace")


checks = {
    "scripts/install_local.sh": [
        "cargo build",
        "install -m 0755",
        "fe-reader doctor",
        "PROFILE",
        "PREFIX",
    ],
    "scripts/install_windows.ps1": [
        "cargo",
        "Copy-Item",
        "RegisterFileAssociation",
        "AllUsers install requires",
        "fe-reader doctor",
    ],
}

for rel, tokens in checks.items():
    text = read(rel)
    lowered = text.lower()
    if "placeholder" in lowered:
        raise SystemExit(f"{rel} must not contain placeholder install text")
    for token in tokens:
        if token not in text:
            raise SystemExit(f"{rel} missing token: {token}")

print("local install helpers: ok")
