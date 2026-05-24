# Offline Collaboration and Review Packets

Fe Reader should support collaboration without becoming a cloud-sync product. The core idea is a portable, mergeable review packet.

## Review packet contents

```text
base document fingerprint
annotations
comments
proposed patch plans
reviewer identity label, optional certificate or key fingerprint
timestamps
workflow receipts
conflict metadata
```

The review packet is a sidecar artifact, not a hidden cloud document.

## Packet file types

| Type | Purpose |
|---|---|
| `.fereview` | Portable review packet for comments/annotations/proposed changes. |
| `.feworkspace` | Local workspace bundle manifest. |
| `.fereceipt` | Operation receipt bundle. |
| `.fetemplate` | Workflow/signature/redaction/template bundle. |

## Collaboration workflows

1. User opens a PDF and exports a review packet.
2. Reviewer imports packet against the same PDF hash.
3. Fe Reader shows comments/proposed changes and conflicts.
4. User accepts/rejects proposed patch plans.
5. Accepted changes go through normal transaction/verification/receipt flow.

## Merge rules

- Comments on different objects/pages merge automatically.
- Competing edits to same object require review.
- Redaction proposals are never applied automatically.
- Metadata scrub/optimisation proposals are never applied automatically.
- Packets are hash-bound to the source document and warn on mismatch.

## Security

- Review packets are untrusted input.
- Plugins cannot run from packets.
- Embedded files in packets are quarantined.
- Signatures on packets are provenance hints, not automatic trust.
- Packet import must not mutate the PDF without explicit approval.

## Optional future

Later versions can support local-network peer review, WebDAV/Nextcloud-hosted packets, Git-backed review folders and signed reviewer identities.
