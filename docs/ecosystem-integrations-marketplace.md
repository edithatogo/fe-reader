# Ecosystem Integrations and Marketplace

The `ecosystem_integrations_marketplace` feature gate is disabled by default and does not block desktop stable launch. It governs SDK, plugin, workflow-pack and marketplace expansion after release gates are stable.

## API And SDK Compatibility

Public API changes require compatibility snapshots and compatibility notes for CLI, MCP, plugin ABI, UniFFI, C ABI and .NET wrapper surfaces. SDK examples must never bypass `OperationIntent -> PatchPlan -> Review/Policy -> Apply -> Verify -> AuditReceipt`.

## Publication Gates

Plugin, workflow-pack, SDK package and marketplace publication require signed artifacts, SBOM, provenance, compatibility reports, support policy, security policy, maintainer approval and rollback plans. Until that evidence exists, publication status remains deferred.

## Automation Safety

Plugin and integration mutations remain read-only or plan-only by default. Plugin runtime publication stays proposal-only, and unsafe plugin runtime enablement is out of scope.

## Marketplace Metadata

Marketplace metadata must link the homepage, `SUPPORT.md`, `SECURITY.md`, license, compatibility evidence and publication blockers. Metadata fixtures live in `fixtures/ecosystem/marketplace/metadata.preview.json`.

## Rollback

Rollback disables plugin/runtime publication, marketplace submission and unsigned package publication while retaining compatibility snapshots, read-only integrations and plan-only workflow packs.

## Evidence

The machine-readable contract is `contracts/snapshots/ecosystem/ecosystem-integrations-marketplace.preview.json`.

Run:

```bash
python3 scripts/ecosystem_integrations_marketplace_check.py
```
