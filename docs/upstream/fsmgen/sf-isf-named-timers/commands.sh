#!/usr/bin/env bash
set -u

BUNDLE_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
CASE="$BUNDLE_DIR/sources/fsmgen-input/named-timers.isf"
mkdir -p "$BUNDLE_DIR/observed/stdout" "$BUNDLE_DIR/observed/stderr" "$BUNDLE_DIR/observed/command-logs" "$BUNDLE_DIR/observed/json" "$BUNDLE_DIR/observed/generated"

run_and_capture() {
  local name="$1"
  shift
  printf '%s' "$*" > "$BUNDLE_DIR/observed/command-logs/$name.command"
  "$@" > "$BUNDLE_DIR/observed/stdout/$name.stdout" 2> "$BUNDLE_DIR/observed/stderr/$name.stderr"
  printf '%s\n' "$?" > "$BUNDLE_DIR/observed/command-logs/$name.exit"
}

run_and_capture original ./bin/fsmgen --strict --check --json "$CASE"
run_and_capture check-default ./bin/fsmgen --check --json "$CASE"
run_and_capture check-strict ./bin/fsmgen --strict --check --json "$CASE"
run_and_capture semantic-default ./bin/fsmgen --emit-semantic-json "$CASE"
run_and_capture schedule ./bin/fsmgen --emit-schedule-json "$CASE"
mkdir -p "$BUNDLE_DIR/observed/generated/lowered"
run_and_capture generate-sv ./bin/fsmgen --outdir "$BUNDLE_DIR/observed/generated/lowered" -l sv -o "$BUNDLE_DIR/observed/generated/output.sv" "$CASE"

# Companion cases (SPECFORGE additions):
#   named-timers       (timers (timer pending_hp 1250) (timer credit_hp 625000)), (start-timer …), (expired …)   the primary
#   two-await-limits   two awaits with distinct watchdog limits in one transaction
#   baseline-good      one await with one watchdog limit: accepted
for VARIANT in baseline-good named-timers two-await-limits; do
  run_and_capture "$VARIANT-check-strict" ./bin/fsmgen --strict --check --json \
    "$BUNDLE_DIR/sources/fsmgen-input/$VARIANT.isf"
done
# What a watchdog timeout does today: the baseline's scheduled .fsm (its timeout state sets last_error).
run_and_capture baseline-good-schedule ./bin/fsmgen --emit-schedule-json "$BUNDLE_DIR/sources/fsmgen-input/baseline-good.isf"
