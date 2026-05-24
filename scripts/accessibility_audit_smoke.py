#!/usr/bin/env python3
from pathlib import Path
root = Path(__file__).resolve().parents[1]
required = [
    root / 'schemas' / 'accessibility-audit.schema.json',
    root / 'contracts' / 'rust' / 'accessibility.rs',
    root / 'docs' / 'ux-accessibility-human-factors.md',
]
missing = [str(p) for p in required if not p.exists()]
if missing:
    print('accessibility smoke missing:', missing)
    raise SystemExit(1)
print('accessibility smoke: ok')
