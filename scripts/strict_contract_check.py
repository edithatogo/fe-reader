#!/usr/bin/env python3
from __future__ import annotations
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
failures: list[str] = []

required = [
    'contracts/contract-manifest.yaml',
    'contracts/cli/cli-contract.md',
    'contracts/ci/contract-test-matrix.yaml',
    'contracts/ci/api-boundary-policy.yaml',
    'schemas/pdf-document-summary.schema.json',
    'schemas/operation-transaction.schema.json',
    'crates/fe_reader_core/src/lib.rs',
    'crates/fe_reader_cli/src/main.rs',
    '.github/workflows/00-pr-contracts.yml',
    '.github/workflows/01-rust-stable.yml',
    '.github/workflows/02-security-supply-chain.yml',
    '.github/workflows/03-cross-platform-smoke.yml',
    '.github/workflows/04-api-compatibility.yml',
    '.github/workflows/05-frontier-nightly.yml',
    '.github/workflows/06-performance-nightly.yml',
    '.github/workflows/07-release.yml',
    '.github/CODEOWNERS',
    '.github/dependabot.yml',
    'renovate.json',
]
for rel in required:
    if not (ROOT / rel).exists():
        failures.append(f'missing required file: {rel}')

cli_contract = ROOT / 'contracts/cli/cli-contract.md'
if cli_contract.exists():
    txt = cli_contract.read_text(encoding='utf-8')
    for token in ['summary.parser', 'page count', 'trailer keys', 'non-fatal parser error']:
        if token not in txt:
            failures.append(f'CLI contract missing parser contract token: {token}')

summary_schema = ROOT / 'schemas/pdf-document-summary.schema.json'
if summary_schema.exists():
    txt = summary_schema.read_text(encoding='utf-8')
    for token in ['"parser"', '"adapter"', '"page_count"', '"encrypted"', '"trailer_keys"', '"error"']:
        if token not in txt:
            failures.append(f'PDF document summary schema missing parser token: {token}')

core_toml = ROOT / 'crates/fe_reader_core/Cargo.toml'
if core_toml.exists():
    txt = core_toml.read_text(encoding='utf-8').lower()
    forbidden = ['tauri', 'pdfium-render', 'rmcp', 'extism', 'wasmtime', 'candle', 'burn', 'wgpu', 'vello', 'skia-safe', 'keyring', 'zbus']
    for name in forbidden:
        if name in txt:
            failures.append(f'fe_reader_core must not depend on {name}')
else:
    failures.append('missing fe_reader_core Cargo.toml')

core_rs = ROOT / 'crates/fe_reader_core/src/lib.rs'
if core_rs.exists():
    txt = core_rs.read_text(encoding='utf-8')
    for symbol in ['OperationIntent', 'PatchPlan', 'TransactionJournal', 'OperationReceipt', 'WriteMode']:
        if symbol not in txt:
            failures.append(f'fe_reader_core missing contract symbol: {symbol}')

workflows = ROOT / '.github/workflows'
if workflows.exists():
    for wf in workflows.glob('*.yml'):
        body = wf.read_text(encoding='utf-8')
        if 'permissions:' not in body:
            failures.append(f'{wf.relative_to(ROOT)} missing explicit permissions')
        if 'pull_request_target' in body:
            failures.append(f'{wf.relative_to(ROOT)} uses pull_request_target, forbidden without ADR')
        if 'timeout-minutes:' not in body:
            failures.append(f'{wf.relative_to(ROOT)} missing job timeout-minutes')

if failures:
    print('STRICT CONTRACT CHECK FAILED')
    for f in failures:
        print(f' - {f}')
    sys.exit(1)
print('strict contract check passed')
