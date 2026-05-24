# Error Taxonomy and User-Facing Messages

Fe Reader needs consistent errors across UI, CLI, MCP, local API, COM, AppleScript, D-Bus, mobile intents, plugins and browser extension calls.

## Goals

- Tell users what happened without leaking document contents.
- Give developers enough structured information to debug.
- Make automation responses stable and machine-readable.
- Preserve privacy in logs, crash reports and support bundles.

## Envelope

```json
{
  "code": "fe.policy.approval_required",
  "category": "approval_required",
  "user_message": "This action needs your approval before the document is changed.",
  "diagnostic": "High-risk mutation requested by MCP tool fe.apply_patch.",
  "retryable": true
}
```

## Message rules

- Do not include document text in error messages.
- Do not include full paths unless the user explicitly asks for diagnostics.
- Prefer actionable language: "Choose a different output location" rather than "EACCES".
- Always include a stable `code` for support and automation.
- Automation must not treat a user-facing message as a policy decision; use structured fields.
