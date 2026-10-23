#!/usr/bin/env bash
# SpecForge's README-POLICY doctrine (LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.2), native since RUSTDOC-AND-XTASK.12
# in `cargo xtask check readme-policy` (xtask/src/checks/readme_policy.rs): README.md within its reviewed line and
# byte ceilings, README_POLICY.md's fenced adoption note, and every reader link and author-overflow destination
# registered as a controlled terminal in doctrine/readme_entrypoint/routed_destinations.tsv. This shim keeps the
# doctrine's registered path; it refuses arguments and activates the project-data roots itself, so its usage line
# names it as the caller invoked it.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# COMMIT-GATE-SINGLE-RUN.1 — this gate takes exactly one optional mode, `--self-test`, and used to
# ACCEPT any other argument and run the ordinary check anyway, reporting success. A near-miss on the
# one real mode therefore ran the wrong thing silently, which is how a hand-run check becomes a no-op.
[ "$#" -eq 0 ] || { [ "$#" -eq 1 ] && [ "${1:-}" = "--self-test" ]; } || {
  printf 'Usage: %s [--self-test]\n' "$0" >&2; exit 2
}
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
cd "$ROOT"
exec cargo xtask check readme-policy "$@"
