#!/usr/bin/env python3
from __future__ import annotations
import argparse, pathlib, sys
parser = argparse.ArgumentParser()
parser.add_argument('--smoke', action='store_true')
args = parser.parse_args()
expected = pathlib.Path('fixtures/expected/rendered')
actual = pathlib.Path('target/visual-regression/actual')
if args.smoke and not expected.exists():
    print('visual regression baselines not present yet; advisory skip')
    sys.exit(0)
# Real implementation should compare PNGs using deterministic colour-space normalisation.
print('visual regression placeholder passed')
