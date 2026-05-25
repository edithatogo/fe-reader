#!/usr/bin/env python3
import hashlib
import json
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
matrix = (root / "packaging/package-matrix.yaml").read_text(encoding="utf-8")
channels = (root / "packaging/release-channels.yaml").read_text(encoding="utf-8")
for token in ["windows:", "macos:", "linux:", "android:", "ios:"]:
    if token not in matrix:
        print(f"package matrix missing target: {token}", file=sys.stderr)
        raise SystemExit(1)
for token in ["nightly:", "preview:", "stable:"]:
    if token not in channels:
        print(f"release channels missing channel: {token}", file=sys.stderr)
        raise SystemExit(1)

evidence_dir = root / "target/release-evidence"
evidence_dir.mkdir(parents=True, exist_ok=True)
files = []
for rel in required:
    path = root / rel
    data = path.read_bytes()
    files.append(
        {
            "path": rel,
            "sha256": hashlib.sha256(data).hexdigest(),
            "bytes": len(data),
        }
    )
report = {
    "check": "release_matrix",
    "status": "pass",
    "required_files": files,
    "targets": ["windows", "macos", "linux", "android", "ios"],
    "channels": ["nightly", "preview", "stable"],
}
(evidence_dir / "release-matrix.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
print("release matrix: ok")
