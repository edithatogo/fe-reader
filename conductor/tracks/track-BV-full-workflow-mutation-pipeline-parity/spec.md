# Track BV: Full Workflow Mutation Pipeline Parity

## Overview

Implement one end-to-end approved low-risk mutation workflow across shared adapters to prove the full Fe Reader pipeline from intent through audit receipt.

## Functional Requirements

- Choose metadata update as the first approved apply workflow.
- Implement the flow through `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.
- Produce a new output document or controlled local artifact; do not silently mutate the original input.
- Expose the approved workflow through shared facade contracts and platform shells where available.
- Keep redaction high-risk and plan/verify-only until verified rewritten output evidence exists.

## Non-Functional Requirements

- Every apply path requires document hash match, patch plan ID, policy evaluation, approval token or interactive confirmation, and audit receipt emission.
- Adapter surfaces must not bypass core policy or review.

## Acceptance Criteria

- Metadata update apply smoke produces a verifiable audit receipt.
- Platform parity matrix updates `approved-mutation-apply` only for surfaces with actual evidence.
- Redaction remains blocked from applied public claims unless separately verified.

## Out of Scope

- Direct redaction apply.
- Signing or signature-preserving updates.
