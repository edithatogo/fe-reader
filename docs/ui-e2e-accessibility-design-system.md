# UI E2E, Accessibility and Design System

## Purpose

Fe Reader must feel native and trustworthy on every platform. UI quality cannot be left until the end because PDF readers live or die on responsiveness, keyboard flow, selection precision and annotation ergonomics.

## Design system

Create a small design-token system rather than ad-hoc CSS:

```text
colour tokens
spacing tokens
typography tokens
focus ring tokens
hit target tokens
annotation state tokens
risk warning tokens
platform density tokens
```

## Accessibility baseline

Target WCAG 2.2 AA for the app UI where applicable. PDF accessibility inspection/repair is a separate document feature, but the Fe Reader UI itself must also be accessible.

Required tests:

- keyboard-only open/read/search/annotate flows
- visible focus for all controls
- screen-reader labels for page thumbnails and annotations
- high contrast/dark mode
- reduced motion
- touch target sizing
- zoom and text scaling
- modal/dialog focus trapping

## End-to-end testing

Use a two-lane strategy:

1. **Frontend contract tests**: fast browser tests against the UI state model.
2. **Real shell tests**: Tauri WebDriver/Selenium/WebdriverIO where supported, plus manual release acceptance scripts for gaps.

Do not rely exclusively on Chromium Playwright tests; the native webview differs across Windows, macOS, Linux, iOS and Android.

## Contracts

See:

```text
contracts/rust/accessibility_audit.rs
contracts/rust/e2e_test_harness.rs
schemas/accessibility-test-report.schema.json
```
