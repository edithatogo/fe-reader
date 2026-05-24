# Platform Automation Deepening

Fe Reader supports native automation, but automation must be constrained by the same policy model as UI, CLI and MCP.

## Windows

- COM `IDispatch` automation for enterprise scripts and Office-style automation.
- PowerShell wrapper around the local API for admins.
- Group Policy/ADMX settings for disabling plugins, external automation, diagnostics and mutating automation.
- Default mode: read-only automation. Mutations require approval token or managed policy.

## macOS

- AppleScript/Scripting Bridge for document inspection, export and workflow planning.
- App Intents/Shortcuts for modern user-facing automations.
- Configuration profiles for managed policy.
- Default mode: read-only automation; mutating App Intents require explicit confirmation.

## Linux

- D-Bus interface for desktop automation and portal-aware operations.
- CLI remains the most portable automation surface.
- Flatpak portals are preferred for file access.
- Default mode: read-only D-Bus methods; mutating methods require policy unlock.

## Android

- Intents for open/share/export.
- DocumentsProvider for workspace exposure.
- Managed configuration for enterprise/work-profile installs.
- Avoid background document mutation through implicit intents.

## iOS/iPadOS

- App Intents and Shortcuts for user-approved workflows.
- Document browser and share sheet integration.
- No silent background mutation of Files app documents.

## Contract rule

All automation surfaces translate into `OperationIntent`; no automation surface may directly call PDF mutation functions.
