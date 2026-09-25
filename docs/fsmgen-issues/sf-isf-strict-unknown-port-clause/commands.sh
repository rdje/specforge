#!/usr/bin/env bash
set -u

BUNDLE_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
CASE="$BUNDLE_DIR/sources/fsmgen-input/bogus-clause.isf"
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

# Companion cases (SPECFORGE additions): the same port with an unknown clause (bogus-clause, the primary), with a
# plausible clause FSMGen does not document on ports (present-when), and with no clause (baseline-good). Each is
# strict-checked and generated to SystemVerilog, so the generated modules can be compared line for line.
for VARIANT in baseline-good present-when bogus-clause; do
  run_and_capture "$VARIANT-check-strict" ./bin/fsmgen --strict --check --json \
    "$BUNDLE_DIR/sources/fsmgen-input/$VARIANT.isf"
  mkdir -p "$BUNDLE_DIR/observed/generated/$VARIANT"
  run_and_capture "$VARIANT-generate-sv" ./bin/fsmgen --strict --outdir "$BUNDLE_DIR/observed/generated/$VARIANT" -l sv \
    -o "$BUNDLE_DIR/observed/generated/$VARIANT/output.sv" "$BUNDLE_DIR/sources/fsmgen-input/$VARIANT.isf"
done

# The comparison the question rests on: each generated module against the one without the clause, ignoring only
# the generation timestamp in the header.
for VARIANT in present-when bogus-clause; do
  if diff <(grep -v '^// Date:' "$BUNDLE_DIR/observed/generated/baseline-good/output.sv") \
          <(grep -v '^// Date:' "$BUNDLE_DIR/observed/generated/$VARIANT/output.sv") > /dev/null; then
    echo "identical" > "$BUNDLE_DIR/observed/command-logs/$VARIANT-vs-baseline.txt"
  else
    echo "differs" > "$BUNDLE_DIR/observed/command-logs/$VARIANT-vs-baseline.txt"
  fi
done
