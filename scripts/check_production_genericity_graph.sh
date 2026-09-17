#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
# COMMIT-GATE-SINGLE-RUN.1 — this gate takes no arguments, so it used to ACCEPT any and run anyway.
# A wrong flag must be loud: a silently-ignored argument is how a hand-run check becomes a no-op.
[ "$#" -eq 0 ] || { printf 'Usage: %s   (this gate takes no arguments)\n' "$0" >&2; exit 2; }

cd "$ROOT"
exec cargo run --quiet --locked --offline -p specforge-production-graph -- --root "$ROOT"
