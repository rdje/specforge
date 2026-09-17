#!/usr/bin/env bash
# SpecForge adapter for the CONSTRAINT-PART-SPAN doctrine (EXTRACTION-QUALITY-GAUGE.3k.2j).
#
# `scripts/measure_constraint_part_span.py` stratifies the published constraint surface by the
# PRODUCER that minted each record, and its `--check` mode fails closed when the kind classifier's
# call-site topology moves — because a stratification derived from a topology that has changed is a
# number about a function that is no longer there.
#
# It was written by `EXTRACTION-QUALITY-GAUGE.3k` and named in that leaf's verification, and it was
# in no driver. Re-derived from each revision's own source with that revision's own scanner, it went
# RED at `.3k.2a` and stayed RED through `.3k.2b` and `.3k.2c` — three leaves that each reported a
# fully green gate. A fail-closed check nothing executes is not a check (DOCTRINE_ENFORCEMENT.md §3),
# which is the whole reason this adapter exists.
#
# Self-test first, then the check, for the reason CORPUS-FRONTIER states: a gate is not trusted on a
# day its own negatives have not been re-proven. Read-only, offline, no provider, sub-second — so it
# belongs at `gate` tier despite reading the persisted corpus, which it only opens for the census
# path and never for `--check`.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
# COMMIT-GATE-SINGLE-RUN.1 — this gate takes no arguments, so it used to ACCEPT any and exit 0.
# A wrong flag must be loud: a silently-ignored argument is how a hand-run check becomes a no-op.
[ "$#" -eq 0 ] || { printf 'Usage: %s   (this gate takes no arguments)\n' "$0" >&2; exit 2; }
cd "$ROOT"

fail=0
python3 -B scripts/measure_constraint_part_span.py --self-test || fail=1
python3 -B scripts/measure_constraint_part_span.py --check || fail=1
exit "$fail"
