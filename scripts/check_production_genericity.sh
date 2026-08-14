#!/usr/bin/env bash
# Compose the complete clean-tree production-genericity proof under one doctrine entry. The
# default is the fast structural gate plus the frozen behavioral design contract. `--self-test`
# adds the CI qualification oracles: controlled mutations for every boundary class, contract
# mutation controls, and the inventory-bound runtime alpha contract for all rules.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
cd "$ROOT"

qualify=0
if [ "$#" -gt 1 ]; then
  printf 'usage: %s [--self-test]\n' "$0" >&2
  exit 2
fi
if [ "$#" -eq 1 ]; then
  if [ "$1" != "--self-test" ]; then
    printf 'usage: %s [--self-test]\n' "$0" >&2
    exit 2
  fi
  qualify=1
fi

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
run_component \
  BEHAVIORAL-CONTRACT \
  'the behavioral population, input planes, relations, held-out split, and evidence taxonomy are frozen' \
  python3 -B "$ROOT/scripts/check_behavioral_genericity_contract.py"

if [ "$qualify" -eq 1 ]; then
  run_component \
    DEPENDENCY-MUTATIONS \
    'forbidden product/tool dependency directions fail closed' \
    perl "$ROOT/scripts/check_production_genericity_dependencies.pl" --self-test
  run_component \
    INVENTORY-MUTATIONS \
    'unclassified modules and schema-specialized artifact fields fail closed' \
    perl "$ROOT/scripts/check_production_genericity_inventory.pl" --self-test
  run_component \
    RULE-MUTATIONS \
    'missing rules, entrypoints, alpha declarations, and safe bypass contracts fail closed' \
    perl "$ROOT/scripts/check_production_genericity_rules.pl" --self-test
  run_component \
    FLOW-MUTATIONS \
    'identity, raw literal/substring/regex, laundering, unregistered mutation, and proofless promotion fail closed while legal sinks pass' \
    cargo test --quiet --locked --offline -p specforge-production-graph --lib \
      tests::information_flow_fixture_fails_closed_on_boundary_breaches -- --exact
  run_component \
    BEHAVIORAL-CONTRACT-MUTATIONS \
    'population omission, unsafe authority, hash drift, leakage, vacuity, and partial comparison declarations fail closed' \
    python3 -B "$ROOT/scripts/check_behavioral_genericity_contract.py" --self-test
  run_component \
    ALPHA-OBLIGATIONS \
    'all 168 runtime rules exactly match inventory and execute their structural alpha obligations' \
    cargo test --quiet --locked --offline -p specforge-core --lib \
      ir::production_genericity_qualification_tests::every_registered_rule_satisfies_its_inventory_bound_structural_alpha_obligation \
      -- --exact
fi

printf '\n============= PRODUCTION GENERICITY REPORT =============\n' >&2
for line in "${report[@]}"; do
  printf '  %s\n' "$line" >&2
done
printf '========================================================\n' >&2

if [ "$fail" -eq 0 ]; then
  if [ "$qualify" -eq 1 ]; then
    printf 'production-genericity: all %d baseline and qualification components PASS.\n' "${#report[@]}" >&2
  else
    printf 'production-genericity: all %d baseline contract components PASS.\n' "${#report[@]}" >&2
  fi
else
  printf 'production-genericity: one or more requested components FAILED.\n' >&2
fi

exit "$fail"
