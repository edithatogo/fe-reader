#!/usr/bin/env python3
import argparse
import pathlib
import sys

FORBIDDEN_IN_CORE = [
    "tauri", "pdfium-render", "rmcp", "extism", "wasmtime", "candle", "burn",
    "wgpu", "vello", "skia-safe", "windows", "cocoa", "objc", "jni", "android",
    "wry", "webview", "dioxus", "iced"
]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--workspace-root", default=".")
    args = parser.parse_args()
    root = pathlib.Path(args.workspace_root)
    core_manifest = root / "crates" / "fe_reader_core" / "Cargo.toml"
    if not core_manifest.exists():
        sample = root / "crates" / "fe_reader_core" / "Cargo.toml.sample"
        core_manifest = sample if sample.exists() else core_manifest
    if not core_manifest.exists():
        print("architecture check: core manifest not found; expected after Wave 0", file=sys.stderr)
        return 0
    text = core_manifest.read_text(encoding="utf-8").lower()
    failures = [dep for dep in FORBIDDEN_IN_CORE if dep in text]
    if failures:
        print("Forbidden dependencies in fe_reader_core:", ", ".join(failures), file=sys.stderr)
        return 1
    print("architecture check: ok")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
