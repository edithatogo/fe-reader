# Strict Contracts Plan

1. [x] Materialise files for this track.
2. [x] Run `python3 scripts/strict_contract_check.py`.
3. [x] Run `python3 scripts/ci_policy_check.py`.
4. [x] Update `docs/v9-coding-agent-start-here.md` with any missing first PRs.
5. [x] Mark advisory checks as hard only after baselines exist and an ADR approves the promotion.

## Completion Evidence

- Added strict mutation contract enforcement for Rust automation, plugin host, MCP tools, web postMessage, COM, Android intents, iOS App Intents, and operation transactions.
- Wired strict mutation, security policy, schema, CI policy, Wave 0 acceptance, and release evidence checks into hard gates where baselines exist.
- Added a main branch ruleset template and release evidence artifact upload path.
- Verified with `scripts/conductor_phase_gate.sh --phase track-AL-strict-contracts --auto-fix`.
