# Enterprise Deployment and Policy

## Purpose

Fe Reader should be usable by individuals, schools, clinics, government teams and enterprises without becoming cloud-first.

## Policy channels

| Platform | Policy path |
|---|---|
| Windows | MSI properties, registry policy, optional ADMX later |
| macOS | managed preferences / configuration profile |
| Linux | system config under `/etc/fe-reader/policy.json` and Flatpak overrides |
| Android | managed configuration where supported |
| iOS | MDM managed app configuration where supported |

## Policy examples

- disable MCP server
- disable plugins
- disable PDF JavaScript actions
- disable RichMedia
- require metadata clean-share prompt before external sharing
- require redaction verification before export
- disable external converters
- force local-only mode
- disable diagnostics upload
- enforce update channel
- set allowed workflow packs

## Precedence

```text
system/MDM policy
  > workspace policy
  > user preference
  > default app setting
```

User preferences cannot weaken system policy.
