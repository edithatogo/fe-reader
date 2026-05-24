# Fe Reader v9 Improvements: Strict Contracts and Repository CI/CD

v9 is a repository-hardening pass. It does not add broad product scope. It makes the existing plan enforceable through contracts, CI/CD, repository governance, release provenance, and bleeding-edge experiment lanes.

## What v9 adds

- Actual `.github/workflows/*.yml` workflows instead of only `ci/*.sample` files.
- Repository governance files: `CODEOWNERS`, Dependabot, Renovate, ruleset template, branch-protection guide.
- Strict contract manifests and hard gates for architecture boundaries, public APIs, CLI output, schemas, security policy, sidecar migrations, and automation safety.
- CI/CD lanes for stable PR checks, cross-platform checks, security/supply chain, API compatibility, nightly/frontier experiments, performance, release provenance, and release candidates.
- A strict distinction between **hard stable gates** and **frontier experiment lanes**.

## Strictness model

Strict does not mean every experimental tool blocks contributors. Strict means the repository has explicit gate levels:

| Gate | Blocks PR? | Purpose |
|---|---:|---|
| P0 hard | Yes | Compile, formatting, clippy, core contract tests, schemas, architecture boundaries, security policy |
| P1 hard after baseline | Yes | Performance budgets, visual regression, compatibility corpus, API snapshots |
| Advisory | No initially | Fuzz smoke, differential oracles, platform packaging smoke before targets are stable |
| Frontier | No, scheduled/manual | Nightly Rust, Miri, sanitizers, GPU experiments, PGO/BOLT, fuzz campaigns |
| Release hard | Yes on release tags | SBOM, attestations, signed artifacts, release evidence, semver/API compatibility |

## Repo policy

The implementation agent must treat repository policy as code. If a workflow, ruleset, CODEOWNERS domain, branch protection, supply-chain check, or contract manifest is missing, implementation must stop and add it before expanding feature scope.

## Bleeding-edge posture

- Stable PR CI uses pinned stable Rust and hard contracts.
- Beta/nightly CI is scheduled and manual; failures create issues rather than blocking normal PRs unless they expose a real stable-contract bug.
- Experimental dependencies must live outside `fe_reader_core` and behind features.
- Git dependencies require an ADR, owner, exit criterion, and expiry date.
- Actions should be pinned to immutable SHAs before production activation. Version tags are acceptable only while bootstrapping the repo.
