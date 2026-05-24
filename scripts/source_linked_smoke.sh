#!/usr/bin/env bash
set -euo pipefail
test -f schemas/source-linked-project.schema.json
test -f contracts/rust/source_linked_document.rs
test -f docs/source-linked-authoring-workflows.md
echo "source-linked smoke: ok"
