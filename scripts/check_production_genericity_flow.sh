#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

cd "$ROOT"
# PRODUCTION-GRAPH-CENSUS-PIN.1 — `--check-census` makes this derivation COMPARE itself against
# doctrine/production_genericity/flow_census.json instead of only printing. The analysis was
# already being paid for on every commit; only the comparison is new.
exec cargo run --quiet --locked --offline -p specforge-production-graph -- --root "$ROOT" --check-census
