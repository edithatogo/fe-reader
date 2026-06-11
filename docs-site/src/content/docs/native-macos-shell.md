---
title: Native macOS Shell
description: The macOS app is a SwiftUI-first native shell that keeps platform code outside the core.
---

The native macOS app is implemented as an adapter surface. It can present local document intake, recents, metadata, status, and command surfaces without adding UI, platform, or renderer dependencies to `fe_reader_core`.

## Implemented checkpoint

- SwiftUI-first three-region shell: library sidebar, document surface, and inspector.
- Local PDF intake through native open and drag-drop flows.
- Recent-document state held in the native shell layer.
- Metadata and status panels driven by adapter evidence.
- Menu and command wiring for open, inspect, validate, export, navigation, zoom, and panel toggles.
- Unsigned local development build path for macOS before certificate-backed packaging.

## Boundary rules

The native shell must not directly mutate PDF bytes. Any future mutating command still has to pass through the shared operation pipeline:

```text
OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt
```

Renderer, AppleScript, App Intents, file-provider, and packaging behavior remain adapter-owned. Core document contracts stay portable across CLI, web, Windows, Linux, Android, and iOS surfaces.

## Known unfinished work

- Real page rendering remains behind the rendering adapter roadmap.
- Certificate-backed signing, notarization, and distribution packaging are later release operations.
- Mutation approval UI is scaffoldable only when the relevant patch-plan, policy, verification, and audit receipt contracts are wired end to end.
