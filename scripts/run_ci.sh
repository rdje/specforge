#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "${ROOT_DIR}"

echo "[specforge-ci] checking formatting"
cargo fmt --all --check

echo "[specforge-ci] running test suite"
cargo test --manifest-path Cargo.toml

echo "[specforge-ci] building docs"
./scripts/run_docs_ci.sh
