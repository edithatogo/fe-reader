#!/usr/bin/env python3
from pathlib import Path
import json, sys
schema = Path('schemas/adr.schema.json')
if not schema.exists():
    print('missing schemas/adr.schema.json', file=sys.stderr); sys.exit(1)
for p in Path('docs/adr').glob('*.json') if Path('docs/adr').exists() else []:
    json.loads(p.read_text())
print('ADR validation smoke complete')
