# Configuration and Policy Engine

## Purpose

Settings, feature flags, enterprise policy, automation restrictions, plugin permissions and active-content rules must not be separate ad-hoc systems.

## Layers

```text
compiled defaults
user settings
workspace settings
enterprise policy
runtime safety overrides
command-specific policy
```

Enterprise policy must override user settings for security-sensitive controls.

## Policy examples

```text
disable MCP mutation tools
disable plugins
disable JavaScript actions
disable external network fetches
require secure redaction verification
require metadata scrub before external share
disable unsigned update channels
limit local API to loopback
```

## Contracts

See:

```text
contracts/rust/config_policy_engine.rs
schemas/config-policy.schema.json
schemas/feature-flag.schema.json
```
