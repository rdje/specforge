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
declare seen_ids=''

# ── The corpus components must have their subject ────────────────────────────────────────────
# COMMIT-GATE-SINGLE-RUN.15. `run_ci.sh` invokes this script DIRECTLY, not through
# `scripts/check_doctrines.sh --only`, so the corpus assertion the driver makes never reached here
# and these components ran against an absent corpus on the first hosted run in five months. The rule
# has to live where the work happens, not only on one path to it.
#
# Only these two are quantified over the persisted corpus. Measured `2026-09-20` in a corpus-free
# clone with a warm cargo cache: the other nine components PASS, so skipping the whole gate would
# give up real enforcement that a runner CAN do. That is why this list is per component.
CORPUS_DEPENDENT_COMPONENTS=(
  "BEHAVIORAL-CONTRACT"
  "BEHAVIORAL-CONTRACT-MUTATIONS"
)

corpus_dependent_component() {
  local want="$1" dep
  for dep in "${CORPUS_DEPENDENT_COMPONENTS[@]}"; do
    [ "$dep" = "$want" ] && return 0
  done
  return 1
}

CORPUS_ABSENT=''
if [ ! -d "$ROOT/generated/source_ir" ] || [ -z "$(ls -A "$ROOT/generated/source_ir" 2>/dev/null)" ]; then
  CORPUS_ABSENT=1
fi
if [ -n "$CORPUS_ABSENT" ] && [ -z "${SPECFORGE_CORPUS_ABSENT:-}" ]; then
  printf '%s: refused — generated/source_ir is empty or absent, so %d components of this gate\n' \
    "$0" "${#CORPUS_DEPENDENT_COMPONENTS[@]}" >&2
  printf '%s: have no subject in this tree and would fail on the missing artifacts.\n' "$0" >&2
  printf '%s: build the corpus, or declare the environment corpus-free with SPECFORGE_CORPUS_ABSENT=1\n' "$0" >&2
  printf '%s: to have them reported as NOT discharged instead.\n' "$0" >&2
  exit 2
fi

run_component() {
  local id="$1"
  local proves="$2"
  local output
  shift 2
  seen_ids="${seen_ids} ${id}"

  # Declared corpus-free: say the obligation was not discharged rather than run it against nothing.
  if [ -n "$CORPUS_ABSENT" ] && corpus_dependent_component "$id"; then
    report+=("SKIP  ${id} — corpus absent and declared; NOTHING was measured")
    printf 'production-genericity: SKIP: no corpus at generated/source_ir — %s does not govern this tree. Nothing was measured; nothing is claimed.\n' "$id"
    return 0
  fi

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
    'all 170 runtime rules exactly match inventory and execute their structural alpha obligations' \
    cargo test --quiet --locked --offline -p specforge-core --lib \
      ir::production_genericity_qualification_tests::every_registered_rule_satisfies_its_inventory_bound_structural_alpha_obligation \
      -- --exact
fi

# A dangling entry in CORPUS_DEPENDENT_COMPONENTS would silently stop guarding a component, so it is
# meta-checked against the components this gate actually ran. `--self-test` reaches every one of them.
if [ "$qualify" -eq 1 ]; then
  for comp in "${CORPUS_DEPENDENT_COMPONENTS[@]}"; do
    case " ${seen_ids} " in
      *" ${comp} "*) ;;
      *) printf '%s: CORPUS_DEPENDENT_COMPONENTS names no component of this gate: %s\n' "$0" "$comp" >&2
         exit 2 ;;
    esac
  done
fi

printf '\n============= PRODUCTION GENERICITY REPORT =============\n' >&2
for line in "${report[@]}"; do
  printf '  %s\n' "$line" >&2
done
printf '========================================================\n' >&2

if [ "$fail" -eq 0 ]; then
  # A skipped component is NOT a passing one, and the count must not absorb it
  # (COMMIT-GATE-SINGLE-RUN.15 — the same mistake `.14` found one layer up, in this gate's own
  # summary line: "all 5 baseline contract components PASS" while one of the five measured nothing).
  executed=0
  ungoverned_ids=''
  for line in "${report[@]}"; do
    case "$line" in
      SKIP*) skip_rest="${line#SKIP  }"
             ungoverned_ids="${ungoverned_ids}${ungoverned_ids:+, }${skip_rest%% *}" ;;
      *)     executed=$((executed + 1)) ;;
    esac
  done
  if [ "$qualify" -eq 1 ]; then
    printf 'production-genericity: %d of %d baseline and qualification components executed and PASS.\n' \
      "$executed" "${#report[@]}" >&2
  else
    printf 'production-genericity: %d of %d baseline contract components executed and PASS.\n' \
      "$executed" "${#report[@]}" >&2
  fi
  [ -n "$ungoverned_ids" ] && printf 'production-genericity: NOT GOVERNED here, so NOT enforced by this run: %s.\n' \
    "$ungoverned_ids" >&2
else
  printf 'production-genericity: one or more requested components FAILED.\n' >&2
fi

exit "$fail"
