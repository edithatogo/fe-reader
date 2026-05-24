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

Initial required checks:

```text
strict-contracts / policy
rust-stable / fmt-clippy-test
security-supply-chain / dependency-policy
cross-platform-smoke / linux
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

`.github/rulesets/main-branch-ruleset.template.json` is intentionally a template. It must be imported or converted using GitHub repository settings/REST API once the exact required check names exist.

## Action pinning

During bootstrap, workflows may use major-version tags for readability. Before public release, all non-local actions should be pinned to immutable commit SHAs, then scanned with `zizmor`.
