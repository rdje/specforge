#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "${ROOT_DIR}"

echo "[specforge-ci] checking formatting"
cargo fmt --all --check

echo "[specforge-ci] running test suite with Rust warnings denied"
CI_RUSTFLAGS="-D warnings"
if [[ -n "${RUSTFLAGS:-}" ]]; then
  CI_RUSTFLAGS="${RUSTFLAGS} ${CI_RUSTFLAGS}"
fi
RUSTFLAGS="${CI_RUSTFLAGS}" cargo test --manifest-path Cargo.toml

echo "[specforge-ci] building docs"
./scripts/run_docs_ci.sh
