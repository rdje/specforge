#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "${ROOT_DIR}"

echo "[specforge-ci] checking formatting"
cargo fmt --all --check

CI_RUSTFLAGS="-D warnings"
if [[ -n "${RUSTFLAGS:-}" ]]; then
  CI_RUSTFLAGS="${RUSTFLAGS} ${CI_RUSTFLAGS}"
fi

echo "[specforge-ci] running clippy with warnings denied"
RUSTFLAGS="${CI_RUSTFLAGS}" cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings

echo "[specforge-ci] running test suite with Rust warnings denied"
RUSTFLAGS="${CI_RUSTFLAGS}" cargo test --manifest-path Cargo.toml

echo "[specforge-ci] building docs"
./scripts/run_docs_ci.sh
