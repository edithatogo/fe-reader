# Track O Plan

## O0 Wave 0

- Materialise `contracts/rust/security_policy.rs`.
- Create `fe_reader_security` crate skeleton.
- Add `templates/policy/default-security-policy.json`.
- Add `scripts/security_policy_check.sh`.

## O1 Wave 1

- Enforce read-only default for automation surfaces.
- Add policy checks to CLI mutation paths.
- Add PDF dangerous-action inspection types.

## O2 Wave 3

- Block secure redaction unless write mode is `FullSanitizingRewrite`.
- Require approval for all export/share operations from automation clients.

## O3 Wave 5

- Apply policy to MCP, COM, AppleScript, D-Bus, web postMessage and plugin proposals.

## O4 Wave 7

- Add enterprise policy precedence tests.
- Produce security review report.
