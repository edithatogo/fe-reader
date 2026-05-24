# Security Threat Model

## Goal

Fe Reader handles hostile files, private documents, automation calls and user secrets. Its safety posture must be designed before feature velocity.

## Primary assets

- Source PDF bytes and prior revisions.
- Redacted content and hidden metadata.
- Signing keys, certificate references and approval tokens.
- Local document catalog and search index.
- Workflow receipts and audit logs.
- Plugin manifests and trust database.
- Native file-system grants on mobile and sandboxed desktop apps.

## Trust boundaries

```text
Untrusted PDF bytes
  -> parser/extractor/render adapter
  -> immutable document model
  -> operation planner
  -> policy gate
  -> mutation engine
  -> verifier
  -> output file
```

Automation clients, plugins, browser extensions, web origins, external converters and PDF-embedded scripts are always untrusted.

## Threat classes

| Threat | Mitigation |
|---|---|
| Malicious PDF parser crash | fuzzing, process isolation for rendering, parser limits, memory/time budgets |
| Hidden content after redaction | full sanitising rewrite, object pruning, text re-extraction, image/OCR verification option |
| Automation abuse | read-only by default, policy gate, approval tokens, document hash binding |
| Plugin escape | WASM sandbox, no direct filesystem, no network by default, proposal-only operations |
| Prompt injection in document text | never treat PDF text as instructions; agents may cite content but cannot inherit permissions from content |
| Metadata leaks | clean-share metadata profile, embedded file inspection, XMP diffing |
| Supply-chain attack | cargo-deny, cargo-vet, SBOM, provenance, fork policy, dependency tiering |
| Update compromise | signed update manifest, artifact digests, platform code signing, staged channels |
| Search index privacy leak | local-only index, encrypted-at-rest option, per-document purge and workspace policy |

## Sandboxing strategy

- Core mutation engine remains in-process but deterministic and fuzzed.
- Rendering may move to a helper process once the trait boundary is stable.
- External converters run in a job sandbox with explicit input/output directories.
- Plugin runtime cannot access documents directly; it receives typed spans/metadata and returns proposed operations.
- Embedded PDF JavaScript, RichMedia and launch actions are recognised but disabled by default.

## Policy engine

All high-risk operations must evaluate `SecurityPolicy` before execution. Policy inputs include operation kind, source, document hash, file grant type, plugin/automation identity, workflow pack, and risk class.
