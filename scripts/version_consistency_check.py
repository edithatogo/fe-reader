#!/usr/bin/env python3
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PRODUCT_VERSION = "0.1.0-preview.1"
CRATE_VERSION = "0.1.0"
LICENSE = "Apache-2.0 OR MIT"
REPO_URL = "https://github.com/edithatogo/fe-reader"
DOCS_URL = "https://edithatogo.github.io/fe-reader/"

failures: list[str] = []


def read(path: str) -> str:
    full = ROOT / path
    if not full.exists():
        failures.append(f"missing versioned file: {path}")
        return ""
    return full.read_text(encoding="utf-8", errors="replace")


def require(path: str, token: str) -> None:
    text = read(path)
    if token not in text:
        failures.append(f"{path} missing token: {token}")


for path in [
    "README.md",
    "VERSION.md",
    "CITATION.cff",
    "packaging/registry-status.yaml",
]:
    require(path, PRODUCT_VERSION)

for path in [
    "README.md",
    "CITATION.cff",
    "Cargo.toml",
    "packaging/nuget/FeReader.Native/FeReader.Native.csproj",
]:
    require(path, LICENSE)

for path in ["README.md", "CITATION.cff", "packaging/registry-status.yaml"]:
    require(path, REPO_URL)

require("README.md", DOCS_URL)
require("LICENSE", "LICENSE-APACHE")
require("LICENSE", "LICENSE-MIT")
require("LICENSE-APACHE", "Apache License")
require("LICENSE-MIT", "MIT License")

citation = read("CITATION.cff")
for token in [f'version: "{PRODUCT_VERSION}"', "cff-version: 1.2.0", f'license: "{LICENSE}"']:
    if token not in citation:
        failures.append(f"CITATION.cff missing token: {token}")

workspace = read("Cargo.toml")
if f'license = "{LICENSE}"' not in workspace:
    failures.append("Cargo workspace license must stay Apache-2.0 OR MIT")

for cargo_toml in sorted((ROOT / "crates").glob("*/Cargo.toml")):
    text = cargo_toml.read_text(encoding="utf-8")
    if cargo_toml.parent.name == "xtask":
        expected = CRATE_VERSION
    else:
        expected = CRATE_VERSION
    match = re.search(r'^version\s*=\s*"([^"]+)"', text, re.M)
    if not match:
        failures.append(f"{cargo_toml.relative_to(ROOT)} missing package version")
    elif match.group(1) != expected:
        failures.append(
            f"{cargo_toml.relative_to(ROOT)} version {match.group(1)} != {expected}"
        )
    if "license.workspace = true" not in text:
        failures.append(f"{cargo_toml.relative_to(ROOT)} must inherit workspace license")

nuget_project = read("packaging/nuget/FeReader.Native/FeReader.Native.csproj")
for token in [
    f"<Version>{PRODUCT_VERSION}</Version>",
    f"<PackageLicenseExpression>{LICENSE}</PackageLicenseExpression>",
    f"<RepositoryUrl>{REPO_URL}</RepositoryUrl>",
]:
    if token not in nuget_project:
        failures.append(f"FeReader.Native.csproj missing token: {token}")

nuget_nuspec = read("packaging/nuget/FeReader.Native.nuspec")
for token in [
    f"<version>{PRODUCT_VERSION}</version>",
    f'<license type="expression">{LICENSE}</license>',
    f'<repository type="git" url="{REPO_URL}" />',
]:
    if token not in nuget_nuspec:
        failures.append(f"FeReader.Native.nuspec missing token: {token}")

registry_status = read("packaging/registry-status.yaml")
for token in [
    "crates_io:",
    "nuget:",
    "homebrew:",
    "winget:",
    "chocolatey:",
    "scoop:",
    "flatpak:",
    "snap:",
    "aur:",
    "google_play:",
    "app_store:",
    "github_releases:",
]:
    if token not in registry_status:
        failures.append(f"registry status missing token: {token}")

if failures:
    print("VERSION CONSISTENCY CHECK FAILED")
    for failure in failures:
        print(f" - {failure}")
    sys.exit(1)

print("version consistency check passed")
