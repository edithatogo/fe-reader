#!/usr/bin/env python3
import hashlib
import json
import pathlib
import sys

import yaml

root = pathlib.Path(__file__).resolve().parents[1]
evidence_dir = root / "target" / "release-evidence"
evidence_dir.mkdir(parents=True, exist_ok=True)
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
matrix_doc = yaml.safe_load(matrix)
channels_doc = yaml.safe_load(channels)
if not isinstance(matrix_doc, dict) or "targets" not in matrix_doc:
    print("package matrix missing targets mapping", file=sys.stderr)
    raise SystemExit(1)
if not isinstance(channels_doc, dict) or "channels" not in channels_doc:
    print("release channels missing channels mapping", file=sys.stderr)
    raise SystemExit(1)

target_checks = {
    "windows": {"local_user", "global_admin", "stores", "bindings"},
    "macos": {"local_user", "global_admin", "stores"},
    "linux": {"local_user", "global_admin", "registries"},
    "android": {"stores", "evaluate"},
    "ios": {"stores"},
}
for platform, subkeys in target_checks.items():
    platform_targets = matrix_doc["targets"].get(platform)
    if not isinstance(platform_targets, dict):
        print(f"package matrix missing platform mapping: {platform}", file=sys.stderr)
        raise SystemExit(1)
    if set(platform_targets) != subkeys:
        print(f"package matrix platform keys mismatch for {platform}: {sorted(platform_targets)}", file=sys.stderr)
        raise SystemExit(1)

channel_checks = {
    "nightly": {"signing": "optional", "publishing": "github_prerelease", "notes": "developer/test only"},
    "preview": {"signing": "required_where_supported", "publishing": "prerelease_registries", "notes": "beta users"},
    "stable": {"signing": "required", "notarization": "required_on_macos", "publishing": "all_configured_registries"},
}
for channel, expected in channel_checks.items():
    channel_doc = channels_doc["channels"].get(channel)
    if not isinstance(channel_doc, dict):
        print(f"release channels missing channel mapping: {channel}", file=sys.stderr)
        raise SystemExit(1)
    for key, value in expected.items():
        if channel_doc.get(key) != value:
            print(f"release channels policy mismatch for {channel}.{key}: {channel_doc.get(key)!r}", file=sys.stderr)
            raise SystemExit(1)

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
for entry in report["required_files"]:
    if set(entry) != {"path", "sha256", "bytes"}:
        print(f"release matrix entry shape mismatch: {entry}", file=sys.stderr)
        raise SystemExit(1)
    if len(entry["sha256"]) != 64:
        print(f"release matrix digest mismatch: {entry}", file=sys.stderr)
        raise SystemExit(1)
if report["targets"] != ["windows", "macos", "linux", "android", "ios"]:
    print("release matrix target ordering mismatch", file=sys.stderr)
    raise SystemExit(1)
if report["channels"] != ["nightly", "preview", "stable"]:
    print("release matrix channel ordering mismatch", file=sys.stderr)
    raise SystemExit(1)
(evidence_dir / "release-matrix.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
print("release matrix: ok")
