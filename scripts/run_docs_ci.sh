#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT_DIR"

cd "${ROOT_DIR}"

if ! command -v mdbook >/dev/null 2>&1; then
  echo "[specforge-docs] mdbook is required but not installed"
  exit 1
fi

echo "[specforge-docs] testing mdBook examples"
mdbook test docs/book

echo "[specforge-docs] building mdBook"
mdbook build docs/book

# RUSTDOC-AND-XTASK.3 — the API reference is the workspace's rustdoc, published beside the book so its
# reference page can link each crate's index. `-D warnings` is the RUSTDOC doctrine's own flag.
echo "[specforge-docs] building the rustdoc API reference"
RUSTDOCFLAGS="-D warnings${RUSTDOCFLAGS:+ $RUSTDOCFLAGS}" cargo doc --offline --locked --workspace --no-deps
api_dir="generated/mdbook/specforge/api"
rm -rf "${api_dir}"
cp -R "${CARGO_TARGET_DIR:-target}/doc" "${api_dir}"
for crate in specforge_core specforge specforge_conformance specforge_production_graph; do
  if [ ! -f "${api_dir}/${crate}/index.html" ]; then
    echo "[specforge-docs] the API reference lacks ${crate}/index.html"
    exit 1
  fi
done
echo "[specforge-docs] API reference written to ${api_dir}"
