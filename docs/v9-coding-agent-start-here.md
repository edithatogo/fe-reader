# v9 Coding Agent Start Here

Start by making the repository enforcement real before implementing feature breadth.

## First commands

```bash
python3 scripts/strict_contract_check.py
python3 scripts/repository_ci_cd_check.py
python3 scripts/frontier_ci_check.py
python3 scripts/ci_policy_check.py
python3 scripts/validate_schemas.py
bash scripts/v8_cli_smoke.sh
python3 scripts/wave0_acceptance_check.py
bash scripts/security_policy_check.sh
```

In an environment with Rust installed:

```bash
cargo metadata --format-version=1
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo run -p fe_reader_cli -- doctor
cargo run -p fe_reader_cli -- inspect fixtures/minimal/minimal.pdf --json
```

## Required first PRs after v9 extraction

1. Activate GitHub Actions workflows.
2. Replace action version tags with pinned commit SHAs where policy requires.
3. Configure branch protection / rulesets using `.github/rulesets/main-branch-ruleset.template.json`.
4. Make `strict_contract_check.py` and `repository_ci_cd_check.py` required PR statuses.
5. Materialise public API snapshots for stable crates once they stop being scaffolds.
6. Convert advisory checks to hard gates only after baselines exist and an ADR/promotion note is accepted.

## Stop rule

Do not add new product features until:

- workspace compiles;
- core dependency firewall passes;
- CLI contract snapshots exist;
- schemas validate;
- operation transaction invariant tests pass;
- security policy denies plugin/external/network mutation by default;
- CI workflows pass static policy checks.
