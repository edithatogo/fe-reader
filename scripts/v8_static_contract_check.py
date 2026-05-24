#!/usr/bin/env python3
import pathlib, sys
root = pathlib.Path(__file__).resolve().parents[1]
core_toml = (root / 'crates/fe_reader_core/Cargo.toml').read_text(encoding='utf-8')
for forbidden in ['tauri', 'pdfium', 'rmcp', 'extism', 'wasmtime', 'candle', 'burn', 'wgpu', 'vello']:
    if forbidden in core_toml:
        print(f'forbidden dependency in fe_reader_core: {forbidden}', file=sys.stderr)
        raise SystemExit(1)
required = [
    'crates/fe_reader_core/src/lib.rs',
    'crates/fe_reader_pdf_model/src/lib.rs',
    'crates/fe_reader_security/src/lib.rs',
    'crates/fe_reader_cli/src/main.rs',
    'fixtures/minimal/minimal.pdf',
]
missing = [path for path in required if not (root / path).exists()]
if missing:
    print('missing v8 files: ' + ', '.join(missing), file=sys.stderr)
    raise SystemExit(1)
print('v8 static contract check: ok')
