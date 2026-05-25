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
