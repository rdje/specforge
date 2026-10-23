#!/usr/bin/env bash
# RUSTDOC-AND-XTASK.5 — the docs build (the mdBook, then the rustdoc API reference beside it) is native in
# `cargo xtask docs` (xtask/src/docs.rs). This shim keeps every recorded `scripts/run_docs_ci.sh` working; it
# activates the project-data roots first so cargo's own process, not only the command, resolves them.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT_DIR"

cd "${ROOT_DIR}"
exec cargo xtask docs "$@"
