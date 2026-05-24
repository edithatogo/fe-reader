#!/usr/bin/env python3
from pathlib import Path
import json, sys
schema = Path('schemas/feature-flag.schema.json')
profile = Path('templates/policy/feature-flags.default.json')
if not schema.exists():
    print('missing schemas/feature-flag.schema.json', file=sys.stderr); sys.exit(1)
if profile.exists():
    json.loads(profile.read_text())
print('feature flag smoke complete')
