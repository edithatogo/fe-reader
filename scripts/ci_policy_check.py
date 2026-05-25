#!/usr/bin/env python3
from pathlib import Path
import re
import sys
ROOT = Path(__file__).resolve().parents[1]
failures = []
workflow_dir = ROOT / '.github' / 'workflows'
required_names = {
    '00-pr-contracts.yml',
    '01-rust-stable.yml',
    '02-security-supply-chain.yml',
    '03-cross-platform-smoke.yml',
    '04-api-compatibility.yml',
    '05-frontier-nightly.yml',
    '06-performance-nightly.yml',
    '07-release.yml',
}
actual = {p.name for p in workflow_dir.glob('*.yml')} if workflow_dir.exists() else set()
missing = required_names - actual
for m in sorted(missing):
    failures.append(f'missing workflow {m}')
for wf in workflow_dir.glob('*.yml'):
    txt = wf.read_text(encoding='utf-8')
    if 'concurrency:' not in txt:
        failures.append(f'{wf.name} missing top-level concurrency')
    if 'timeout-minutes:' not in txt:
        failures.append(f'{wf.name} missing job timeout-minutes')
    if 'permissions:\n  contents: read' not in txt:
        failures.append(f'{wf.name} must use read-only contents permission by default')
    for line in txt.splitlines():
        stripped = line.strip()
        if not stripped.startswith('- uses:'):
            continue
        uses_ref = stripped.removeprefix('- uses:').split('#', 1)[0].strip()
        if not re.search(r'@[0-9a-f]{40}$', uses_ref) and 'ALLOW_VERSION_TAGS_DURING_BOOTSTRAP' not in stripped:
            failures.append(f'{wf.name} action use must be SHA pinned or carry bootstrap marker: {uses_ref}')
    if 'id-token: write' in txt and 'attest' not in txt and 'release' in wf.name:
        failures.append(f'{wf.name} grants id-token without attestation step')
    if 'contents: write' in txt and 'release' not in wf.name:
        failures.append(f'{wf.name} has contents: write outside release workflow')
    for forbidden_permission in ['actions: write', 'checks: write', 'packages: write']:
        if forbidden_permission in txt:
            failures.append(f'{wf.name} grants forbidden permission: {forbidden_permission}')

    if wf.name in {
        '00-pr-contracts.yml',
        '01-rust-stable.yml',
        '02-security-supply-chain.yml',
        '03-cross-platform-smoke.yml',
        '04-api-compatibility.yml',
    }:
        if 'continue-on-error: true' in txt:
            failures.append(f'{wf.name} is a stable lane and must not continue-on-error')
        if 'pull_request:' not in txt or 'push:' not in txt or 'branches: [main]' not in txt:
            failures.append(f'{wf.name} stable lane must run on pull_request and push to main')

    if wf.name in {'05-frontier-nightly.yml', '06-performance-nightly.yml'}:
        if 'pull_request:' in txt or 'push:' in txt:
            failures.append(f'{wf.name} frontier lane must not run on PR/push')
        if 'workflow_dispatch:' not in txt or 'schedule:' not in txt:
            failures.append(f'{wf.name} frontier lane must be manual and scheduled')
        if 'continue-on-error: true' not in txt:
            failures.append(f'{wf.name} frontier lane must be isolated with continue-on-error')
        if 'permissions:\n  contents: read' not in txt:
            failures.append(f'{wf.name} frontier lane must use read-only contents permission')
        for forbidden_permission in ['contents: write', 'id-token: write', 'packages: write']:
            if forbidden_permission in txt:
                failures.append(f'{wf.name} frontier lane grants forbidden permission: {forbidden_permission}')
        if wf.name == '05-frontier-nightly.yml' and 'actions/upload-artifact' in txt:
            failures.append(f'{wf.name} frontier lane must not upload artifacts during bootstrap')
        if wf.name == '06-performance-nightly.yml':
            for token in ['actions/upload-artifact', 'artifacts/perf/**', 'if-no-files-found: warn', 'retention-days:']:
                if token not in txt:
                    failures.append(f'{wf.name} performance evidence upload missing token: {token}')

    if wf.name == '07-release.yml':
        if 'workflow_dispatch:' not in txt:
            failures.append('07-release.yml must be manually dispatched during bootstrap')
        if 'bash scripts/release_evidence_check.sh' not in txt:
            failures.append('07-release.yml must produce release evidence')
        if 'python3 scripts/release_matrix_check.py' not in txt:
            failures.append('07-release.yml must validate release package matrix')
        if 'bash scripts/release_readiness_check.sh' not in txt:
            failures.append('07-release.yml must run release readiness')
        for command in [
            'bash scripts/sbom_audit.sh',
            'bash scripts/generate_provenance_attestation.sh',
            'bash scripts/signing_readiness_check.sh',
            'python3 scripts/release_provenance_check.py',
        ]:
            if command not in txt:
                failures.append(f'07-release.yml missing release provenance command: {command}')
        if 'actions/upload-artifact' not in txt or 'target/release-evidence/**' not in txt:
            failures.append('07-release.yml must upload release evidence artifacts')
        if 'if-no-files-found: error' not in txt:
            failures.append('07-release.yml must fail when release evidence artifact files are missing')
        if 'retention-days:' not in txt:
            failures.append('07-release.yml must set release evidence retention-days')

    for match in re.finditer(r'\bscripts/[A-Za-z0-9_.\-/]+', txt):
        script_rel = match.group(0).rstrip('",\'')
        if not (ROOT / script_rel).exists():
            failures.append(f'{wf.name} references missing script {script_rel}')

pr_contracts = workflow_dir / '00-pr-contracts.yml'
if pr_contracts.exists():
    txt = pr_contracts.read_text(encoding='utf-8')
    for command in [
        'python3 scripts/validate_schemas.py',
        'python3 scripts/v8_static_contract_check.py',
        'python3 scripts/strict_contract_check.py',
        'python3 scripts/strict_mutation_contract_check.py',
        'python3 scripts/repository_ci_cd_check.py',
        'python3 scripts/frontier_ci_check.py',
        'python3 scripts/ci_policy_check.py',
        'python3 scripts/wave0_acceptance_check.py',
    ]:
        if command not in txt:
            failures.append(f'00-pr-contracts.yml missing hard gate: {command}')

stable_commands = {
    '01-rust-stable.yml': [
        'cargo metadata --format-version=1',
        'cargo fmt --all -- --check',
        'cargo test --workspace --all-targets',
        'cargo clippy --workspace --all-targets --all-features -- -D warnings',
    ],
    '02-security-supply-chain.yml': [
        'bash scripts/security_policy_check.sh',
        'bash scripts/actions_security_gate.sh',
        'bash scripts/supply_chain_gate.sh',
    ],
    '03-cross-platform-smoke.yml': [
        'cargo metadata --format-version=1',
        'cargo test -p fe_reader_core -p fe_reader_pdf_model -p fe_reader_security -p fe_reader_cli',
        'cargo run -p fe_reader_cli -- doctor',
    ],
    '04-api-compatibility.yml': [
        'bash scripts/api_compat_check.sh',
        'bash scripts/public_api_snapshot_check.sh',
    ],
}
for wf_name, commands in stable_commands.items():
    path = workflow_dir / wf_name
    if not path.exists():
        continue
    txt = path.read_text(encoding='utf-8')
    for command in commands:
        if command not in txt:
            failures.append(f'{wf_name} missing stable command: {command}')

frontier_commands = {
    '05-frontier-nightly.yml': [
        'toolchain: [beta, nightly]',
        'cargo +${{ matrix.toolchain }} check --workspace --all-targets',
        'bash scripts/miri_smoke.sh',
        'bash scripts/sanitizer_smoke.sh',
        'bash scripts/fuzz_smoke.sh',
        'bash scripts/gpu_frontier_smoke.sh',
        'bash scripts/differential_oracle_smoke.sh',
    ],
    '06-performance-nightly.yml': [
        'bash scripts/perf_smoke.sh',
        'bash scripts/toolchain_experiment_smoke.sh',
    ],
}
for wf_name, commands in frontier_commands.items():
    path = workflow_dir / wf_name
    if not path.exists():
        continue
    txt = path.read_text(encoding='utf-8')
    for command in commands:
        if command not in txt:
            failures.append(f'{wf_name} missing frontier command: {command}')
if failures:
    print('CI POLICY CHECK FAILED')
    for f in failures:
        print(f' - {f}')
    sys.exit(1)
print('ci policy check passed')
