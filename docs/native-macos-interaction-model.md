# Native macOS Shell — Interaction Model

## 1. Primary Navigation

### Split Layout Navigation

The shell uses an explicit three-pane SwiftUI layout for the first native pass:

1. **Sidebar (Library)** — narrow column, drives the main content view
2. **Canvas (Document)** — primary content area
3. **Inspector** — trailing detail panel

Navigation flow:

```
Sidebar selection → Canvas updates → Inspector reflects selection
```

- Sidebar selection determines what the canvas shows (recent list, open document, etc.)
- The inspector always reflects the currently active item in the canvas.
- There is no master-detail within the sidebar itself for v1.

### Command-Driven Navigation

Actions bypass the sidebar hierarchy when triggered by keyboard or toolbar:

```
Cmd+O → File open dialog → Canvas shows document → Inspector shows metadata
Cmd+I → Focus inspector panel
Cmd+Shift+L → Toggle sidebar visibility
```

Commands may repurpose the focus but never change the sidebar selection unilaterally.

## 2. Command Paths

### Menu Bar

Standard macOS menu bar structure:

```
Fe Reader         File              Edit              View              Window
  About             Open (Cmd+O)      Cut               Toggle Sidebar    Minimize
  Quit              Open Recent       Copy              Toggle Inspector  Zoom
                    Close (Cmd+W)     Paste             Focus Canvas
                    Export (Cmd+E)    Select All        Zoom In/Out
                    Validate           (Cmd+A)          Fit to Width
                     (Cmd+Shift+V)                      Actual Size
                    Redact
                     (Cmd+Shift+R)
```

### Toolbar

- **Leading group**: Open, Open Recent (popover), Inspect, Export
- **Trailing group**: Validate badge, Redact (conditional), Sidebar toggle, Inspector toggle
- Toolbar buttons are icon-only with tooltips. No persistent labels.


### Keyboard Entry Points

| Action | Shortcut | Context |
|---|---|---|
| Open file | Cmd+O | Global, any state |
| Open recent | Cmd+Shift+O | Global, any state |
| Close document | Cmd+W | Open-document state |
| Inspect | Cmd+I | Open-document state, toggles inspector focus |
| Export | Cmd+E | Open-document state |
| Validate | Cmd+Shift+V | Open-document state |
| Redact | Cmd+Shift+R | Open-document + selection + policy approved |
| Toggle sidebar | Cmd+Shift+L | Global |
| Toggle inspector | Cmd+Shift+I | Global, only when sidebar width allows |
| Focus canvas | Cmd+0 | Open-document state |
| Zoom in | Cmd++ (Cmd+=) | Open-document state |
| Zoom out | Cmd+- | Open-document state |
| Fit to width | Cmd+9 | Open-document state |
| Actual size | Cmd+8 | Open-document state |
| New window | Cmd+N | Global |

### Arrow Key Navigation

| Context | Key | Action |
|---|---|---|
| Sidebar | Up/Down | Move selection between recent items |
| Canvas (document open) | Up/Down | Scroll page vertically |
| Canvas (document open) | Left/Right | Previous/next page |
| Inspector | Up/Down | Move between field groups |
| Inspector (expandable) | Left/Right | Collapse/expand section |

### Tab Order

1. Sidebar items → Canvas → Inspector fields → Status bar
2. Tab navigates Z-order (left to right, top to bottom).
3. Shift+Tab reverses.

## 3. Focus Expectations

| State | Initial Focus | Tab Order |
|---|---|---|
| Empty | Sidebar first item (if recent), else toolbar Open button | Sidebar → Toolbar → Status |
| Loading | Loading indicator (non-interactive) | Toolbar (Cancel) → Sidebar |
| Open Document | Document canvas (first responder for keyboard input) | Toolbar → Sidebar → Canvas → Inspector → Status |
| Error | Error label in canvas (programmatic VoiceOver focus) | Toolbar → Sidebar → Canvas → Status |
| Read-Only / Confirm | Confirmation dialog or banner | Dialog buttons → Canvas → Sidebar → Status |

- Canvas is first responder when a document is open.
- Sidebar retains selection across state transitions.
- Focus never lands on a disabled element.

## 4. Resize Behavior

### Breakpoints

| Window Width | Behavior |
|---|---|
| ≥1120pt | Full layout: sidebar + canvas + inspector |
| 800–1119pt | Sidebar collapses to icon-only at <800pt |
| 600–799pt | Inspector hides automatically. Canvas uses full width. |
| <600pt | Minimum window size: 800×500pt (inspector hidden, sidebar compact) |

```
1120pt        800pt        600pt
  │             │             │
  │ Full 3-col  │ 2-col       │ min width
  │             │ (sidebar    │ (enforced)
  │             │  collapses) │
  ▼             ▼             ▼
├────────┬──────────────┬────────┤
│ Side   │   Canvas     │ Insp.  │
│ bar    │              │        │
├────────┴──────────────┴────────┤
│         240pt→180pt (sidebar)  │
│         280pt→220pt (inspector)│
```

- Sidebar collapse: maintains icon-only labels at <800px.
- Inspector hide: disappears at <600px. Cmd+Shift+I shows as overlay.
- Resize is smooth. No snap-to-breakpoint.

### Split Divider

- Sidebar/Canvas: draggable, 180–360pt range.
- Canvas/Inspector: draggable, 220–400pt range.
- Standard macOS split-view appearance (1pt line).

## 5. State Transitions

### Empty → Loading → Open

```
User drops file / Cmd+O
        │
        ▼
  ┌───────────┐
  │  EMPTY    │ ← App launch, no recent files
  └─────┬─────┘
        │ File selected
        ▼
  ┌───────────┐
  │  LOADING  │ ← Progress indicator, commands disabled
  └─────┬─────┘
   ┌────┴────┐
   │         │
   ▼         ▼
  ┌──────┐ ┌───────┐
  │ OPEN │ │ ERROR │ ← Adapter error / corrupt file
  └──────┘ └───────┘
```

### Open → Error

- On validation failure: canvas remains open, inspector shows red badge.
- On file error: canvas shows error, document unloaded.

### Open → Read-Only

```
Open → Select → Initiate Redact
        │
        ▼
  ┌─────────────┐
  │ READ-ONLY   │ ← Mutation approval banner
  │ CONFIRM     │ ← Patch plan in inspector
  └──────┬──────┘
   ┌─────┴─────┐
   │           │
   ▼           ▼
  ┌──────┐  ┌────────┐
  │ OPEN │  │ OPEN + │ ← Audit receipt displayed
  │      │  │ RECEIPT│    Document updated
  └──────┘  └────────┘
```

- User can always cancel redaction.
- No auto-approval. Policy evaluation is explicit.

## 6. Drag and Drop

### Drop Zone (Empty State)

- Accepts: PDF files (`.pdf`), and any file the adapter reports as readable.
- Visual feedback: border becomes `accent` color, fill becomes elevated opacity.
- Rejection: non-PDF shows "Cannot open this file type" toast.

### Drag-to-Open (Any State)

- Dropping PDF onto canvas when document is open closes current and opens new.
- Confirmation dialog if current document has unsaved mutation state.

## 7. Context Menus

### Sidebar Item
```
Open | Pin/Unpin | Copy Path | Show in Finder | Remove from Recent
```

### Canvas (Open Document)
```
Zoom In | Zoom Out | Fit to Width | Actual Size | Copy | Select All | Inspect Selection
```

### Inspector Field
```
Copy Value | Copy Key | Show Full Value (for truncated text)
```

## 8. Keyboard Accessibility

- All toolbar actions have keyboard shortcuts (see section 2).
- Menu bar items discoverable via standard macOS keyboard navigation.
- No dead keys or chord conflicts with standard macOS shortcuts.
- Full Keyboard Access (Tab to all controls) supported for all interactive elements.

## 9. Window Management

- Single window by default. Cmd+N opens new window with empty state.
- Each window is independent: sidebar, document, inspector are per-window.
- Closing last window quits app (standard macOS behavior).
- Window title: "Fe Reader" when empty, "Fe Reader — filename.pdf" when document open.
