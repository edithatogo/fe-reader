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
if failures:
    print('CI POLICY CHECK FAILED')
    for f in failures:
        print(f' - {f}')
    sys.exit(1)
print('ci policy check passed')
