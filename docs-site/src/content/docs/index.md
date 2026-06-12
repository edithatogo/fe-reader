---
title: Fe Reader
description: Bleeding-edge local-first PDF workflow platform for private, verifiable document work.
---

Fe Reader is a local-first PDF workflow platform built around privacy, verification, metadata transparency, automation safety, and native integration.

The project is implementation-first: contracts compile, smoke tests run, and every capability claim must eventually be backed by fixtures, differential oracles, visual evidence, or a documented limitation.

## Current edge

- Pure `fe_reader_core` document and workflow contracts.
- Adapter crates for rendering, PDF parsing, automation, plugins, bindings, and platform integration.
- Operation mutation pipeline: `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
- Native macOS shell checkpoint with SwiftUI-first layout, local document intake, recent documents, metadata panels, status surfaces, and command wiring.
- Starlight docs site pages for release quality and release pipeline evidence artifacts.
- Release provenance scaffolding with SBOM status, signing readiness, and public-channel policy checks.
- Render backend scaffolding with validated tile requests, stable cache keys, PDFium adapter boundaries, and conservative GPU policy flags.
- Read-only transaction journal inspection and recovery listing for sidecars that need crash-recovery review.
- Read-only metadata diagnostics for document-info fields, trailer keys, XMP stream length, XMP SHA-256, safe previews, and non-fatal parser/decode errors.
- Expanded security-policy aliases for COM, AppleScript, D-Bus, Android intents, iOS App Intents, browser extensions, local APIs, plugins, external tools, and network access.

## What it is not

Fe Reader is not a clone of any single PDF vendor. It is a privacy-preserving, evidence-oriented PDF platform that aims to meet baseline PDF expectations while exposing the verification and safety machinery that professional workflows need.
