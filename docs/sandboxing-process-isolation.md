# Sandboxing and Process Isolation Plan

## Phase 1: architectural separation

- Keep `fe_reader_core` free of rendering, UI, platform and plugin runtime dependencies.
- Keep renderers behind `RenderBackend`.
- Keep converters behind `ConversionProvider`.
- Keep plugins behind `PluginHost` and proposal-only APIs.

## Phase 2: helper process model

Introduce `fe-reader-renderd` and optionally `fe-reader-convertd`:

```text
App/CLI
  -> bounded IPC request
  -> helper process
  -> render/conversion result
```

The helper receives file grants or file bytes according to policy. It cannot write the source document.

## Platform isolation targets

| Platform | Isolation approach |
|---|---|
| Windows | Job Objects, restricted token, AppContainer/MSIX where practical |
| macOS | App Sandbox, security-scoped bookmarks, hardened runtime, helper entitlements |
| Linux | Flatpak portals, seccomp/bubblewrap for helper processes where practical |
| Android | SAF URI grants, no broad storage permission, isolated app sandbox |
| iOS | document browser/security-scoped access, app sandbox |
| Web | browser origin sandbox, WASM memory isolation, explicit file grants |

## Filesystem rules

- Rendering helpers may read documents and write cache files only.
- Mutation engine writes only to explicit output handles or user-approved replacement paths.
- Conversion providers receive temporary working directories and explicit output handles.
- Plugins never receive raw write handles.

## Failure policy

If a helper crashes, the document is not modified. UI reports a render/conversion failure and may restart the helper.
