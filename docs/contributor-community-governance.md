# Contributor, Community and Governance Plan

## Purpose

Fe Reader is intentionally broad. Without contribution governance, the project will sprawl.

## Required files

```text
CONTRIBUTING.md
CODE_OF_CONDUCT.md
SECURITY.md
GOVERNANCE.md
MAINTAINERS.md
docs/rfcs/README.md
.github/ISSUE_TEMPLATE/
.github/PULL_REQUEST_TEMPLATE.md
.github/dependabot.yml
.github/renovate.json
```

## DCO vs CLA decision

Default recommendation: use DCO first. It is lightweight and common for open-source infrastructure projects. Revisit CLA only if a foundation, corporate stewardship, dual licensing or app-store/legal constraints require it.

## Maintainer roles

```text
Core Engine Maintainer
Rendering Maintainer
Platform Maintainer
Security Maintainer
Workflow Pack Maintainer
Release Manager
Compatibility Corpus Maintainer
Documentation Maintainer
```

## RFC requirement

An RFC is required for:

- new public API surface
- new plugin capability
- new automation mutation tool
- core architecture change
- bundled dependency with significant licence/security impact
- new workflow pack that claims domain-specific compliance
- enabling a frontier feature by default

## Review evidence

- Pull requests should cite the relevant smoke report, fixture or contract check when changing docs or contributor guidance.
- Governance changes should note whether they alter `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `GOVERNANCE.md` or `MAINTAINERS.md`.
- Documentation-only edits should still preserve the contract boundary and avoid promising unsupported product behaviour.
