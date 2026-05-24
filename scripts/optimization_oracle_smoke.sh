#!/usr/bin/env bash
set -euo pipefail
test -f schemas/pdf-optimization-plan.schema.json
test -f contracts/rust/pdf_optimization.rs
test -f docs/pdf-optimization-linearization-compression.md
echo "optimization oracle smoke: ok"
