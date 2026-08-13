#!/usr/bin/env bash
# Compose the complete clean-tree production-genericity proof under one doctrine entry.
#
# Mutation breadth and executable per-rule alpha obligations are intentionally outside this
# wrapper; SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.vi owns those qualification oracles.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
cd "$ROOT"

fail=0
declare -a report=()

run_component() {
  local id="$1"
  local proves="$2"
  local output
  shift 2

  if output="$("$@" 2>&1)"; then
    report+=("PASS  ${id} — ${proves}")
    if [ -n "$output" ]; then
      printf '%s\n' "$output"
    fi
  else
    report+=("FAIL  ${id} — ${proves}")
    if [ -n "$output" ]; then
      printf '%s\n' "$output" >&2
    fi
    fail=1
  fi
}

run_component \
  DEPENDENCIES \
  'the product package direction and enforcement-tool disconnection hold' \
  perl "$ROOT/scripts/check_production_genericity_dependencies.pl"
run_component \
  INVENTORY \
  'the production module and claim-family inventories are exact and complete' \
  perl "$ROOT/scripts/check_production_genericity_inventory.pl"
run_component \
  RULES \
  'every canonical field, producer/mutator, seam, and conformance bypass is registered' \
  perl "$ROOT/scripts/check_production_genericity_rules.pl"
run_component \
  INFORMATION-FLOW \
  'the compiled graph enforces raw/identity noninterference and proof-only promotion' \
  bash "$ROOT/scripts/check_production_genericity_flow.sh"

printf '\n============= PRODUCTION GENERICITY REPORT =============\n' >&2
for line in "${report[@]}"; do
  printf '  %s\n' "$line" >&2
done
printf '========================================================\n' >&2

if [ "$fail" -eq 0 ]; then
  printf 'production-genericity: all %d structural components PASS.\n' "${#report[@]}" >&2
else
  printf 'production-genericity: one or more structural components FAILED.\n' >&2
fi

exit "$fail"
