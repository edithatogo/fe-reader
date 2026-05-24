# IP, Brand and Originality Guardrails

Fe Reader should target broad industry capability parity while remaining an original local-first workflow platform.

This document is not legal advice. It is a product and engineering checklist to avoid unnecessary copying risk.

## Product language

Use:

- local-first PDF workflow platform;
- verified document operations;
- workflow packs;
- document health;
- safe automation;
- metadata transparency;
- privacy-preserving diagnostics.

Avoid:

- vendor-clone language in public docs;
- UI copying/trade-dress imitation;
- proprietary feature names;
- screenshots or interaction flows that imitate one vendor's exact layout.

## Engineering rules

- Reimplement behaviours from public standards and user needs, not from proprietary internals.
- Use neutral capability names: `redaction verification`, `preflight`, `document health`, `workflow receipt`.
- Prefer public standards: ISO 32000/PDF 2.0, PDF/A, PDF/UA, PDF/X, XMP, C2PA, platform APIs.
- Maintain an ADR for any feature inspired by a commercial baseline and explain the standards/user-need basis.

## Review gate

Before public beta, conduct:

- trademark/name check;
- icon and UI originality review;
- patent-risk triage for unusual workflows;
- dependency license review;
- export-control/crypto distribution review where applicable.
