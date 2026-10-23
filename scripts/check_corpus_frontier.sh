#!/usr/bin/env bash
# SpecForge adapter for the CORPUS-FRONTIER doctrine (CORPUS-COVERAGE.4.1), native since RUSTDOC-AND-XTASK.14.a in
# `cargo xtask check corpus-frontier` (xtask/src/checks/corpus_frontier.rs).
#
# The refresh frontier must be DERIVED from persisted corpus evidence and diffed against an exact declaration,
# never carried as a number each slice decrements. The native check runs the core's self-test first when checking
# this repository, so the gate is never trusted on a day its own negatives have not been re-proven. This shim keeps
# the doctrine's registered path, its argument refusal and the project-data activation.
set -uo pipefail

ADAPTER_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ADAPTER_ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ADAPTER_ROOT"
ROOT="$ADAPTER_ROOT"
if [ "$#" -gt 0 ]; then
  if [ "$#" -ne 2 ] || [ "$1" != '--root' ] || [ ! -d "$2" ]; then
    printf 'Usage: %s [--root PROJECT_ROOT]\n' "$0" >&2
    exit 2
  fi
  ROOT="$(cd "$2" && pwd)"
fi
cd "$ADAPTER_ROOT"
exec cargo xtask check corpus-frontier --root "$ROOT"
