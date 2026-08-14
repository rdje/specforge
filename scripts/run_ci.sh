#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT_DIR"

cd "${ROOT_DIR}"

echo "[specforge-ci] enforcing every registered doctrine via the registry/driver (all tiers)"
./scripts/check_doctrines.sh --all

echo "[specforge-ci] qualifying production genericity with controlled mutations, behavioral-contract faults, and per-rule alpha obligations"
./scripts/check_production_genericity.sh --self-test

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

echo "[specforge-ci] rechecking project-data residue after all producers"
./scripts/check_project_data_locality.sh
