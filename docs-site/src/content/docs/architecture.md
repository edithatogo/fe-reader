---
title: Architecture
description: Fe Reader keeps document contracts pure and pushes integration details into adapters.
---

The non-negotiable boundary is:

```text
fe_reader_core
  pure document/workflow core
  no UI, no platform, no renderer, no AI, no MCP, no plugin runtime

adapters
  rendering, platform, app integrations, MCP, plugins, web, native shell
```

This keeps the core stable enough for CLI, native, web, automation, and plugin surfaces to share the same operation and policy model.

## Write path

Every write must pass through:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

Automation surfaces are read-only by default. Mutations require a document hash match, patch plan ID, policy evaluation, approval token or interactive confirmation, and an audit receipt.

## Adapter posture

Adapters may use platform, renderer, PDFium, MCP, plugin, or GPU dependencies. `fe_reader_core` must not. This allows Fe Reader to stay local-first while still supporting native integration and bleeding-edge adapter lanes under explicit policy gates.
