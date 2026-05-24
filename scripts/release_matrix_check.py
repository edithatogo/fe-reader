#!/usr/bin/env python3
import pathlib
import sys

root = pathlib.Path(__file__).resolve().parents[1]
required = [
    "packaging/package-matrix.yaml",
    "packaging/release-channels.yaml",
    "packaging/windows/winget/FeReader.yaml",
    "packaging/windows/scoop/fe-reader.json",
    "packaging/macos/homebrew/fe-reader.rb",
    "packaging/linux/flatpak/org.fereader.FeReader.yml",
    "packaging/linux/snap/snapcraft.yaml",
]
missing = [p for p in required if not (root / p).exists()]
if missing:
    print("missing packaging files:", missing, file=sys.stderr)
    raise SystemExit(1)
print("release matrix: ok")
