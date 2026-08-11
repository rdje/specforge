#!/usr/bin/env bash
# SpecForge adapter for the CORPUS-FRONTIER doctrine (CORPUS-COVERAGE.4.1).
#
# The refresh frontier must be DERIVED from persisted corpus evidence and diffed against an exact
# declaration, never carried as a number each slice decrements. The core, its rationale, and its
# four checks live in scripts/check_corpus_frontier_census.pl.
#
# Runs the core's self-test first when checking this repository, so the gate is never trusted on a
# day its own negatives have not been re-proven.
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

fail=0
if [ "$ROOT" = "$ADAPTER_ROOT" ]; then
  perl "$ROOT/scripts/check_corpus_frontier_census.pl" --self-test || fail=1
fi

perl "$ROOT/scripts/check_corpus_frontier_census.pl" \
  --root "$ROOT" \
  --contract doctrine/corpus_frontier/census.json \
  --check || fail=1

exit "$fail"
