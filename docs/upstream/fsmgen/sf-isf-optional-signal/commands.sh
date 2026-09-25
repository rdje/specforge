#!/usr/bin/env bash
set -u

BUNDLE_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
CASE="$BUNDLE_DIR/sources/fsmgen-input/absent-by-default.isf"
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

# Companion cases (SPECFORGE additions). Each differs from the accepted baseline in one stated place:
#   absent-by-default   MECID_WIDTH defaults to 0 (the port is absent unless enabled)
#   literal-zero-width  the port is declared (width 0)
#   tie-off             an absent input declares the value it reads, (absent-value 0)
#   present-when        a port is present only when PROT_Present is non-zero (PROT_Present 0)
for VARIANT in baseline-good absent-by-default literal-zero-width tie-off present-when; do
  run_and_capture "$VARIANT-check-strict" ./bin/fsmgen --strict --check --json \
    "$BUNDLE_DIR/sources/fsmgen-input/$VARIANT.isf"
done

# present-when passes strict, so show what it generates: with PROT_Present 0 the port is still emitted,
# because the (present ...) clause is accepted and ignored.
mkdir -p "$BUNDLE_DIR/observed/generated/present-when"
run_and_capture present-when-generate-sv ./bin/fsmgen --outdir "$BUNDLE_DIR/observed/generated/present-when/lowered" \
  -l sv -o "$BUNDLE_DIR/observed/generated/present-when/output.sv" "$BUNDLE_DIR/sources/fsmgen-input/present-when.isf"
grep -c 'AWPROT' "$BUNDLE_DIR/observed/generated/present-when/output.sv" \
  > "$BUNDLE_DIR/observed/command-logs/present-when-awprot-line-count.txt"
