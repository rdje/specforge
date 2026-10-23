#!/usr/bin/env bash
# SpecForge adapter for the RUSTDOC doctrine (RUSTDOC-AND-XTASK.2), native since RUSTDOC-AND-XTASK.8 in
# `cargo xtask check rustdoc` (xtask/src/checks/rustdoc.rs): the workspace documents with no rustdoc warning under
# `-D warnings`, after a self-test that requires a scratch crate with an unresolved link to fail. This shim keeps
# the doctrine's registered path; it activates the project-data roots and refuses arguments itself, so its usage
# line names it as the driver invoked it.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
[ "$#" -eq 0 ] || { printf 'Usage: %s   (this gate takes no arguments)\n' "$0" >&2; exit 2; }
cd "$ROOT"
exec cargo xtask check rustdoc
