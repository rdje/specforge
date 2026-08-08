#!/usr/bin/env bash
# Lock the two current-state mdBook facts repaired by LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5a.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ACTOR_BOOK="$ROOT/docs/book/src/domain/actor-connectivity.md"
SCOPE_BOOK="$ROOT/docs/book/src/reference/documentation-scope.md"
TEMPORAL_BOOK="$ROOT/docs/book/src/direction/temporal-intent-capture.md"
ISF_CODE="$ROOT/crates/specforge/src/ir/isf_ir.rs"
CLI_CODE="$ROOT/crates/specforge/src/cli.rs"
DISPATCH_CODE="$ROOT/crates/specforge/src/lib.rs"

fail=0
problem() {
  printf 'book-current-truth: %s\n' "$1" >&2
  fail=1
}

require_literal() {
  local path="$1"
  local literal="$2"
  local description="$3"
  grep -Fq "$literal" "$path" || problem "$description"
}

reject_literal() {
  local path="$1"
  local literal="$2"
  local description="$3"
  if grep -Fq "$literal" "$path"; then
    problem "$description"
  fi
}

for path in "$ACTOR_BOOK" "$SCOPE_BOOK" "$TEMPORAL_BOOK" "$ISF_CODE" "$CLI_CODE" "$DISPATCH_CODE"; do
  [ -f "$path" ] || problem "required current-truth source '${path#"$ROOT"/}' is missing."
done

if [ "$fail" -eq 0 ]; then
  require_literal "$ISF_CODE" 'select_initiator_actor(&intent_ir.actor_ports)' \
    'ISF lowering no longer selects its initiator from actor_ports; reverify and update the canonical book contract.'
  require_literal "$ISF_CODE" 'initiator_perspective_directions(&intent_ir.actor_ports, &initiator)' \
    'ISF lowering no longer derives initiator-perspective directions; reverify and update the canonical book contract.'
  require_literal "$ACTOR_BOOK" 'The adapter selects one structurally' \
    'actor-connectivity.md no longer states the current initiator-selection contract.'
  require_literal "$ACTOR_BOOK" '../pipeline/isf-adapter.md#which-way-does-each-signal-point' \
    'actor-connectivity.md lost its canonical ISF-direction pointer.'
  reject_literal "$ACTOR_BOOK" 'deliberately does **not** re-derive' \
    'actor-connectivity.md restored the stale claim that ISF lowering ignores actor-relative direction.'

  require_literal "$CLI_CODE" 'ExtractContracts(ExtractContractsArgs)' \
    'the extract-contracts CLI surface is absent; reverify the temporal-intent delivery claim.'
  require_literal "$DISPATCH_CODE" 'Commands::ExtractContracts(args) => commands::extract_contracts::run(args)' \
    'the extract-contracts command is no longer dispatched; reverify the temporal-intent delivery claim.'
  require_literal "$TEMPORAL_BOOK" 'This bounded design now ships through' \
    'temporal-intent-capture.md no longer states the delivered constrained-extraction status.'
  require_literal "$TEMPORAL_BOOK" '../commands/quality-and-learning.md#extract-contracts-and-signal-resolve' \
    'temporal-intent-capture.md lost its canonical live-command pointer.'
  reject_literal "$TEMPORAL_BOOK" 'None of it ships any code today' \
    'temporal-intent-capture.md restored the stale pre-delivery design claim.'
fi

if [ "$fail" -ne 0 ]; then
  printf 'book-current-truth: FAILED — reverify code first, then keep the canonical mdBook statement and pointer aligned.\n' >&2
  exit 1
fi

printf 'book-current-truth: actor-direction and constrained-extraction facts match code and canonical book routes.\n'
