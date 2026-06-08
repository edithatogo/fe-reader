#!/usr/bin/env python3
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "target" / "release-evidence"
EVIDENCE.mkdir(parents=True, exist_ok=True)


def read(rel):
    return (ROOT / rel).read_text(encoding="utf-8")


def require(condition, message, failures):
    if not condition:
        failures.append(message)


def required_file(rel, failures):
    path = ROOT / rel
    require(path.exists(), f"missing {rel}", failures)
    return path


def require_tokens(rel, tokens, failures):
    text = read(rel)
    for token in tokens:
        require(token in text, f"{rel} missing Wave8 token {token}", failures)


def load_json(rel, failures):
    path = required_file(rel, failures)
    if path.exists():
        try:
            return json.loads(path.read_text(encoding="utf-8"))
        except Exception as exc:
            failures.append(f"{rel} is not valid JSON: {exc}")
    return {}


def check_guides(failures):
    require_tokens("docs/user-docs-training-community.md", ["User guide", "Admin guide", "Developer guide", "Training packs"], failures)
    require_tokens("docs/enterprise-deployment-policy.md", ["Enterprise", "policy", "Windows", "macOS", "Linux"], failures)
    require_tokens("docs/developer-ecosystem-sdk.md", ["SDK", "MCP", "Plugin SDK", "OperationIntent -> PatchPlan"], failures)


def check_examples(failures):
    for rel in [
        "examples/README.md",
        "examples/cli/inspect-minimal.sh",
        "examples/workflows/legal-deidentify.plan.json",
        "examples/source-linked-projects/typst-provider-capability.json",
        "examples/review-packets/local-review-packet.json",
        "examples/plugins/read-only-plugin-manifest.json",
    ]:
        required_file(rel, failures)
    require_tokens("examples/README.md", ["cli/", "workflows/", "source-linked-projects/", "review-packets/", "plugins/"], failures)
    require("OperationIntent -> PatchPlan" in read("examples/workflows/legal-deidentify.plan.json"), "workflow example must cite mutation pipeline", failures)
    source = load_json("examples/source-linked-projects/typst-provider-capability.json", failures)
    discovery = source.get("provider_capability_discovery", {})
    require(discovery.get("required_before_execution") is True, "source-linked example must require provider capability discovery", failures)
    plugin = load_json("examples/plugins/read-only-plugin-manifest.json", failures)
    require(plugin.get("may_apply_patch_plan") is False, "plugin example must be proposal-only", failures)


def check_accessibility_and_privacy(failures):
    require_tokens("docs/ux-accessibility-human-factors.md", ["Keyboard navigation", "Screen-reader", "WCAG 2.2 AA"], failures)
    required_file("templates/accessibility/checklists/app-accessibility-smoke.md", failures)
    required_file("target/release-evidence/wave7-release-hardening.json", failures)
    require_tokens("docs/privacy-diagnostics-observability.md", ["support bundle", "preview", "redaction", "local by default"], failures)
    require_tokens("docs/privacy-preserving-quality-signals.md", ["support bundle", "local-first", "privacy"], failures)


def check_cache_optimization_docs(failures):
    require_tokens("docs/content-addressed-cache-workspace.md", ["privacy-sensitive", "not authoritative", "source document hash"], failures)
    require_tokens("docs/pdf-optimization-linearization-compression.md", ["receipt", "signature", "safe"], failures)
    required_file("templates/optimization/safe-rewrite.json", failures)


def check_community_templates(failures):
    for rel in [
        "CONTRIBUTING.md",
        "CODE_OF_CONDUCT.md",
        "SECURITY.md",
        "GOVERNANCE.md",
        "MAINTAINERS.md",
        "docs/rfcs/README.md",
        ".github/PULL_REQUEST_TEMPLATE.md",
        ".github/ISSUE_TEMPLATE/bug_report.md",
        ".github/ISSUE_TEMPLATE/feature_request.md",
        ".github/ISSUE_TEMPLATE/performance_regression.md",
        ".github/ISSUE_TEMPLATE/corpus_fixture.md",
        ".github/ISSUE_TEMPLATE/accessibility_issue.md",
    ]:
        required_file(rel, failures)
    require_tokens(".github/PULL_REQUEST_TEMPLATE.md", ["Contracts and Safety", "Evidence"], failures)


def main():
    failures = []
    try:
        check_guides(failures)
        check_examples(failures)
        check_accessibility_and_privacy(failures)
        check_cache_optimization_docs(failures)
        check_community_templates(failures)
    except Exception as exc:
        failures.append(str(exc))

    report = {
        "check": "wave8_adoption_ecosystem",
        "status": "fail" if failures else "pass",
        "criteria": [
            "user admin developer guides",
            "cli workflow source-linked review-packet plugin examples",
            "accessibility release readiness",
            "support bundle privacy review",
            "source-linked provider capability discovery",
            "optimization and cache risk documentation",
            "community templates",
        ],
        "failures": failures,
    }
    (EVIDENCE / "wave8-adoption-ecosystem.json").write_text(json.dumps(report, sort_keys=True) + "\n", encoding="utf-8")
    if failures:
        for failure in failures:
            print(f"wave8 adoption ecosystem failure: {failure}", file=sys.stderr)
        raise SystemExit(1)
    print("wave8 adoption ecosystem smoke: ok")


if __name__ == "__main__":
    main()
