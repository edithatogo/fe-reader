# Track O: Security Isolation & Policy Spec

Track O owns threat modelling, sandbox boundaries, automation safety, policy evaluation and dangerous PDF feature handling.

## Deliverables

- `fe_reader_security` crate.
- `SecurityPolicyEngine` implementation.
- default policy templates.
- sandbox/process isolation plan.
- automation read-only defaults.
- embedded JavaScript/RichMedia/Launch action detection policy.
- security phase-gate checks.

## Dependencies

- A0 core operation types.
- E platform adapters for platform-specific permission enforcement.
- I/L/M automation surfaces for policy enforcement.
