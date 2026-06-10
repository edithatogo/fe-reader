#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IMAGE="${FE_LINUX_RUST_IMAGE:-fe-reader-linux-rust:1.95.0}"
DOCKERFILE="${FE_LINUX_RUST_DOCKERFILE:-${ROOT}/ci/linux-rust/Dockerfile}"

if ! command -v docker >/dev/null 2>&1; then
  echo "docker is required for the Linux Rust container smoke test" >&2
  exit 1
fi

docker build \
  --build-arg RUST_VERSION="${FE_RUST_VERSION:-1.95.0}" \
  -t "${IMAGE}" \
  -f "${DOCKERFILE}" \
  "${ROOT}/ci/linux-rust"

docker run --rm \
  -e CARGO_TERM_COLOR=always \
  -e CARGO_HOME=/tmp/cargo-home \
  -e CARGO_TARGET_DIR=/tmp/fe-reader-target \
  -e HOME=/tmp \
  -e PATH=/usr/local/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
  --user "$(id -u):$(id -g)" \
  -v "${ROOT}:/work" \
  -w /work \
  "${IMAGE}" \
  bash -c '
    set -euo pipefail
    cargo metadata --format-version=1 >/dev/null
    cargo test -p fe_reader_core -p fe_reader_pdf_model -p fe_reader_security -p fe_reader_cli
    cargo run -p fe_reader_cli -- doctor
    python3 scripts/mobile_smoke_bindings_check.py
  '
