#!/usr/bin/env bash
set -euo pipefail
if [[ -f templates/policy/default-security-policy.json ]]; then
  python3 -m json.tool templates/policy/default-security-policy.json >/dev/null
fi
if grep -R "execute_pdf_javascript.*true" -n docs contracts schemas templates 2>/dev/null; then
  echo "PDF JavaScript must not be enabled by default" >&2
  exit 1
fi
echo "security policy check passed"
