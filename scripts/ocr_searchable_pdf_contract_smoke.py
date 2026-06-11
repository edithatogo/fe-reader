#!/usr/bin/env python3
"""Validate the OCR/searchable PDF provider contract.

The smoke test is intentionally provider-neutral. It verifies that OCR work is
represented as a conversion job with explicit confidence, bounding-box and
native-vs-OCR text provenance requirements, without adding an OCR engine to
fe_reader_core.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = ROOT / "schemas/conversion-job.schema.json"
DOC = ROOT / "docs/scanning-ocr-ingestion.md"
DIAGRAM = ROOT / "docs/mermaid/conversion_pipeline.mmd"
REPORT = ROOT / "target/conversion-reports/ocr-searchable-pdf-contract-smoke.json"


def fail(message: str) -> None:
    raise SystemExit(f"OCR searchable PDF contract smoke failed: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def validate_with_jsonschema(job: dict[str, object], schema: dict[str, object]) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception:
        return
    jsonschema.validate(job, schema)


def main() -> int:
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    jobs: list[dict[str, object]] = [
        {
            "job_id": "job-image-set-searchable-pdf-smoke",
            "input_kind": "image_set",
            "output_kind": "pdf",
            "input_uri": "fixtures/scanned/image-set-manifest.json",
            "output_uri": "target/conversion/scanned-searchable.pdf",
            "preserve_metadata": False,
            "create_source_map": True,
            "provider_hint": "ocr_provider_contract",
            "options": {
                "mode": "searchable_pdf",
                "require_word_confidence": True,
                "require_bounding_boxes": True,
                "ocr_text_provenance": "ocr_derived",
                "native_text_policy": "preserve_and_label",
            },
        },
        {
            "job_id": "job-pdf-ocr-layer-smoke",
            "input_kind": "pdf",
            "output_kind": "pdf",
            "input_uri": "fixtures/minimal/minimal.pdf",
            "output_uri": "target/conversion/minimal-ocr-layer.pdf",
            "preserve_metadata": True,
            "create_source_map": True,
            "provider_hint": "ocr_provider_contract",
            "options": {
                "mode": "ocr_text_layer",
                "require_word_confidence": True,
                "require_bounding_boxes": True,
                "ocr_text_provenance": "ocr_derived",
                "native_text_policy": "do_not_overwrite_native_text",
            },
        },
    ]

    supported_pairs = {("image_set", "pdf"), ("pdf", "pdf")}
    for job in jobs:
        validate_with_jsonschema(job, schema)
        pair = (str(job["input_kind"]), str(job["output_kind"]))
        require(pair in supported_pairs, f"unsupported OCR pair: {pair}")
        options = job["options"]
        require(isinstance(options, dict), f"{job['job_id']} options must be an object")
        for token in ["require_word_confidence", "require_bounding_boxes", "ocr_text_provenance"]:
            require(options.get(token), f"{job['job_id']} missing OCR option: {token}")
        require(
            options.get("native_text_policy") in {"preserve_and_label", "do_not_overwrite_native_text"},
            f"{job['job_id']} has unsafe native text policy",
        )

    doc = DOC.read_text(encoding="utf-8")
    for token in [
        "OCR results must carry confidence and bounding boxes.",
        "OCR-derived text must be clearly distinguishable from native PDF text.",
        "source-map evidence",
        "provider contract",
    ]:
        require(token in doc, f"OCR doc missing token: {token}")

    diagram = DIAGRAM.read_text(encoding="utf-8")
    for token in ["OCR Provider", "Searchable PDF", "OCR Text Layer"]:
        require(token in diagram, f"conversion diagram missing OCR token: {token}")

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(
        json.dumps(
            {
                "check": "ocr_searchable_pdf_contract",
                "status": "pass",
                "jobs": jobs,
                "policy": "provider contract only; execution requires capability discovery and explicit conversion approval",
                "core_dependency_boundary": "no OCR engine dependency in fe_reader_core",
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    print(f"OCR searchable PDF contract smoke: {REPORT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
