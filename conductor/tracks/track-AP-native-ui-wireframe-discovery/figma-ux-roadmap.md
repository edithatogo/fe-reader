# Figma UX Roadmap

Figma file: https://www.figma.com/design/LY0vDEVEjqd96xcjvcYqW1

## Role of the Figma Artifact

The Figma file is the design source of truth for the native macOS shell until a newer approved design artifact supersedes it. Track AQ must implement against the named shell regions and states. Track AR must validate keyboard, accessibility, resize and visual evidence against the same artifact.

## UX Development Framework

Use this loop for each shell region and state:

1. Discover: define user intent, current product boundary, and native macOS expectations.
2. Model: update Figma with the layout, state and command surface.
3. Specify: record tokens, behavior and acceptance criteria in AP.
4. Implement: build only through AQ using adapter boundaries.
5. Validate: verify via AR with screenshots, keyboard paths, accessibility labels and resize checks.
6. Reconcile: update Figma or document intentional deviations before closing the phase.

## Design Tokens and Rules

- Spacing: prefer 8-point increments; reserve larger gaps for region separation.
- Typography: use macOS system typography scale; avoid hero-scale text inside the app shell.
- Color: use semantic roles for window background, sidebar, surface, border, text, muted text, accent and danger.
- Shape: use modest radii; cards are for repeated items or tools, not every section.
- Icons: toolbar and command buttons should use familiar native symbols where possible.
- Density: prioritize scannable operational density over marketing-style whitespace.

## Figma to Implementation Traceability

### Main Shell

| Figma Area | State | AQ Task | AQ Impl Target | AR Task | AR Val Target |
|---|---|---|---|---|---|
| Main shell | All | AQ-1.1 | Window root, min 800x500, title "Fe Reader" | AR-1.1 | Screenshot at 1120x760 and 800x500 |
| Main shell | All | AQ-1.2 | Explicit three-pane shell (sidebar\|canvas\|inspector) | AR-1.2 | Dividers visible, sidebar 220pt, inspector 260pt |
| Main shell | All | AQ-1.3 | NSToolbar: leading + trailing groups | AR-1.3 | Toolbar icons, tooltips, correct groups |
| Main shell | All | AQ-1.4 | Status bar (24pt): filename/state, pages when open, size, local-first policy | AR-1.4 | Status bar visible with content |
| Title | Empty | AQ-1.5 | Window title "Fe Reader" | AR-1.5 | Title text verified |
| Title | Open | AQ-1.6 | Title "Fe Reader — filename.pdf" | AR-1.6 | Title updates on open |
| Layout | Resize | AQ-1.7 | Sidebar collapses icon-only at <800px | AR-1.7 | Screenshot at 799px |
| Layout | Resize | AQ-1.8 | Inspector hides at <600px, overlay via shortcut | AR-1.8 | Screenshot at 599px, overlay test |
| Layout | Resize | AQ-1.9 | Smooth resize, no snap | AR-1.9 | Visual inspect |

### Library Sidebar

| Figma Area | State | AQ Task | AQ Impl Target | AR Task | AR Val Target |
|---|---|---|---|---|---|
| Sidebar | Empty | AQ-2.1 | "No recent documents" placeholder | AR-2.1 | Screenshot empty |
| Sidebar | Open | AQ-2.2 | Recent: filename, path, timestamp | AR-2.2 | Screenshot with items |
| Sidebar | Open | AQ-2.3 | Click recent opens document | AR-2.3 | Functional click test |
| Sidebar | Open | AQ-2.4 | Context menu: Open, Pin, Copy Path, Finder, Remove | AR-2.4 | Right-click menu functional |
| Sidebar | All | AQ-2.5 | Arrow key navigation | AR-2.5 | Keyboard nav test |
| Sidebar | All | AQ-2.6 | VoiceOver labels | AR-2.6 | VO audit |

### Document Canvas

| Figma Area | State | AQ Task | AQ Impl Target | AR Task | AR Val Target |
|---|---|---|---|---|---|
| Canvas | Empty | AQ-3.1 | Drop zone: dashed border, 48pt SF Symbol, text | AR-3.1 | Screenshot empty drop zone |
| Canvas | Empty | AQ-3.2 | Click = NSOpenPanel, PDF filter | AR-3.2 | Click opens dialog |
| Canvas | Empty | AQ-3.3 | Drag PDF starts loading, non-PDF toast | AR-3.3 | Drag test pass/reject |
| Canvas | Empty | AQ-3.4 | Recent placeholder (grayed, max 5) | AR-3.4 | Screenshot with recent |
| Canvas | Loading | AQ-3.5 | Spinning NSProgressIndicator | AR-3.5 | Screenshot loading |
| Canvas | Loading | AQ-3.6 | Inspector: "Loading document..." | AR-3.6 | Status text verified |
| Canvas | Loading | AQ-3.7 | Cancel returns to empty | AR-3.7 | Functional test |
| Canvas | Open | AQ-3.8 | Page surface via NSViewRepresentable | AR-3.8 | Screenshot page visible |
| Canvas | Open | AQ-3.9 | Arrow key page nav | AR-3.9 | Nav test |
| Canvas | Open | AQ-3.10 | Zoom in/out, fit width, actual size | AR-3.10 | Zoom commands functional |
| Canvas | Error | AQ-3.11 | File not found + Browse | AR-3.11 | Screenshot error |
| Canvas | Error | AQ-3.12 | Unsupported format | AR-3.12 | Screenshot error |
| Canvas | Error | AQ-3.13 | Corrupt doc + recovery options | AR-3.13 | Screenshot error |
| Canvas | Error | AQ-3.14 | Generic error + Retry + Report | AR-3.14 | Screenshot error |
| Canvas | Error | AQ-3.15 | Error in canvas, not modal | AR-3.15 | No modal for recoverable |
| Canvas | R/O | AQ-3.16 | "Mutation requires approval" banner | AR-3.16 | Banner visible |
| Canvas | R/O | AQ-3.17 | Patch plan displayed | AR-3.17 | Plan readable |
| Canvas | R/O | AQ-3.18 | Confirm gate disabled→enabled | AR-3.18 | Functional test |
| Canvas | R/O | AQ-3.19 | Apply mutation, audit receipt | AR-3.19 | Receipt visible |

### Inspector Panel

| Figma Area | State | AQ Task | AQ Impl Target | AR Task | AR Val Target |
|---|---|---|---|---|---|
| Inspector | Empty | AQ-4.1 | "No document open" | AR-4.1 | Screenshot empty |
| Inspector | Open | AQ-4.2 | Document info: path, pages, size, hash, dates | AR-4.2 | All fields populated |
| Inspector | Open | AQ-4.3 | Selection info (when selected in canvas) | AR-4.3 | Select content, info updates |
| Inspector | Open | AQ-4.4 | Actions: Export, Validate buttons | AR-4.4 | Buttons present |
| Inspector | Open | AQ-4.5 | Validation badge (color coded) | AR-4.5 | Correct badge color |
| Inspector | Open | AQ-4.6 | Expandable sections (XMP metadata) | AR-4.6 | Expand/collapse works |
| Inspector | R/O | AQ-4.7 | Read-only badge when mutation pending | AR-4.7 | Badge visible |
| Inspector | R/O | AQ-4.8 | Operation intent + patch plan | AR-4.8 | Plan fields visible |
| Inspector | All | AQ-4.9 | Tab order vertical, arrows in groups | AR-4.9 | Keyboard nav test |
| Inspector | All | AQ-4.10 | VoiceOver labels on all fields | AR-4.10 | VO audit |

### Command Surface

| Figma Area | State | AQ Task | AQ Impl Target | AR Task | AR Val Target |
|---|---|---|---|---|---|
| Commands | All | AQ-5.1 | Cmd+O opens file dialog | AR-5.1 | Shortcut functional |
| Commands | All | AQ-5.2 | Cmd+Shift+O recent popover | AR-5.2 | Shortcut functional |
| Commands | Open | AQ-5.3 | Cmd+W close doc | AR-5.3 | Functional test |
| Commands | Open | AQ-5.4 | Cmd+I focus inspector | AR-5.4 | Shortcut functional |
| Commands | Open | AQ-5.5 | Cmd+E export dialog | AR-5.5 | Shortcut functional |
| Commands | Open | AQ-5.6 | Cmd+Shift+V validate | AR-5.6 | Shortcut functional |
| Commands | Open | AQ-5.7 | Cmd+Shift+L toggle sidebar | AR-5.7 | Shortcut functional |
| Commands | Open | AQ-5.8 | Cmd+Shift+I toggle inspector | AR-5.8 | Shortcut functional |
| Commands | Open | AQ-5.9 | Cmd+0 focus canvas | AR-5.9 | Shortcut functional |
| Commands | Open+Sel | AQ-5.10 | Cmd+Shift+R redact (gated) | AR-5.10 | Shortcut gated correctly |
| Commands | All | AQ-5.11 | Cmd+N new window | AR-5.11 | New window independent |
| Commands | All | AQ-5.12 | Toolbar buttons enabled/disabled per state | AR-5.12 | State correctness |
| Commands | All | AQ-5.13 | Menu bar for all toolbar actions | AR-5.13 | Menu bar complete |

### AP Phase Deliverables

| AP Artifact | Location |
|---|---|
| UX Brief | `docs/native-macos-ux-brief.md` |
| Design Token Guidance | `docs/native-macos-design-tokens.md` |
| Interaction Model | `docs/native-macos-interaction-model.md` |
| Handoff Notes | `docs/native-macos-handoff-notes.md` |
| Figma Wireframe | Figma file link above |

## Roadmap Dependencies

- AP completes the UX brief, Figma file and token guidance before AQ implementation is accepted.
- AQ implements the shell using the Figma regions as named milestones.
- AR verifies mature UX quality against the same Figma regions and records deviations.
- Track D remains the parent UI/native-shell track.
- Track Z and Track AB remain the accessibility, keyboard and UI E2E alignment tracks.
- Track C and Track E remain the native binding and platform-integration dependency tracks.

## Deviation Policy

Implementation may deviate from Figma only when the runtime proves a better native behavior, an adapter boundary requires it, or accessibility requires it. The deviation must be recorded here or in the phase notes before the phase closes.
