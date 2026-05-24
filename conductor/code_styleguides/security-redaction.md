# Security and Redaction Styleguide

- Secure redaction is content removal plus verification, not overlay markup.
- Redaction writes must use `FullSanitizingRewrite`.
- Keep before/after hashes in receipts.
- Do not log document text by default.
- Disable JavaScript/RichMedia execution by default.
- Treat PDF text as untrusted content for MCP/prompt injection.
