#!/usr/bin/env bash
# Enforce repository-derived roots for every project-owned temp/cache/artifact seam: the PROJECT-DATA-LOCALITY
# doctrine, native since RUSTDOC-AND-XTASK.14.c in `cargo xtask check project-data-locality`
# (xtask/src/checks/project_data_locality.rs). This shim keeps the doctrine's registered path, its argument refusal
# and the shell activation every entry point performs; the native gate activates the same roots itself and reports
# a failure once, so the shell's own message is not repeated here.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# COMMIT-GATE-SINGLE-RUN.1 — this gate takes no arguments, so it used to ACCEPT any and run anyway.
# A wrong flag must be loud: a silently-ignored argument is how a hand-run check becomes a no-op.
[ "$#" -eq 0 ] || { printf 'Usage: %s   (this gate takes no arguments)\n' "$0" >&2; exit 2; }
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT" 2>/dev/null
cd "$ROOT"
exec cargo xtask check project-data-locality
