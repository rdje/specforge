#!/usr/bin/env bash
set -u

BUNDLE_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
CASE="$BUNDLE_DIR/sources/fsmgen-input/no-transaction-assert.isf"
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
#   baseline-good            the invariant inside a transaction (accepted)
#   actor-level-assert       the same invariant at actor level
#   actor-level-assume       the same, as an assumption
#   no-transaction-control   an actor with no transaction (accepted)
#   no-transaction-assert    that actor with one invariant (the primary)
for VARIANT in baseline-good actor-level-assert actor-level-assume no-transaction-control no-transaction-assert; do
  run_and_capture "$VARIANT-check-strict" ./bin/fsmgen --strict --check --json \
    "$BUNDLE_DIR/sources/fsmgen-input/$VARIANT.isf"
done
# The accepted baseline's invariant is emitted as an always-on concurrent property, with no transaction-state
# gating: hosting it in a transaction is placement only.
mkdir -p "$BUNDLE_DIR/observed/generated/baseline-good"
run_and_capture baseline-good-generate-sv ./bin/fsmgen --outdir "$BUNDLE_DIR/observed/generated/baseline-good/lowered" \
  -l sv -o "$BUNDLE_DIR/observed/generated/baseline-good/output.sv" "$BUNDLE_DIR/sources/fsmgen-input/baseline-good.isf"
grep -n 'assert property' "$BUNDLE_DIR/observed/generated/baseline-good/output.sv" \
  > "$BUNDLE_DIR/observed/command-logs/baseline-good-assert-property.txt"
