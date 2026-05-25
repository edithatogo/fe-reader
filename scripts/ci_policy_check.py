#!/usr/bin/env python3
from pathlib import Path
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
    if 'actions/checkout@v' in txt and 'ALLOW_VERSION_TAGS_DURING_BOOTSTRAP' not in txt:
        failures.append(f'{wf.name} uses checkout version tag without bootstrap marker')
    if 'id-token: write' in txt and 'attest' not in txt and 'release' in wf.name:
        failures.append(f'{wf.name} grants id-token without attestation step')
    if 'contents: write' in txt and 'release' not in wf.name:
        failures.append(f'{wf.name} has contents: write outside release workflow')

    if wf.name in {
        '00-pr-contracts.yml',
        '01-rust-stable.yml',
        '02-security-supply-chain.yml',
        '03-cross-platform-smoke.yml',
        '04-api-compatibility.yml',
    } and 'continue-on-error: true' in txt:
        failures.append(f'{wf.name} is a stable lane and must not continue-on-error')

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
        if 'actions/upload-artifact' in txt:
            failures.append(f'{wf.name} frontier lane must not upload artifacts during bootstrap')

    if wf.name == '07-release.yml':
        if 'workflow_dispatch:' not in txt:
            failures.append('07-release.yml must be manually dispatched during bootstrap')
        if 'bash scripts/release_evidence_check.sh' not in txt:
            failures.append('07-release.yml must produce release evidence')
        if 'python3 scripts/release_matrix_check.py' not in txt:
            failures.append('07-release.yml must validate release package matrix')
        if 'bash scripts/release_readiness_check.sh' not in txt:
            failures.append('07-release.yml must run release readiness')
        if 'actions/upload-artifact' not in txt or 'target/release-evidence/**' not in txt:
            failures.append('07-release.yml must upload release evidence artifacts')

    for line in txt.splitlines():
        stripped = line.strip()
        if not stripped.startswith('- run:'):
            continue
        command = stripped.removeprefix('- run:').strip()
        parts = command.split()
        if len(parts) >= 2 and parts[0] in {'bash', 'python3'} and parts[1].startswith('scripts/'):
            script = ROOT / parts[1]
            if not script.exists():
                failures.append(f'{wf.name} references missing script {parts[1]}')

pr_contracts = workflow_dir / '00-pr-contracts.yml'
if pr_contracts.exists():
    txt = pr_contracts.read_text(encoding='utf-8')
    for command in [
        'python3 scripts/validate_schemas.py',
        'python3 scripts/v8_static_contract_check.py',
        'python3 scripts/strict_contract_check.py',
        'python3 scripts/strict_mutation_contract_check.py',
        'python3 scripts/ci_policy_check.py',
        'python3 scripts/wave0_acceptance_check.py',
    ]:
        if command not in txt:
            failures.append(f'00-pr-contracts.yml missing hard gate: {command}')
if failures:
    print('CI POLICY CHECK FAILED')
    for f in failures:
        print(f' - {f}')
    sys.exit(1)
print('ci policy check passed')
