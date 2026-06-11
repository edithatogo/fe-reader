# Figma Deviation Notes — Native macOS Shell

Reference Figma file: https://www.figma.com/design/LY0vDEVEjqd96xcjvcYqW1

## Process

Implementation may deviate from the Figma wireframe only when runtime proves a better native behavior, an adapter boundary requires it, or accessibility requires it. Each deviation must be recorded here with rationale before the phase closes.

## Recorded Deviations

| # | Area | Figma Reference | Implementation | Rationale | Status |
|---|---|---|---|---|---|
| 1 | Sidebar width | 240pt (from AP figma) | Fixed 220pt first-pass shell pane | Explicit pane layout renders reliably in unsigned/headless verification and avoids blank sidebar captures observed with NavigationSplitView. | Accepted |
| 2 | Inspector toggle | Dedicated toolbar button + sidebar toggle | Toolbar "sidebar.trailing" icon + ToolbarTitleMenu toggle | Standard macOS inspector toggling pattern uses toolbar button. Avoids duplicating the sidebar split toggle. | Accepted |
| 3 | Minimum window size | 800×600 (from AP interaction model) | 900×520 (SwiftUI frame min) | Three-pane shell needs ~900pt width for readable sidebar, canvas and inspector. Reduced min height to 520 to match standard content height. | Accepted |
| 4 | Redact action | Inline active button | Disabled button with "mutation pipeline not yet available" label | Mutation pipeline requires OperationIntent → PatchPlan → Review → Apply → Verify → AuditReceipt flow. Not shortcutting this. | Required by product architecture |
| 5 | Escape/Return in error state | Not specified | Dismiss bound to Escape key, Try Again bound to Return key | Standard macOS dialog behavior pattern added for user convenience. | Accepted |
| 6 | Verification screenshot | Marker-file based capture in build script | NSView bitmapImageRepForCachingDisplay triggered from SwiftUI onAppear | SwiftUI requires NSApplicationDelegateAdaptor for screenshot capture. Minimal interop. | Accepted |
| 7 | Split-view implementation | Flexible NavigationSplitView | Explicit HStack panes with dividers | NavigationSplitView rendered reserved sidebar space without visible content in the unsigned preview capture. Explicit panes are more deterministic until a later AppKit-backed split view pass. | Accepted |
| 8 | Metadata fields | Figma showed placeholder document info | Shell extracts file size, page count, PDF author/date and local fingerprint via a native adapter | This gives the inspector and status bar real local evidence while keeping fe_reader_core free of UI/platform dependencies. | Accepted |

## Open Questions

| # | Question | Status |
|---|---|---|
| 1 | Should the Figma wireframe be updated to show the explicit 220pt first-pass sidebar? | Pending — Figma update deferred to next design pass |
| 2 | Should the Open Recent menu use NSDocumentController or custom UserDefaults storage? | Custom UserDefaults storage chosen for now; NSDocumentController path is documented in architecture |
| 3 | Should loading state show a determinate or indeterminate progress indicator? | Indeterminate ProgressView used; can switch to determinate when core provides progress reporting |
