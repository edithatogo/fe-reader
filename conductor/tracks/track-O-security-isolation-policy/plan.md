# Track O Plan

## O0 Wave 0

- [x] Materialise `contracts/rust/security_policy.rs`.
- [x] Create `fe_reader_security` crate skeleton.
- [x] Add `templates/policy/default-security-policy.json`.
- [x] Add `scripts/security_policy_check.sh`.
- Evidence: `contracts/rust/security_policy.rs`, `crates/fe_reader_security/src/lib.rs`, `templates/policy/default-security-policy.json`, `scripts/security_policy_check.sh`.

## O1 Wave 1

- [x] Enforce read-only default for automation surfaces.
- [x] Add policy checks to CLI mutation paths.
- [x] Add PDF dangerous-action inspection types.
- Evidence: `crates/fe_reader_security/src/lib.rs`, `crates/fe_reader_cli/src/main.rs`, `scripts/strict_mutation_contract_check.py`, `scripts/policy_engine_smoke.py`.

## O2 Wave 3

- [x] Block secure redaction unless write mode is `FullSanitizingRewrite`.
- [x] Require approval for all export/share operations from automation clients.
- Evidence: `crates/fe_reader_redaction/src/lib.rs`, `scripts/redaction_verification_smoke.sh`, `scripts/security_policy_check.sh`.

## O3 Wave 5

- [x] Apply policy to MCP, COM, AppleScript, D-Bus, web postMessage and plugin proposals.
- Evidence: `contracts/mcp/server-policy.yaml`, `contracts/platform/windows-com/FeReaderAutomation.idl`, `contracts/platform/linux-dbus/org.fereader.FeReader1.xml`, `contracts/web/postmessage-contract.md`.

## O4 Wave 7

- [x] Add enterprise policy precedence tests.
- [x] Produce security review report.
- Evidence: `crates/fe_reader_config/src/lib.rs`, `docs/security-review-track-o.md`, `target/release-evidence/track-o-security-review.json`.
