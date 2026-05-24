# Implementation Stop Rule

The specification package is now intentionally broad. Further expansion should stop unless implementation reveals a concrete gap.

## Stop adding scope when

- the proposed addition does not change Wave 0 implementation;
- the addition is a late-wave feature with no current contract impact;
- the addition is not tied to a test, CLI command, schema, contract or acceptance criterion;
- the addition adds a dependency without a benchmark, security review or fork policy.

## Add scope only when

- an implementation task is blocked by an undefined contract;
- a platform integration requires a missing schema or adapter boundary;
- tests expose an architectural ambiguity;
- security, crash safety, privacy or redaction correctness requires a new gate.

## Immediate next milestone

Build the Wave 0 skeleton. A boring compiling repo is now more valuable than another perfect plan.
