#!/usr/bin/env python3
"""Validate the Wave 0 PDF model summary contract."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/pdf-document-summary.schema.json"
SNAPSHOT = ROOT / "contracts/snapshots/rust-public-api/fe_reader_pdf_model.preview.json"
MINIMAL = "fixtures/minimal/minimal.pdf"
MINIMAL_V1_0 = "fixtures/corpus/basic/minimal-v1_0.pdf"
MINIMAL_V2_0 = "fixtures/corpus/basic/minimal-v2_0.pdf"
MALFORMED = "fixtures/corpus/malformed-adversarial/truncated-catalog.pdf"


def fail(message: str) -> None:
    raise SystemExit(f"pdf model contract check failed: {message}")


def inspect(path: str) -> dict[str, object]:
    output = subprocess.check_output(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "fe_reader_cli",
            "--",
            "inspect",
            path,
            "--json",
        ],
        cwd=ROOT,
        text=True,
    )
    payload = json.loads(output)
    if not isinstance(payload, dict):
        fail("CLI inspect output must be a JSON object")
    return payload


def validate_with_jsonschema(instance: object, schema: object) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(instance=instance, schema=schema)


def require_schema_tokens() -> None:
    schema_text = SCHEMA.read_text(encoding="utf-8")
    for token in (
        '"document_id"',
        '"type": "string"',
        '"fingerprint"',
        '"parser"',
        '"page_count"',
        '"trailer_keys"',
        '"error"',
        '"additionalProperties": false',
    ):
        if token not in schema_text:
            fail(f"PDF document summary schema missing token {token}")


def require_snapshot() -> None:
    try:
        snapshot = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"PDF model snapshot is not valid JSON: {exc}")

    expected = {
        "surface": "rust_crate",
        "crate": "fe_reader_pdf_model",
        "stability": "preview",
        "phase": "A3",
        "contract": "pdf_model",
    }
    for key, value in expected.items():
        if snapshot.get(key) != value:
            fail(f"PDF model snapshot {key} expected {value!r}, got {snapshot.get(key)!r}")

    public_types = set(snapshot.get("public_types", []))
    for type_name in ("PdfDocumentSummary", "PdfHeader", "PdfParserSummary", "PdfRect"):
        if type_name not in public_types:
            fail(f"PDF model snapshot missing public type {type_name}")


def require_minimal_summary() -> None:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    payload = inspect(MINIMAL)
    summary = payload.get("summary")
    if not isinstance(summary, dict):
        fail("minimal CLI inspect output missing summary object")
    validate_with_jsonschema(summary, schema)

    document_id = summary.get("document_id")
    if not isinstance(document_id, str):
        fail("summary document_id must serialize as a string")
    if document_id != payload.get("intent", {}).get("document_id"):
        fail("summary document_id must match the emitted intent")
    if document_id != payload.get("plan", {}).get("document_id"):
        fail("summary document_id must match the emitted plan")

    fingerprint = summary.get("fingerprint")
    if not isinstance(fingerprint, dict):
        fail("summary fingerprint must be an object")
    sha256_hex = fingerprint.get("sha256_hex")
    if not isinstance(sha256_hex, str) or not re.fullmatch(r"[0-9a-f]{64}", sha256_hex):
        fail("summary fingerprint sha256_hex must be lowercase hex")
    intent_fingerprint = payload.get("intent", {}).get("document_fingerprint")
    if fingerprint != intent_fingerprint:
        fail("summary fingerprint must match the emitted intent fingerprint")

    header = summary.get("header")
    if not isinstance(header, dict):
        fail("summary header must be an object")
    if header.get("version") != "1.5":
        fail(f"minimal fixture header version expected '1.5', got {header.get('version')!r}")
    if summary.get("eof_marker_hint") is not True:
        fail("minimal fixture must report an EOF marker")
    if summary.get("encrypted_hint") is not False:
        fail("minimal fixture must not report an encrypted hint")
    if summary.get("linearized_hint") is not False:
        fail("minimal fixture must not report a linearized hint")

    parser = summary.get("parser")
    if not isinstance(parser, dict):
        fail("summary parser must be an object")
    expected = {
        "adapter": "lopdf",
        "page_count": 1,
        "encrypted": False,
        "error": None,
    }
    for key, value in expected.items():
        if parser.get(key) != value:
            fail(f"minimal parser {key} expected {value!r}, got {parser.get(key)!r}")
    if "Root" not in parser.get("trailer_keys", []):
        fail("minimal parser trailer keys must include Root")


def require_versioned_fixture(path: str, expected_version: str) -> None:
    payload = inspect(path)
    summary = payload.get("summary")
    if not isinstance(summary, dict):
        fail(f"{path} inspect output missing summary object")
    validate_with_jsonschema(summary, json.loads(SCHEMA.read_text(encoding="utf-8")))

    header = summary.get("header")
    if not isinstance(header, dict):
        fail(f"{path} header must be an object")
    if header.get("version") != expected_version:
        fail(
            f"{path} expected header version {expected_version!r}, got {header.get('version')!r}"
        )

    parser = summary.get("parser")
    if not isinstance(parser, dict):
        fail(f"{path} parser must be an object")
    if parser.get("adapter") != "lopdf":
        fail(f"{path} parser adapter must be lopdf")
    if parser.get("page_count") != 1:
        fail(f"{path} parser must report one page")
    if parser.get("error") is not None:
        fail(f"{path} parser must not report an error")
    if summary.get("eof_marker_hint") is not True:
        fail(f"{path} must report an EOF marker")


def require_malformed_summary() -> None:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    summary = inspect(MALFORMED).get("summary")
    if not isinstance(summary, dict):
        fail("malformed CLI inspect output missing summary object")
    validate_with_jsonschema(summary, schema)

    if summary.get("header", {}).get("version") != "1.7":
        fail("malformed fixture must still report its PDF header version")
    if summary.get("eof_marker_hint") is not False:
        fail("malformed truncated fixture must not report an EOF marker")
    parser = summary.get("parser")
    if not isinstance(parser, dict):
        fail("malformed summary parser must be an object")
    if parser.get("adapter") != "lopdf":
        fail("malformed summary parser adapter must be lopdf")
    if parser.get("error") in (None, ""):
        fail("malformed fixture must surface a non-fatal parser error")
    if parser.get("page_count") is not None:
        fail("malformed fixture must not claim a page count")
    if parser.get("version") is not None:
        fail("malformed fixture must not claim a parser PDF version")
    if parser.get("encrypted") is not None:
        fail("malformed fixture must not claim parser encryption status")


def main() -> int:
    require_schema_tokens()
    require_snapshot()
    require_minimal_summary()
    require_versioned_fixture(MINIMAL_V1_0, "1.0")
    require_versioned_fixture(MINIMAL_V2_0, "2.0")
    require_malformed_summary()
    print("pdf model contract check passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
