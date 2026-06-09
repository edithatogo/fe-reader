# Dependency and Tool Version Policy

## Rule

Use the newest verified stable release for core dependencies, but only after review of:

```text
licence
MSRV/rust-version impact
security advisory status
API stability
transitive dependency growth
platform support
performance impact
```

## Frontier lanes

Bleeding-edge dependencies are allowed only in feature-gated edge crates and must have an exit plan.

## Mandatory checks before adding a dependency

```bash
cargo deny check
cargo audit
cargo vet
cargo tree -e features
cargo bloat --release --crates || true
```

`cargo vet` exemptions are bootstrap debt, not a substitute for human audits.
They may keep CI actionable while the dependency tree is being stabilized, but
public release readiness should track the generated vet report and retire
exemptions into real audits over time.

## Direct fork rule

Fork only when there is a blocking bug, security fix, app-store/platform constraint, or inactive upstream. Every fork needs a documented owner, upstream issue/PR link, rebase cadence, removal plan and target exit date.
