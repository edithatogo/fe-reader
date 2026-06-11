---
title: Automation Safety
description: Automation is read-only first and mutates only through reviewed patch plans.
---

COM, AppleScript, D-Bus, MCP, mobile intents, browser extensions, and plugins are read-only by default.

Mutation requires:

- document hash match
- patch plan ID
- policy evaluation
- approval token or interactive confirmation
- audit receipt emission

This rule exists so automation can be powerful without becoming a hidden document mutation path.

## Current policy surface

The CLI policy checker accepts named source aliases for `com`, `applescript`, `dbus`, `android-intent`, `ios-app-intent`, `browser-extension`, `local-api`, and `plugin`. These aliases map into the shared operation-source model while preserving conservative default decisions.

Representative high-risk actions include `apply`, `export`, `external-tool`, `automation`, `plugin`, and `network`. The default posture is review-required or denied unless an explicit policy lane allows the operation.

## Recovery posture

Transaction journal sidecars can be inspected without mutation, and recovery listing identifies incomplete journals that need human or policy-directed crash-recovery review. Recovery diagnostics are evidence surfaces, not hidden repair operations.
