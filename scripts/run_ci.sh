#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "${ROOT_DIR}"

echo "[specforge-ci] enforcing doctrines (DOCTRINE_ENFORCEMENT.md: memory-arch + knowledge-map + task-acceptance via the registry/driver)"
./scripts/check_doctrines.sh

echo "[specforge-ci] checking formatting"
cargo fmt --all --check

CI_RUSTFLAGS="-D warnings"
if [[ -n "${RUSTFLAGS:-}" ]]; then
  CI_RUSTFLAGS="${RUSTFLAGS} ${CI_RUSTFLAGS}"
fi
CI_RUSTDOCFLAGS="-D warnings"
if [[ -n "${RUSTDOCFLAGS:-}" ]]; then
  CI_RUSTDOCFLAGS="${RUSTDOCFLAGS} ${CI_RUSTDOCFLAGS}"
fi

echo "[specforge-ci] running clippy with warnings denied"
RUSTFLAGS="${CI_RUSTFLAGS}" cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings

echo "[specforge-ci] running test suite with Rust warnings denied"
RUSTFLAGS="${CI_RUSTFLAGS}" cargo test --manifest-path Cargo.toml

echo "[specforge-ci] building Rust docs with warnings denied"
RUSTDOCFLAGS="${CI_RUSTDOCFLAGS}" cargo doc --manifest-path Cargo.toml --no-deps

echo "[specforge-ci] building docs"
./scripts/run_docs_ci.sh
