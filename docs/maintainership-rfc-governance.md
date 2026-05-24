# Maintainer, RFC and ADR Governance

## Why this belongs in the implementation package

A project this broad can fail by becoming a bag of features. Fe Reader needs a lightweight governance model from day one so architecture, plugin ABI, PDF mutation semantics, automation policy and public SDKs evolve coherently.

## Documents

```text
ADRs: small architecture decisions that are already made.
RFCs: proposed changes to public architecture, contracts or product direction.
Compatibility notes: public API/ABI/contract migration decisions.
Release notes: user-facing changes and known limitations.
Corpus notes: why a fixture exists and what behaviour it guards.
```

## Required ADRs at project start

```text
0001-headless-rust-core.md
0002-operation-intent-patch-plan-receipt.md
0003-rendering-adapter-boundary.md
0004-secure-redaction-full-rewrite.md
0005-platform-integration-boundary.md
0006-deterministic-before-ml.md
0007-workflow-packs-not-domain-specific-core.md
0008-fork-policy-upstream-first.md
0009-automation-read-only-by-default.md
0010-feature-flags-and-frontier-lanes.md
```

## RFC triggers

An RFC is required for:

- Changing the core mutation pipeline.
- Making a frontier feature default-on.
- Adding a new public automation write surface.
- Declaring a public API stable.
- Forking a major dependency for more than one release cycle.
- Adding networked/cloud behaviour.
- Shipping a feature that changes privacy or telemetry posture.

## Review groups

```text
Core PDF maintainers
Platform integration maintainers
Security/redaction reviewers
Performance reviewers
Accessibility/i18n reviewers
Release/distribution maintainers
Workflow-pack maintainers
```

See `schemas/adr.schema.json` and `scripts/adr_validate.py`.
