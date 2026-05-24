# Coding Agent Evaluation Harness

## Purpose

This package is designed to be read by coding agents. v6 adds an explicit harness to evaluate whether agents are implementing the specification rather than drifting.

## Evaluation dimensions

```text
architecture compliance
schema compliance
CLI contract compliance
operation safety compliance
platform adapter boundaries
performance budget compliance
fixture/corpus coverage
security policy compliance
accessibility baseline compliance
API compatibility
```

## Golden tasks

Create a `tests/agent_eval/` suite with tasks such as:

```text
add a read-only CLI command
add a schema-backed workflow template
add a rendering cache metric without touching core
add a platform adapter stub without leaking OS types
add a redaction verification check without incremental append
```

The harness should grade generated changes by running phase gates and checking file-boundary rules.
