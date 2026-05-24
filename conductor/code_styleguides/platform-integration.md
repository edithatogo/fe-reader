# Platform Integration Styleguide

- Prefer vendor-supported permission models.
- Store persistent file grants through platform secure storage where possible.
- Never assume a path is stable on mobile; use document handles/URIs.
- Recent document registration should not leak sensitive names if user disables it.
- Notifications must be opt-in where platform requires permission.
- Automation APIs must be read-only by default.
