# Native macOS Shell — Handoff Notes for Track AQ (Implementation) and Track AR (Validation)

## 1. Handoff Summary

This document captures the approved design decisions from Track AP (Native UI Wireframe Discovery) that must flow into Track AQ (Implementation) and Track AR (Validation). These notes freeze the UX contract for the AP phase.

### Documents Produced

| Document | Status |
|---|---|
| Figma wireframe | Approved for first-pass implementation (Track AP Figma file) |
| UX Brief (`docs/native-macos-ux-brief.md`) | Approved |
| Design Tokens (`docs/native-macos-design-tokens.md`) | Approved |
| Interaction Model (`docs/native-macos-interaction-model.md`) | Approved |
| Figma UX Roadmap (`conductor/tracks/.../figma-ux-roadmap.md`) | Updated |

## 2. Approved Shell Regions

| Region | Must Implement | Layout Rule |
|---|---|---|
| **Toolbar / Title Area** | NSToolbar or SwiftUI Toolbar with icon-only buttons, tooltips | Leading (document actions) + Trailing (view toggles) groups. Always visible. |
| **Library Sidebar** | List of recent documents, pinned items | 240pt default width, collapses to icon-only at <800px. Uses `listStyle(.sidebar)`. |
| **Document Canvas** | Drop zone (empty), page surface (open), progress (loading), error display | Always present, flexible width. Background `windowBackground`. |
| **Inspector Panel** | Metadata groups, selection info, action status, validation badges | 280pt default width, hides at <600px. Expandable key-value sections. |
| **Status Bar** | Document info, validation state, file size | 22pt height, `callout` text. Below canvas, above window edge. |

## 3. Approved Shell States

| State | Must Implement | Key Behaviors |
|---|---|---|
| **Empty** | Centered drop zone with dashed border, SF Symbol (48pt), instruction text | Click or drag to open. Recent files placeholder. Toolbar: Open/Recent active only. |
| **Loading** | Indeterminate progress indicator in canvas, "Loading..." in inspector | Cancel returns to empty. Sub-second expected; elapsed after 2s. |
| **Open Document** | Page surface in canvas, metadata in inspector, filename in title | Page nav via arrow keys. Inspector tabs: Document info, Selection, Actions. |
| **Error** | In-canvas error display, colored badge in inspector | No modals (except blocking). Toolbar commands remain available. |
| **Read-Only / Confirm** | Mutation approval banner, patch plan display, confirm gate | Policy eval before confirm. Audit receipt after apply. Cancel returns to open. |


## 4. Approved Command Surface

### Primary Commands (always active in open-document state)

| Command | Shortcut | Implementation Notes |
|---|---|---|
| Open | Cmd+O | NSOpenPanel, PDF filter. Drag-drop as alternative. |
| Open Recent | Cmd+Shift+O | Sidebar recent list, popover from toolbar. |
| Inspect | Cmd+I | Focus inspector panel, scroll to metadata. |

### Secondary Commands

| Command | Shortcut | Implementation Notes |
|---|---|---|
| Export | Cmd+E | Export dialog, format options via adapter. |
| Validate | Cmd+Shift+V | Run adapter validation, show result in inspector. |

### Tertiary (Mutation-Gated)

| Command | Shortcut | Implementation Notes |
|---|---|---|
| Redact | Cmd+Shift+R | Disabled by default. Enabled only when: document + selection + policy + confirmed. |

### Utility Commands

| Command | Shortcut | Notes |
|---|---|---|
| Toggle Sidebar | Cmd+Shift+L | Animated collapse/expand |
| Toggle Inspector | Cmd+Shift+I | Show/hide; overlay at <600px |
| Focus Canvas | Cmd+0 | Set first responder to canvas |
| Zoom In/Out | Cmd++ / Cmd+- | Canvas zoom level |
| Fit to Width | Cmd+9 | |
| Actual Size | Cmd+8 | 100% zoom |
| New Window | Cmd+N | Independent window instance |
| Close | Cmd+W | Close current document |

## 5. Layout Rules to Preserve

### Size Constraints

| Rule | Value |
|---|---|
| Default window size | 1120×760pt |
| Minimum window size | 800×500pt |

## 6. Implementation Order (Suggested)

### AQ Milestone 1: Shell Skeleton
- Window with minimum size constraint
- Three-column split layout (sidebar | canvas | inspector)
- Stub views for each region
- NSToolbar with placeholder buttons

### AQ Milestone 2: Empty State
- Drop zone with dashed border, drag detection
- Recent files placeholder
- Toolbar state: Open/Recent active, others disabled

### AQ Milestone 3: Open + Loading States
- NSOpenPanel integration (Cmd+O)
- Loading indicator
- Document metadata display in inspector
- Filename in window title

### AQ Milestone 4: Error + Validation States
- In-canvas error display for each error variant
- Validation button and status badge
- Recovery options in error states

### AQ Milestone 5: Resize + Keyboard
- Sidebar collapse at <800px
- Inspector hide at <600px
- All keyboard shortcuts wired
- Focus management

### AQ Milestone 6: Read-Only / Confirmation
- Mutation approval banner
- Patch plan display
- Policy eval stub
- Audit receipt display

## 7. Open Questions for Implementation Track

These could not be resolved during AP and must be addressed during AQ:

1. **Sidebar content model**: Flat recent + pinned list (UX brief) vs. current placeholder sections (Library, Recent, Contracts, Release). The placeholder sections should be replaced.

2. **Document canvas impl**: NSViewRepresentable (recommended for performance) vs. pure SwiftUI. Depends on rendering adapter API.

3. **Inspector tabs**: Accordion/expandable sections (recommended) vs. tab view. Either acceptable if scannable.

4. **Recent files persistence**: UserDefaults (recommended), NSDocumentController, or custom JSON file.

5. **Safe-open stub**: The safe-open contract is not yet in core adapter. Build error state UI with stub recovery.

6. **Validation adapter API**: Async binding interface not yet defined. AQ should define it.

7. **Mutation gating UX**: Policy evaluation UI (inline vs. dialog) needs refinement when core policy engine is available.

## 8. Deviation Policy Reference

From the Figma UX Roadmap:

> Implementation may deviate from Figma only when the runtime proves a better native behavior, an adapter boundary requires it, or accessibility requires it. The deviation must be recorded in the phase notes before the phase closes.

### Deviation Recording Template

```
**Deviation**: [Description of what changed]
**Rationale**: [Why the deviation is necessary]
**Figma reference**: [Frame/area affected]
**Approval**: [Who approved]
**Date**: [When deviation was recorded]
```

### Known Acceptable Deviations

See `docs/native-macos-figma-deviations.md`. The major accepted runtime deviation is the explicit three-pane SwiftUI shell, replacing `NavigationSplitView` after the unsigned/headless capture path showed reserved sidebar space without visible content.

## 9. AR Validation Checklist

- [x] Window default size captured at 1800×1144 pixels in verification artifact
- [x] Minimum window size enforced at 900×520pt for the first-pass three-pane shell
- [x] Sidebar visible and stable in the native preview artifact
- [x] Inspector can be hidden through toolbar/menu command
- [x] Empty: drop zone visible, Open active, document actions disabled
- [x] Loading: progress indicator state exists
- [x] Open: metadata adapter populates file size, page count, hash, author/date where available
- [x] Error: in-canvas display, no modal for recoverable errors
- [x] Read-only: mutating Redact action remains disabled until the mutation pipeline exists
- [x] Primary keyboard shortcuts wired for open, close, inspect, export, validate and inspector toggle
- [x] Toolbar buttons functional for available actions
- [x] Drag-to-open path wired for PDF file URLs
- [x] System typography and semantic colors used
- [x] Accessibility labels on primary shell regions and commands
- [x] Dark mode automatic adaptation verified in preview
- [x] No cloud sync, analytics, or telemetry in build
- [x] Deviations recorded in `docs/native-macos-figma-deviations.md`

Responsive sidebar collapse and inspector overlay behavior remain follow-up polish items for the AppKit split-view pass. They are not hidden as completed runtime behavior.
