# Web / Browser Extension postMessage Contract

## Message envelope

```json
{
  "fe_reader_protocol": "0.1",
  "message_id": "uuid",
  "origin": "browser-extension|web-local|self-hosted",
  "operation": "open|inspect|plan_workflow|plan_redaction|plan_conversion",
  "risk": "read_only|plan_only",
  "payload": {}
}
```

## Rules

- Browser messages create `FeOperationIntent` with source `WebPostMessage`.
- Browser integrations are read-only or plan-only unless handed off to native app with explicit user approval.
- Browser postMessage must not apply write changes or export converted output directly.
- Web local file access must use user-granted file handles.
- No hidden background upload of document contents.
