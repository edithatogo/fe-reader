# Repository Governance and Branch Protection

## Main branch policy

The `main` branch should require:

- pull request before merge;
- at least two approvals once maintainers exist;
- CODEOWNERS review for security, PDF mutation, release and platform automation areas;
- required status checks;
- signed commits or GitHub verified signatures where feasible;
- linear history;
- no force pushes;
- no deletions;
- stale review dismissal on changed code;
- administrator enforcement once the repo stabilises.

## Required status checks

Initial required checks use the emitted job contexts from the bootstrap workflows:

```text
strict-contracts
repository-ci-cd
rust
policy
smoke
```

After baselines exist, add:

```text
api-compatibility / public-api
performance-nightly / budget-smoke
compatibility-corpus / smoke
visual-regression / smoke
release-candidate / evidence
```

## Ruleset template

`.github/rulesets/main-branch-ruleset.template.json` is intentionally a template. It encodes pull-request review, CODEOWNERS review, required status checks, linear history, signed commits, no force pushes and no branch deletion. It must be imported or converted using GitHub repository settings/REST API once the exact emitted required check names exist.

If GitHub does not expose a setting through repository rulesets for the active plan, configure it manually in repository settings and document the manual setting in the first governance PR.

## Dependency automation ownership

Renovate owns scheduled Cargo dependency refresh PRs so high-risk PDF, frontier and Rust patch/minor routing can be expressed in one policy file. Dependabot owns GitHub Actions version refreshes. Dependabot security alerts remain enabled at the repository level.

## Action pinning

During bootstrap, workflows may use major-version tags for readability. Before public release, all non-local actions should be pinned to immutable commit SHAs, then scanned with `zizmor`.
