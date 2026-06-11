# Job Scheduler, Resource Limits, Power and Thermal Budgets

## Purpose

Fe Reader will perform many long-running operations: OCR, secure redaction, conversion, preflight, rendering, search indexing, thumbnail generation, workflow packs and export. These cannot be fire-and-forget calls from the UI.

## Job model

Every long-running operation must run as a typed job with:

```text
JobSpec -> queued -> running -> cancelling | paused | completed | failed -> retained receipt
```

Jobs must expose:

- progress percentage and stage label
- cancellability
- idempotency key
- resource limits
- output artifact references
- failure cause and retry policy
- crash recovery state

## Power and thermal policy

Performance is not only throughput. On mobile and laptops, Fe Reader must optimise for:

```text
interactive latency
battery drain
thermal throttling
memory pressure
background execution limits
```

## Modes

```text
InteractiveFast      # user is waiting; prioritise latency
Balanced             # default
BatterySaver         # reduce parallelism and prefetch
BackgroundBatch      # low priority, resumable, notification on completion
Benchmark            # deterministic mode for perf CI
```

## Required budgets

- cold app start
- first page visible
- warm tile render
- search index build for 100/1000 pages
- secure redaction rewrite
- conversion export
- OCR batch page cost
- memory peak
- battery/thermal smoke on mobile hardware once available

## Contracts

See:

```text
contracts/rust/job_scheduler.rs
contracts/rust/power_budget.rs
schemas/job-run.schema.json
schemas/power-budget.schema.json
```

## Evidence

- `scripts/job_contract_smoke.py` validates the scheduler contract, the job run schema and the power budget schema.
- Power and thermal policy must stay explicit so long-running work can be scheduled without hiding resource limits.
