#!/usr/bin/env bash
set -u

BUNDLE_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
CASE="$BUNDLE_DIR/sources/fsmgen-input/parameter-width.isf"
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

# Companion cases (SPECFORGE additions): the same actor with a literal 32-bit port (baseline-good), with the port
# sized by DATA_WIDTH defaulting to 32 (parameter-width, the primary), and defaulting to 64. Each is generated to
# SystemVerilog, and its module parameter and WDATA declarations are extracted: the parameter is declared, but
# the port and its register are sized by the resolved default, so overriding the parameter changes nothing.
for VARIANT in baseline-good parameter-width parameter-width-64; do
  run_and_capture "$VARIANT-check-strict" ./bin/fsmgen --strict --check --json \
    "$BUNDLE_DIR/sources/fsmgen-input/$VARIANT.isf"
  mkdir -p "$BUNDLE_DIR/observed/generated/$VARIANT"
  run_and_capture "$VARIANT-generate-sv" ./bin/fsmgen --outdir "$BUNDLE_DIR/observed/generated/$VARIANT/lowered" \
    -l sv -o "$BUNDLE_DIR/observed/generated/$VARIANT/output.sv" "$BUNDLE_DIR/sources/fsmgen-input/$VARIANT.isf"
  grep -n -E 'parameter|WDATA(_next)?[,;]|WDATA <= [0-9]' "$BUNDLE_DIR/observed/generated/$VARIANT/output.sv" \
    > "$BUNDLE_DIR/observed/command-logs/$VARIANT-declarations.txt"
done
