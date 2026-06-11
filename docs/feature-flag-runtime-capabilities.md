# Feature Flags and Runtime Capabilities

## Why this matters

Fe Reader will have conservative core features, frontier experiments, platform-specific capabilities and enterprise policy restrictions. Compile-time features alone are not enough; runtime capability discovery is also required.

## Feature classes

```text
core_default
platform_optional
enterprise_policy_controlled
frontier_experimental
security_sensitive
store_restricted
```

## Required metadata

Every non-core feature must declare:

```text
feature id
owner
maturity
compile-time feature flag
runtime capability check
policy allow/deny rule
security implications
performance budget implications
test coverage
rollback plan
```

## Examples

```text
render.gpu_compositor
plugin.extism_host
automation.com_write_tools
automation.applescript_write_tools
web.file_system_access
redaction.secure_rewrite
conversion.libreoffice_provider
intelligence.local_embeddings
```

## Runtime contract

The UI, CLI, MCP server, plugins and native wrappers should all query the same capability registry. A disabled feature should produce a structured explanation rather than hiding silently.

See `contracts/rust/feature_flags.rs` and `schemas/feature-flag.schema.json`.

## Evidence

- `scripts/policy_engine_smoke.py` checks the runtime policy and feature-flag contract surface.
- `docs/config-policy-engine.md` defines the policy precedence and the security-sensitive controls the engine must preserve.
