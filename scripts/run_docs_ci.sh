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
