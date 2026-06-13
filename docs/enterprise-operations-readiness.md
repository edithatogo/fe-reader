# Enterprise Operations Readiness

Fe Reader enterprise operations stay local-first. Managed deployment support must not introduce remote analytics, mandatory cloud management or default collection of private document contents.

## Install Modes

Install mode evidence is defined in `packaging/enterprise/install-modes.yaml`.

| Platform | Per-user | Global or managed | Offline artifact blockers |
| --- | --- | --- | --- |
| macOS | `~/Applications` | `/Applications` | Developer ID signing and notarization profile |
| Windows | LocalAppData or portable zip | Program Files via MSI/MSIX | Authenticode signing certificate and password |
| Linux | `~/.local/bin` or AppImage | package manager or `/opt` | distro package signing and repository publication |

## Managed Policy

The baseline lockdown template lives at `packaging/enterprise/managed-lockdown-policy.json`. Policy precedence is:

```text
system/MDM policy
  > workspace policy
  > user preference
  > default app setting
```

User preferences cannot weaken managed policy. Risky integrations such as MCP mutation, plugins, external converters, native automation apply and network access are disabled by default in the lockdown template.

## Updates and Rollback

Signed update fixtures live under `packaging/enterprise/update-manifests/`. Stable and LTS manifests must include artifact digests, artifact signatures, manifest signatures, provenance paths and signing-readiness evidence paths.

Rollback manifests are explicit: `rollback: true` is required, and unsigned or digest-mismatched artifacts fail the enterprise operations smoke check.

## Support Bundle Review

Support bundle defaults are controlled by `packaging/enterprise/support-bundle-allowlist.yaml`.

Default bundles may include app version, operating system, architecture, enabled feature flags, policy summaries, error classes, redacted crash stacks, dependency versions, release evidence digests and performance counters.

Default bundles must not include raw PDFs, document text, rendered page images, search indexes, credentials, access tokens, private keys, full file paths, usernames or document titles. Users must preview and approve any support bundle export.

## Evidence

Run:

```bash
python3 scripts/enterprise_operations_readiness_check.py
```

The script writes `target/release-evidence/enterprise-operations-readiness.json`.
