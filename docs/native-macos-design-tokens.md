# Native macOS Shell — Design Token Guidance

## 1. Spacing

Use an **8-point grid** for all layout values. No fractional point values.

### Region Separation

| Token | Value | Usage |
|---|---|---|
| `spacing-region` | 32pt | Between major layout regions (sidebar ↔ canvas, canvas ↔ inspector) |
| `spacing-section` | 24pt | Between sections within a panel (e.g., metadata vs. actions) |
| `spacing-group` | 16pt | Between grouped controls, sidebar items |
| `spacing-item` | 8pt | Between related inline elements (label + value, icon + text) |
| `spacing-inset` | 16pt | Padding inside cards, panels, and surface containers |

### Specific Region Insets

| Region | Horizontal Padding | Vertical Padding |
|---|---|---|
| Sidebar (internal) | 12pt | 8pt |
| Document Canvas | 24pt | 24pt |
| Inspector Panel | 16pt | 16pt |
| Toolbar | 8pt | 8pt |

## 2. Typography

Use **macOS system font** (`SF Pro` / `SF Mono`) throughout. No custom fonts.

### Dynamic Type Scale

| Token | NSFont.TextStyle | Point Size | Weight | Usage |
|---|---|---|---|---|
| `text-hero` | `.largeTitle` | 34pt | Bold | App title (empty state only) |
| `text-title` | `.title1` | 28pt | Semibold | Section headings, document title |
| `text-headline` | `.headline` | 15pt | Semibold | Card titles, field labels |
| `text-body` | `.body` | 13pt | Regular | Primary content, metadata values |
| `text-callout` | `.callout` | 12pt | Regular | Sidebar items, file names |
| `text-caption` | `.caption1` | 11pt | Regular | Status text, timestamps, hash previews |
| `text-mono` | `.body` (SF Mono) | 13pt | Regular | Code values, hashes, identifiers |

### Rules

- Use `NSFont.preferredFont(forTextStyle:)` and `font(_:textStyle:)` for dynamic type.
- Never hardcode point sizes where system text styles can be used.
- Mono font only for hash values, document identifiers, and code output.
- Keep hero-scale text (34pt) limited to the empty state app title.


## 3. Color Roles

All colors defined semantically. No hardcoded hex values outside of this table.

### macOS Light Mode

| Token | System Color | NSColor | Usage |
|---|---|---|---|
| `windowBackground` | `.windowBackground` | `NSColor.windowBackgroundColor` | Main window, canvas background |
| `sidebarBackground` | — | `NSColor.controlBackgroundColor` | Sidebar background |
| `surfaceBackground` | `.quaternary.opacity(0.3)` | `NSColor.quaternaryLabelColor` with alpha | Cards, grouped sections |
| `border` | `.quaternary.opacity(0.6)` | `NSColor.separatorColor` | Borders between regions, card borders |
| `primaryText` | `.primary` | `NSColor.labelColor` | Primary content |
| `secondaryText` | `.secondary` | `NSColor.secondaryLabelColor` | Descriptions, metadata labels |
| `tertiaryText` | `.tertiary` | `NSColor.tertiaryLabelColor` | Placeholders, disabled content |
| `accent` | `.accentColor` | `NSColor.controlAccentColor` | Selected items, interactive highlights |
| `danger` | `.red` | `NSColor.systemRed` | Errors, validation failures |
| `success` | `.green` | `NSColor.systemGreen` | Validation passes, verified badges |
| `warning` | `.orange` | `NSColor.systemOrange` | Warning states, advisory info |

### macOS Dark Mode

All tokens map to the same semantic `NSColor` values, which automatically adapt to dark mode. No custom dark-mode overrides.

### Implementation

In SwiftUI:
```swift
.background(Color(nsColor: .windowBackgroundColor))
.foregroundColor(.primary)
```

In AppKit:
```swift
view.layer?.backgroundColor = NSColor.controlBackgroundColor.cgColor
textField.textColor = .labelColor
```


## 4. Shape & Radius

### Corner Radii

| Token | Value | Usage |
|---|---|---|
| `radius-sm` | 6pt | Small indicators, status badges |
| `radius-md` | 8pt | Cards, grouped containers, drop zone |
| `radius-lg` | 12pt | Large containers, panel backgrounds |
| `radius-full` | Continuous capsule | Chips, pills, tags |

### Rules

- Use `.continuous` corner style (SwiftUI) or `cornerCurve = .continuous` (AppKit layer).
- Cards should appear structured, not decorative. Avoid drop shadows.
- Drop zone border: 2pt dashed border with `border` color, `.continuous` radius.

## 5. Icons

### Guidance

- Use **SF Symbols** for all icons. No custom icon artwork.
- Prefer familiar system symbols: `doc`, `folder`, `clock`, `magnifyingglass`, `square.and.arrow.up`, `checkmark.seal`, `eye`.
- Toolbar icon size: 16×16pt (standard).
- Sidebar icon size: 16×16pt.
- Empty state icon: 48×48pt.
- Status icons (badges): 12×12pt.
- Use `.symbolRenderingMode(.hierarchical)` or `.monochrome` (preferred for toolbar).
- Use `.foregroundStyle(.secondary)` for inactive toolbar items.

### Approved Toolbar Icons

| Action | SF Symbol |
|---|---|
| Open | `doc.badge.plus` or `folder` |
| Open Recent | `clock.arrow.circlepath` |
| Inspect | `magnifyingglass` |
| Export | `square.and.arrow.up` |
| Validate | `checkmark.seal` |
| Redact | `eye.slash` (disabled state: crossed out) |
| Close | `xmark` |

## 6. Density

### Principles

- **Operational density**: Show information without excessive whitespace. Users scan documents, not UI.
- **No marketing whitespace**: 32pt is the maximum region gap. No hero padding.
- **Compact inspector**: Metadata labels and values should fit on one line where possible.
- **Sidebar density**: Items have 8pt vertical padding, 12pt horizontal padding. No avatar-sized row heights.

### Target Densities

| Region | Target | Minimum |
|---|---|---|
| Sidebar item height | 28pt | 24pt |
| Toolbar button area | 30×30pt | 24×24pt |
| Inspector row height | 20pt | 18pt |
| Status bar height | 22pt | 20pt |
| Card minimum height | 80pt | 64pt |

## 7. Component Anatomy

### Toolbar Buttons

- Icon-only by default. `callout` tooltip on hover.
- Grouped in leading (document actions) and trailing (view options) positions.
- Disabled state: `tertiaryText` color, no interaction.
- Toggle state (sidebar/inspector): `accent` color when active.
- Hit target: 30×30pt minimum.

### Sidebar Items

- 28pt height, `callout` text size.
- Selected state: `accent` background (system highlight).
- Drag-to-reorder for pinned items (future).
- Context menu: "Pin", "Remove from Recent", "Copy Path".

### Cards

- Used for: recent document card, verified output card, status summary.
- Background: `surfaceBackground` with `radius-md`.
- Border: `border` color, 0.5pt width.
- Single column in sidebar, grid in canvas area.
- 16pt inset from all edges.

### Inspector Fields

- Group headers: `text-headline`, `secondaryText`.
- Key-value rows: `text-body` for both, key in `secondaryText`, value in `primaryText`.
- Tab order: vertical. Arrow keys within groups.
- Expandable sections for long metadata (e.g., XMP metadata).

### Status Bar

- 22pt height, `callout` text size.
- Shows: filename (last 2 path components), page count, validation status, file size.
- Badge colors: `success`, `danger`, `warning`, or `tertiaryText` (unvalidated).

## 8. Window Metrics

| Property | Value |
|---|---|
| Default window size | 1120×760pt |
| Minimum window size | 800×500pt |
| Sidebar default width | 240pt |
| Sidebar minimum width | 180pt |
| Inspector default width | 280pt |
| Inspector minimum width | 220pt |

## 9. Elevation / Shadows

- **No drop shadows** on cards or panels.
- Use background color differentiation (e.g., `surfaceBackground`) rather than shadow for depth.
- The only exception: the drag-drop zone in empty state may use a subtle shadow when hovered.

## 10. Motion & Animation

- Keep animations minimal and utilitarian.
- Sidebar collapse: 0.2s ease-in-out.
- Inspector show/hide: 0.2s ease-in-out.
- State transitions (empty → loading → open): no fade, instant swap.
- Loading indicator: use native NSProgressIndicator spinning style.
- No parallax, spring animations, or ornamental transitions.


