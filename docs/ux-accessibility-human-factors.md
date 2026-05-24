# UX, Accessibility & Human Factors Plan

Fe Reader should make advanced PDF operations safe and understandable. The product should feel fast and native, but it should also be operable by keyboard, screen reader, touch, stylus and automation.

## Product UX principles

1. **Progressive disclosure:** simple reader first; expert tools are discoverable but not noisy.
2. **Preview before mutation:** destructive or irreversible operations create a `PatchPlan` preview.
3. **Receipts for high-risk operations:** redaction, metadata scrub, optimisation, repair, signing and export emit receipts.
4. **Undo or explicit irreversibility:** operations either support undo/transaction replay or clearly state why they are one-way.
5. **Native where it matters:** recent files, file permissions, print/share dialogs, accessibility APIs and notifications use native integration.
6. **Keyboard parity:** every important UI operation needs a command palette action and shortcut option.
7. **No surprise network:** cloud, sync and diagnostics are off unless explicitly enabled.

## Accessibility targets

### Application accessibility

- Keyboard navigation for all reader, annotation, metadata and workflow panels.
- Visible focus indicators and high-contrast mode.
- Reduced-motion mode.
- Screen-reader labels for toolbar items, document outline, page thumbnails, annotation list and workflow queues.
- Logical tab order.
- Hit targets suitable for touch and stylus.
- Accessible errors and warnings, especially for redaction, repair and signing.

### Web/PWA accessibility

- Target WCAG 2.2 AA for Fe Reader Web/PWA and documentation.
- Use automated checks plus human keyboard/screen-reader smoke tests.
- Ensure command palette, annotation review and redaction review are operable without pointer input.

### PDF accessibility workflows

- Inspect PDF tags, reading order, alt text, table structure and language metadata.
- Explain why a PDF is not accessible before attempting repair.
- Provide accessibility report export.
- Later: tag repair assistant for simple documents.

## Platform accessibility interfaces

| Platform | Integration |
|---|---|
| Windows | UI Automation names/roles, keyboard accelerators, high contrast, Narrator smoke tests. |
| macOS | NSAccessibility/AX metadata through native shell, VoiceOver smoke tests. |
| Linux | AT-SPI-compatible accessible labels where toolkit permits, GNOME/KDE keyboard and screen-reader smoke tests. |
| Android | TalkBack labels, content descriptions, touch target sizing, reduced motion. |
| iOS/iPadOS | VoiceOver labels, Dynamic Type where native views are used, Pencil/touch alternatives. |
| Web | WCAG 2.2 AA, ARIA only where semantic HTML is insufficient, automated axe-like checks later. |

## UX deliverables

```text
contracts/rust/accessibility.rs
schemas/accessibility-audit.schema.json
scripts/accessibility_audit_smoke.py
templates/accessibility/checklists/*.md
```

## Milestones

| Wave | Work |
|---|---|
| 0 | Accessibility contract, command registry, keyboard policy. |
| 1 | Reader keyboard navigation and screen-reader labels for core UI. |
| 2 | Annotation/review panels accessible by keyboard. |
| 3 | Redaction/workflow review queues accessible and receipt copy reviewed. |
| 4 | Installer and update UX reviewed for accessibility. |
| 5 | Automation and web/PWA accessibility smoke tests. |
| 7+ | Formal accessibility certification pathway and external testing. |
