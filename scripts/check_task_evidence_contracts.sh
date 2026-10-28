#!/usr/bin/env bash
# Check every registered active-task-evidence contract. Native implementation:
# xtask/src/checks/task_evidence_contracts.rs (RUSTDOC-AND-XTASK.15.y).
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

cd "$ADAPTER_ROOT" || exit 1
export SPECFORGE_SHIM_PATH="$0"
exec cargo xtask check task-evidence-contracts --root "$ROOT"
