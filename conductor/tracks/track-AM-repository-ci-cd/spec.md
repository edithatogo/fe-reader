# Repository CI/CD Spec

Purpose: make v9 repository enforcement executable and auditable.

Acceptance:

- contract manifests exist;
- GitHub workflows exist and pass `ci_policy_check.py`;
- stable lanes are hard gates;
- frontier lanes are scheduled/manual and isolated;
- release lanes produce evidence artifacts;
- no mutating surface bypasses transaction contracts.
