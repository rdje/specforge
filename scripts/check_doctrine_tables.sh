#!/usr/bin/env bash
# SpecForge's DOCTRINE-TABLES doctrine (RUSTDOC-AND-XTASK.18), native from its first commit in
# `cargo xtask check doctrine-tables` (xtask/src/checks/doctrine_tables.rs): the doctrines scripts/check_doctrines.sh
# registers are exactly those documented in docs/DOCTRINE_INSTANCE.md §10, with the tier and check the driver gives
# them, and in the book's enforcement table, with the same kind. This shim is the doctrine's registered path; it
# refuses arguments and activates the project-data roots itself, so its usage line names it as the caller invoked it.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
[ "$#" -eq 0 ] || { [ "$#" -eq 1 ] && [ "${1:-}" = "--self-test" ]; } || {
  printf 'Usage: %s [--self-test]\n' "$0" >&2; exit 2
}
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
cd "$ROOT"
exec cargo xtask check doctrine-tables "$@"
