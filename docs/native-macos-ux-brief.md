# Native macOS Shell — UX Brief

## 1. Primary Users

| Persona | Description |
|---|---|
| **PDF Workflow Developer** | Developer or technical user who inspects, validates, and exports PDF documents locally. Works with metadata transparency, patch plans, and audit receipts. Prefers keyboard-driven interaction. |
| **Verification Operator** | User who needs to confirm document integrity, check redaction completeness, and validate outputs against known schemas. Cares about audit trails. |
| **Automation Engineer** | User who triggers read-only automation (AppleScript, MCP) against the shell but never bypasses the mutation pipeline. Needs clear state indicators. |

Non-goal: non-technical consumer PDF reader. Fe Reader is a *workflow platform*, not a casual viewer.

## 2. Top Workflows

### Open and Inspect (highest frequency)

1. Launch app → empty state with drop zone
2. Drag or Cmd+O → file browser
3. Document loads → inspector shows metadata (size, page count, hash)
4. Review metadata → validate document integrity
5. Close or open next

### Validate Document

1. Open document → Validate (Cmd+Shift+V)
2. Shell shows validation status in inspector panel
3. If errors → error state with actionable message
4. If pass → verified badge shown

### Export Document

1. Open document → Export (Cmd+E)
2. Present export options (format, scope)
3. Confirm → export proceeds through adapter boundary
4. Completion → receipt shown in inspector

### Redact Document (mutation-gated — tertiary)

1. Open document → select content → Redact (Cmd+Shift+R)
2. Mutation pipeline: OperationIntent → PatchPlan → Review → Policy → Apply → Verify → AuditReceipt
3. Read-only mode enforced until approval token granted
4. Confirmation required before apply
5. Audit receipt displayed after completion

## 3. Shell Layout

```
┌─────────────────────────────────────────────────────────────┐
│  Toolbar / Title Area    [Open] [Recent] [Inspect] [...]  │
├──────────┬──────────────────────────────────┬──────────────┤
│          │                                  │              │
│ Library  │    Document Canvas               │  Inspector   │
│ Sidebar  │    (page surface / empty zone    │  Panel       │
│          │     / loading / error)           │              │
│ Recent   │                                  │  • Metadata  │
│ Pinned   │                                  │  • Selection │
│          │                                  │  • Actions   │
│          │                                  │  • Status    │
│          │                                  │              │
├──────────┴──────────────────────────────────┴──────────────┤
│  Status Bar (document info, verification state, path)      │
└─────────────────────────────────────────────────────────────┘
```

Four named regions:

| Region | Role | Visibility |
|---|---|---|
| **Toolbar / Title Area** | Primary commands, window title, document filename | Always visible |
| **Library Sidebar** | Recent documents, pinned workspaces, verified outputs | Collapsible (<800px) |
| **Document Canvas** | Empty drop zone, page surface, loading/error states | Always present |
| **Inspector Panel** | Metadata, selection, actions, state | Hides at <600px |

## 4. Shell States

### 4.1 Empty State

Trigger: No document open, first launch, or after closing all documents.

Visual:
- Document canvas shows centered drop zone with dashed border
- SF Symbol `doc.badge.plus` or `arrow.down.doc` (48pt)
- Text: "Open a PDF document to begin" (secondary text)
- Recent files placeholder below drop zone (grayed out, max 5)
- Toolbar: Open, Recent buttons active. Inspect, Export, Validate, Redact disabled.
- Sidebar shows empty state with "No recent documents"

Interaction:
- Click drop zone → opens file browser
- Drag PDF onto drop zone → loads document
- Cmd+O → opens file browser

### 4.2 Loading State

Trigger: File selected, document loading through adapter.

Visual:
- Document canvas shows indeterminate progress indicator (NSProgressIndicator.spinning)
- Inspector shows "Loading document..." in status area
- Toolbar commands disabled except Cancel
- Sidebar remains interactive

Interaction:
- Cancel loading → returns to empty state
- Loading progress communicated through adapter callback

Duration expectation: sub-second for most documents. For large documents (>100MB or complex), show elapsed time after 2 seconds.

### 4.3 Open Document State

Trigger: Document loaded and ready.

Visual:
- Document canvas shows page surface (first page or active page)
- Document filename in window title and toolbar
- Inspector shows: filename, path, page count, file size, hash, creation/modification dates, validation status
- Sidebar shows document in recent list with timestamp
- Toolbar: Open, Recent, Inspect, Export, Validate active. Redact conditionally active.

Interaction:
- Page navigation: scroll/arrow keys
- Inspector tabs: Document info, Selection, Actions
- Sidebar: click recent item to switch documents
- Keyboard shortcuts active for all commands


### 4.4 Error State

Trigger: File open failure, validation failure, adapter error.

Variants:

| Error Type | Visual | Action |
|---|---|---|
| **File not found** | Canvas: warning symbol + "File could not be found" | Offer "Browse" button |
| **Unsupported format** | Canvas: alert symbol + "Unsupported file format" | Offer "Open anyway?" if recoverable |
| **Corrupt document** | Canvas: error symbol + "Document appears corrupt" | Show recovery options via safe-open |
| **Validation failure** | Inspector: red badge + "Validation failed: [reason]" | Show detailed error in inspector |
| **Core adapter error** | Canvas: error symbol + generic error + code | "Retry" and "Report" buttons |

All error states: toolbar commands remain available for opening new files. Error is displayed **in** the canvas region, not as a modal dialog (except for truly blocking conditions).

### 4.5 Read-Only / Confirmation State

Trigger: Redaction or any mutation operation initiated.

Visual:
- Inspector shows read-only badge
- Canvas shows "Mutation requires approval" banner at top
- Operation intent displayed in inspector panel with patch plan details
- Confirmation button disabled until policy evaluation passes

Interaction:
- User reviews patch plan in inspector
- Policy evaluation runs automatically when plan is generated
- If approved: Confirm button becomes active
- On confirm: Apply → Verify → AuditReceipt flow
- Audit receipt displayed in inspector status area after completion

## 5. Command Hierarchy

### Primary (always active in open-document state)

| Command | Shortcut | Surface |
|---|---|---|
| Open | Cmd+O | Toolbar button, menu bar, drop zone |
| Open Recent | Cmd+Shift+O | Sidebar, menu bar |
| Inspect | Cmd+I | Toolbar button, menu bar, inspector |

### Secondary

| Command | Shortcut | Surface |
|---|---|---|
| Export | Cmd+E | Toolbar button, menu bar |
| Validate | Cmd+Shift+V | Toolbar button, menu bar, inspector |

### Tertiary (mutation-gated)

| Command | Shortcut | Surface |
|---|---|---|
| Redact | Cmd+Shift+R | Toolbar button (disabled by default), menu bar |

Only enabled when:
1. Document is open
2. Selection exists
3. Policy evaluation passes
4. User confirms via approval dialog


### Utility

| Command | Shortcut | Surface |
|---|---|---|
| Close Document | Cmd+W | Window, menu bar |
| Toggle Sidebar | Cmd+Shift+L | Menu bar |
| Toggle Inspector | Cmd+Shift+I | Menu bar |
| Focus Canvas | Cmd+0 | Menu bar |
| Zoom In/Out | Cmd++ / Cmd+- | Menu bar |
| Fit to Width | Cmd+9 | Menu bar |
| Actual Size | Cmd+8 | Menu bar |

## 6. Explicit Anti-Goals

The following are **not** part of this shell and must not appear in the design or be implemented:

- ❌ **Cloud sync or cloud storage integration**. No iCloud, Dropbox, Google Drive, or any network-sync feature.
- ❌ **Analytics or telemetry**. No usage tracking, crash reporting, or metrics upload.
- ❌ **Store publishing**. No App Store distribution; Developer ID-signed builds only at release.
- ❌ **Plugin runtime**. No plugin system, extension API, or third-party code execution in the shell.
- ❌ **AI/ML features**. No local LLM, RAG, or ML features in the shell. (Future tracks may evaluate, but not in this shell.)
- ❌ **Non-PDF document editing**. Creating new documents from scratch is out of scope.
- ❌ **Non-macOS platforms**. This shell is macOS-only. Windows, Linux, Android, and iOS are separate tracks.
- ❌ **Marketing-style landing pages**. No hero animations, onboarding carousels, or decorative illustrations.
- ❌ **Browser-based rendering**. No WebKit/WebView for document rendering. Native rendering via adapter only.

## 7. Accessibility Baseline

- All regions must have `accessibilityLabel` and `accessibilityRole` set.
- Sidebar items must be keyboard-navigable with arrow keys.
- Focus must be programmatically settable on the document canvas.
- All toolbar actions must be reachable via menu bar and keyboard.
- Error states must expose `accessibilityNotification` for screen readers.
- Dynamic type must resize text without clipping.
- No elements smaller than 44×44pt for interactive targets.

## 8. Privacy Preserving Design

- File paths are never displayed in full in the title bar (last two path components only).
- Recent files list is stored locally, never synced.
- No document content is sent over the network.
- Validation is performed locally via core adapter contracts.
- Audit receipts are stored locally and never transmitted.
