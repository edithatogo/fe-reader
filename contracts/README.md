# Contracts

Contracts define the stable boundaries between Fe Reader's core, platform integrations, automation surfaces, application integrations, conversion providers, plugin host, and UI shells.

Every implementation must respect these rules:

1. `FeOperationIntent` is the universal entry point for operations.
2. High-risk operations return patch plans, not immediate mutation.
3. Platform adapters must not leak OS-specific types into `fe_reader_core`.
4. Application integrations must declare capabilities and risk level.
5. Automation APIs are read-only by default.
6. Plugins may propose operations but may not directly mutate PDFs.
7. Web and browser extension integrations use explicit user-granted file access.


## v6 contracts

v6 adds contracts for:

```text
Document IR
Transformation passes
Job scheduler
Power/thermal budgets
Accessibility audit reports
E2E test scenarios
PDF Time Machine
Active Content Firewall
Config/policy engine
```

Coding agents should implement these as typed boundaries before adding UI or platform-specific logic.
