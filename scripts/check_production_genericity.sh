#!/usr/bin/env bash
# Compose the complete clean-tree production-genericity proof under one doctrine entry. The
# default is the fast structural gate plus the frozen behavioral design contract. `--self-test`
# adds the CI qualification oracles: controlled mutations for every boundary class, contract
# mutation controls, and the inventory-bound runtime alpha contract for all rules.
#
# RUSTDOC-AND-XTASK.15.e — native since that leaf in `cargo xtask check production-genericity`
# (xtask/src/checks/production_genericity.rs), which runs the four xtask components in-process and
# the graph crate and qualification tests through cargo, as this script did. This shim keeps the
# doctrine's registered path, the project-data activation and the argument check, and passes the
# name it was invoked by, which the gate's refusals quote.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
cd "$ROOT"

if [ "$#" -gt 1 ] || { [ "$#" -eq 1 ] && [ "$1" != "--self-test" ]; }; then
  printf 'usage: %s [--self-test]\n' "$0" >&2
  exit 2
fi

exec cargo xtask check production-genericity --program "$0" "$@"
