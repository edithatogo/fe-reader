#!/usr/bin/env python3
from __future__ import annotations
import subprocess
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
    '.github/rulesets/main-branch-ruleset.template.json',
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

manifest = ROOT / 'contracts/contract-manifest.yaml'
if manifest.exists():
    txt = manifest.read_text(encoding='utf-8')
    for gate in [
        'architecture_boundary',
        'schema_validation',
        'operation_transaction',
        'cli_json_contract',
        'security_policy_default_deny',
        'automation_mutation_requires_approval',
    ]:
        if gate not in txt:
            failures.append(f'contract manifest missing hard gate: {gate}')

core_toml = ROOT / 'crates/fe_reader_core/Cargo.toml'
if core_toml.exists():
    txt = core_toml.read_text(encoding='utf-8').lower()
    forbidden = ['tauri', 'pdfium-render', 'rmcp', 'extism', 'wasmtime', 'candle', 'burn', 'wgpu', 'vello', 'skia-safe', 'keyring', 'notify-rust', 'zbus', 'windows', 'objc', 'cocoa']
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

mcp_tools = ROOT / 'contracts/mcp/tools.manifest.json'
if mcp_tools.exists():
    txt = mcp_tools.read_text(encoding='utf-8')
    for token in ['"default_mode": "read_only"', 'fe.plan_redaction', 'fe.plan_workflow', 'fe.apply_approved_patch', 'approval token']:
        if token not in txt:
            failures.append(f'MCP tools manifest missing mutation-safety token: {token}')

mcp_policy = ROOT / 'contracts/mcp/server-policy.yaml'
if mcp_policy.exists():
    txt = mcp_policy.read_text(encoding='utf-8')
    for token in ['default_mode: read_only', 'document_hash_match', 'patch_plan_id', 'user_approval_token', 'policy_allow_rule']:
        if token not in txt:
            failures.append(f'MCP server policy missing approval token: {token}')

web_contract = ROOT / 'contracts/web/postmessage-contract.md'
if web_contract.exists():
    txt = web_contract.read_text(encoding='utf-8')
    for token in ['OperationIntent', 'read-only or plan-only', 'explicit user approval', 'No hidden background upload']:
        if token not in txt:
            failures.append(f'web postMessage contract missing safety token: {token}')

matrix = ROOT / 'contracts/ci/contract-test-matrix.yaml'
if matrix.exists():
    txt = matrix.read_text(encoding='utf-8')
    for token in [
        'python3 scripts/strict_mutation_contract_check.py',
        'bash scripts/release_evidence_check.sh',
        'python3 scripts/release_matrix_check.py',
    ]:
        if token not in txt:
            failures.append(f'contract test matrix missing command: {token}')

if failures:
    print('STRICT CONTRACT CHECK FAILED')
    for f in failures:
        print(f' - {f}')
    sys.exit(1)
subprocess.run([sys.executable, 'scripts/strict_mutation_contract_check.py'], cwd=ROOT, check=True)
print('strict contract check passed')
