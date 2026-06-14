#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PREFIX="${PREFIX:-$HOME/.local}"
BINDIR="$PREFIX/bin"
PROFILE="${PROFILE:-release}"

case "$PROFILE" in
  debug)
    cargo build --manifest-path "$ROOT/Cargo.toml" -p fe_reader_cli
    BINARY="$ROOT/target/debug/fe-reader"
    ;;
  release)
    cargo build --manifest-path "$ROOT/Cargo.toml" -p fe_reader_cli --release
    BINARY="$ROOT/target/release/fe-reader"
    ;;
  *)
    echo "Unsupported PROFILE=$PROFILE; expected debug or release" >&2
    exit 2
    ;;
esac

if [[ ! -x "$BINARY" ]]; then
  echo "Built binary is missing or not executable: $BINARY" >&2
  exit 1
fi

install -d "$BINDIR"
install -m 0755 "$BINARY" "$BINDIR/fe-reader"

cat <<EOF
Installed Fe Reader CLI to:
  $BINDIR/fe-reader

Add this to PATH if needed:
  export PATH="$BINDIR:\$PATH"

Verify with:
  fe-reader doctor
EOF
