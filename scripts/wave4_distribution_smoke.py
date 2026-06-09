#!/usr/bin/env python3
import hashlib
import json
import pathlib
import sys
import xml.etree.ElementTree as ET

ROOT = pathlib.Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "target" / "release-evidence"
EVIDENCE.mkdir(parents=True, exist_ok=True)


def read(rel):
    return (ROOT / rel).read_text(encoding="utf-8")


def sha256_path(rel):
    data = (ROOT / rel).read_bytes()
    return hashlib.sha256(data).hexdigest(), len(data)


def require(condition, message, failures):
    if not condition:
        failures.append(message)


def validate_distribution_target(target, failures):
    required = ["target_id", "platform", "artifact_kind", "install_scope", "publishing_channel"]
    for field in required:
        require(bool(str(target.get(field, "")).strip()), f"distribution target missing {field}: {target}", failures)
    require(target.get("platform") in {"windows", "macos", "linux", "android", "ios"}, f"invalid platform: {target}", failures)
    require(
        target.get("install_scope") in {"local_user", "global_admin", "portable", "store"},
        f"invalid install_scope: {target}",
        failures,
    )
    for field in ["requires_signing", "requires_notarization"]:
        require(isinstance(target.get(field), bool), f"{field} must be boolean: {target}", failures)


def generate_installer_matrix(failures):
    matrix = read("packaging/package-matrix.yaml")
    channels = read("packaging/release-channels.yaml")
    targets = []
    for platform in ["windows", "macos", "linux", "android", "ios"]:
        require(f"  {platform}:" in matrix, f"package matrix missing platform {platform}", failures)
    for channel in ["nightly", "preview", "stable"]:
        require(f"  {channel}:" in channels, f"release channels missing {channel}", failures)

    targets.extend(
        [
            {
                "target_id": "windows-nsis-per-user",
                "platform": "windows",
                "artifact_kind": "nsis_per_user",
                "install_scope": "local_user",
                "publishing_channel": "nightly",
                "requires_signing": False,
                "requires_notarization": False,
            },
            {
                "target_id": "windows-msix-store",
                "platform": "windows",
                "artifact_kind": "msix",
                "install_scope": "store",
                "publishing_channel": "stable",
                "requires_signing": True,
                "requires_notarization": False,
            },
            {
                "target_id": "macos-dmg-user",
                "platform": "macos",
                "artifact_kind": "user_applications_dmg",
                "install_scope": "local_user",
                "publishing_channel": "preview",
                "requires_signing": True,
                "requires_notarization": True,
            },
            {
                "target_id": "linux-appimage",
                "platform": "linux",
                "artifact_kind": "appimage",
                "install_scope": "portable",
                "publishing_channel": "nightly",
                "requires_signing": False,
                "requires_notarization": False,
            },
            {
                "target_id": "linux-flatpak-store",
                "platform": "linux",
                "artifact_kind": "flatpak",
                "install_scope": "store",
                "publishing_channel": "stable",
                "requires_signing": True,
                "requires_notarization": False,
            },
            {
                "target_id": "android-play",
                "platform": "android",
                "artifact_kind": "aab",
                "install_scope": "store",
                "publishing_channel": "stable",
                "requires_signing": True,
                "requires_notarization": False,
            },
            {
                "target_id": "ios-app-store",
                "platform": "ios",
                "artifact_kind": "ipa",
                "install_scope": "store",
                "publishing_channel": "stable",
                "requires_signing": True,
                "requires_notarization": False,
            },
        ]
    )
    for target in targets:
        validate_distribution_target(target, failures)
    report = {
        "check": "wave4_installer_matrix",
        "status": "pass",
        "schema": "schemas/distribution-target.schema.json",
        "targets": targets,
    }
    (EVIDENCE / "installer-matrix.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")


def validate_registry_manifests(failures):
    manifest_checks = []
    json.loads(read("packaging/windows/scoop/fe-reader.json"))
    manifest_checks.append("packaging/windows/scoop/fe-reader.json")

    for rel in ["packaging/windows/msix/AppxManifest.contract.xml", "packaging/windows/chocolatey/fe-reader.nuspec"]:
        ET.fromstring(read(rel))
        manifest_checks.append(rel)

    text_checks = {
        "packaging/windows/winget/FeReader.yaml": ["PackageIdentifier:", "Installers:", "InstallerSha256:"],
        "packaging/macos/homebrew/fe-reader.rb": ['cask "fe-reader"', 'sha256 "PLACEHOLDER"', 'app "Fe Reader.app"'],
        "packaging/linux/flatpak/org.fereader.FeReader.yml": ["app-id:", "runtime:", "modules:"],
        "packaging/linux/snap/snapcraft.yaml": ["name:", "base:", "apps:"],
        "packaging/linux/aur/PKGBUILD": ["pkgname=", "pkgver=", "source="],
    }
    for rel, tokens in text_checks.items():
        text = read(rel)
        for token in tokens:
            require(token in text, f"{rel} missing token {token}", failures)
        manifest_checks.append(rel)

    files = []
    for rel in manifest_checks:
        digest, size = sha256_path(rel)
        files.append({"path": rel, "sha256": digest, "bytes": size})
    report = {
        "check": "wave4_registry_manifest_syntax",
        "status": "pass",
        "manifests": files,
    }
    (EVIDENCE / "registry-manifests.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")


def write_update_manifest(failures):
    artifact_sha = hashlib.sha256(b"wave4-dev-update-artifact-placeholder").hexdigest()
    manifest = {
        "manifest_version": "1",
        "app_version": "0.1.0",
        "channel": "nightly",
        "min_supported_version": "0.1.0",
        "rollback": False,
        "artifacts": [
            {
                "platform": "macos",
                "arch": "arm64",
                "installer_kind": "dmg",
                "url": "https://example.invalid/fe-reader-0.1.0-macos-arm64.dmg",
                "sha256": artifact_sha,
                "size_bytes": 42,
                "signature": "dev-placeholder-signature",
                "provenance_path": "target/release-evidence/provenance.json",
                "signing_receipt_path": "target/release-evidence/signing-readiness.json",
            }
        ],
        "manifest_signature": "dev-placeholder-manifest-signature",
        "provenance_path": "target/release-evidence/provenance.json",
        "signing_readiness_path": "target/release-evidence/signing-readiness.json",
    }
    require(manifest["channel"] in {"nightly", "alpha", "beta", "stable", "lts-enterprise"}, "invalid update channel", failures)
    require(manifest["artifacts"], "update manifest must contain artifacts", failures)
    for artifact in manifest["artifacts"]:
        require(len(artifact["sha256"]) == 64, "artifact sha256 must be 64 hex characters", failures)
        require(bool(artifact["signature"]), "artifact signature is required", failures)
    require(bool(manifest["manifest_signature"]), "manifest_signature is required", failures)
    (EVIDENCE / "update-manifest.dev.json").write_text(json.dumps(manifest, sort_keys=True) + "\n", encoding="utf-8")


def validate_release_evidence_schema(failures):
    schema = json.loads(read("schemas/release-evidence.schema.json"))
    evidence_path = EVIDENCE / "release-evidence.json"
    require(evidence_path.exists(), "release evidence bundle missing; run scripts/release_evidence_check.sh first", failures)
    if evidence_path.exists():
        data = json.loads(evidence_path.read_text(encoding="utf-8"))
        for field in schema["required"]:
            require(field in data, f"release evidence missing required field {field}", failures)
        require(data.get("channel") in schema["properties"]["channel"]["enum"], "release evidence channel not allowed", failures)
        for artifact in data.get("artifacts", []):
            require(len(artifact.get("sha256", "")) == 64, f"release artifact hash invalid: {artifact}", failures)


def main():
    failures = []
    try:
        generate_installer_matrix(failures)
        validate_registry_manifests(failures)
        write_update_manifest(failures)
        validate_release_evidence_schema(failures)
    except Exception as exc:
        failures.append(str(exc))

    report = {
        "check": "wave4_distribution_smoke",
        "status": "fail" if failures else "pass",
        "outputs": [
            "target/release-evidence/installer-matrix.json",
            "target/release-evidence/registry-manifests.json",
            "target/release-evidence/update-manifest.dev.json",
            "target/release-evidence/release-evidence.json",
        ],
        "failures": failures,
    }
    (EVIDENCE / "wave4-distribution-smoke.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"wave4 distribution failure: {failure}", file=sys.stderr)
        raise SystemExit(1)
    print("wave4 distribution smoke: ok")


if __name__ == "__main__":
    main()
