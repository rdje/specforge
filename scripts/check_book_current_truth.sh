#!/usr/bin/env bash
# Lock code-bound mdBook facts plus the bounded root compatibility routes.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
# COMMIT-GATE-SINGLE-RUN.1 — this gate takes no arguments, so it used to ACCEPT any and run anyway.
# A wrong flag must be loud: a silently-ignored argument is how a hand-run check becomes a no-op.
[ "$#" -eq 0 ] || { printf 'Usage: %s   (this gate takes no arguments)\n' "$0" >&2; exit 2; }
ACTOR_BOOK="$ROOT/docs/book/src/domain/actor-connectivity.md"
SCOPE_BOOK="$ROOT/docs/book/src/reference/documentation-scope.md"
TEMPORAL_BOOK="$ROOT/docs/book/src/direction/temporal-intent-capture.md"
SUMMARY="$ROOT/docs/book/src/SUMMARY.md"
EXTRACTION_BOOK="$ROOT/docs/book/src/reference/extraction-architecture.md"
INTENT_CONTRACT_BOOK="$ROOT/docs/book/src/reference/intentir-contract.md"
LIVE_DOCS_BOOK="$ROOT/docs/book/src/reference/live-docs.md"
DERIVED_STATE_REGISTRY="$ROOT/doctrine/live_document_size/derived_state_contracts.jsonl"
DERIVED_STATE_CHECKER="$ROOT/scripts/check_derived_state_contracts.pl"
DERIVED_STATE_ADAPTER="$ROOT/scripts/check_derived_state_authorities.pl"
USER_POINTER="$ROOT/USER_GUIDE.md"
EXTRACTION_POINTER="$ROOT/EXTRACTION_ARCHITECTURE.md"
GRAPH_POINTER="$ROOT/KNOWLEDGE_GRAPH_ARCHITECTURE.md"
INTENT_POINTER="$ROOT/INTENTIR_SPEC.md"
ISF_CODE="$ROOT/crates/specforge/src/ir/isf_ir.rs"
ADAPTER_CODE="$ROOT/crates/specforge/src/ir/adapters.rs"
INTENT_CODE="$ROOT/crates/specforge/src/ir/intent.rs"
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

for path in \
  "$ACTOR_BOOK" "$SCOPE_BOOK" "$TEMPORAL_BOOK" "$SUMMARY" "$EXTRACTION_BOOK" \
  "$INTENT_CONTRACT_BOOK" "$LIVE_DOCS_BOOK" "$USER_POINTER" "$EXTRACTION_POINTER" "$GRAPH_POINTER" \
  "$INTENT_POINTER" "$ISF_CODE" "$ADAPTER_CODE" "$INTENT_CODE" "$CLI_CODE" "$DISPATCH_CODE" \
  "$DERIVED_STATE_REGISTRY" "$DERIVED_STATE_CHECKER" "$DERIVED_STATE_ADAPTER"
do
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

  require_literal "$INTENT_CODE" 'pub struct IntentIr {' \
    'the canonical IntentIR type seam is absent; reverify the product-boundary contract.'
  require_literal "$ADAPTER_CODE" 'pub enum AdapterTarget {' \
    'the adapter target enum is absent; reverify the adapter-boundary contract.'
  require_literal "$ADAPTER_CODE" '    Isf,' \
    'ISF is no longer the sole declared adapter target; reconcile the product contract and book.'
  require_literal "$INTENT_CONTRACT_BOOK" '`IntentIR` is the canonical product boundary of SpecForge.' \
    'the normative IntentIR book contract lost the canonical product boundary.'
  require_literal "$INTENT_CONTRACT_BOOK" 'SpecForge has one adapter target: `.isf`.' \
    'the normative IntentIR book contract lost the sole-adapter boundary.'
  require_literal "$EXTRACTION_BOOK" 'A specification distributes meaning across six evidence modalities.' \
    'the extraction book contract lost the complete evidence-modality boundary.'
  require_literal "$EXTRACTION_BOOK" 'Adapters lower canonical intent; they do not author missing source semantics.' \
    'the extraction book contract lost the adapter/non-invention boundary.'
  require_literal "$SUMMARY" '(reference/extraction-architecture.md)' \
    'SUMMARY.md lost the direct extraction-contract route.'
  require_literal "$SUMMARY" '(reference/intentir-contract.md)' \
    'SUMMARY.md lost the direct IntentIR-contract route.'
  require_literal "$LIVE_DOCS_BOOK" '## Containment and project-data locality are fully enforced' \
    'live-docs.md no longer states the completed containment/locality status.'
  require_literal "$LIVE_DOCS_BOOK" '#### Adoption program closed and verified' \
    'live-docs.md lost the closing implementation and verification subsection.'
  reject_literal "$LIVE_DOCS_BOOK" '## Containment doctrine active; migrations in progress' \
    'live-docs.md restored the stale migration-in-progress status.'
  reject_literal "$LIVE_DOCS_BOOK" 'Existing oversized ledgers are explicit transition debt' \
    'live-docs.md restored pre-migration ledger debt as current truth.'
  require_literal "$LIVE_DOCS_BOOK" 'Fourteen bounded contracts classify one derive-on-read field' \
    'live-docs.md lost the implemented derived-state contract count and class boundary.'
  require_literal "$DERIVED_STATE_REGISTRY" '"contract_id":"rust_prerequisite_copies"' \
    'the derived-state registry lost the Cargo-owned Rust prerequisite contract.'
  require_literal "$DERIVED_STATE_REGISTRY" '"contract_id":"fsmgen_gitlink_copies"' \
    'the derived-state registry lost the Git-index-owned FSMGen gitlink contract.'
  require_literal "$DERIVED_STATE_CHECKER" 'derive_on_read verified_copy authored_intent immutable_evidence' \
    'the neutral derived-state checker lost its closed four-class domain.'
  require_literal "$DERIVED_STATE_ADAPTER" "git', '-C', \$root, 'ls-files', '--stage', '--', 'subs/fsmgen'" \
    'the local derived-state adapter no longer derives the FSMGen object from the Git index.'

  require_literal "$USER_POINTER" '(docs/book/src/SUMMARY.md)' \
    'USER_GUIDE.md is no longer a direct compatibility route to the maintained book.'
  require_literal "$EXTRACTION_POINTER" '(docs/book/src/reference/extraction-architecture.md)' \
    'EXTRACTION_ARCHITECTURE.md lost its canonical book-contract route.'
  require_literal "$GRAPH_POINTER" '(docs/book/src/domain/actor-connectivity.md)' \
    'KNOWLEDGE_GRAPH_ARCHITECTURE.md lost its structural graph route.'
  require_literal "$GRAPH_POINTER" '(docs/book/src/domain/temporal-semantics.md)' \
    'KNOWLEDGE_GRAPH_ARCHITECTURE.md lost its temporal-semantics route.'
  require_literal "$INTENT_POINTER" '(docs/book/src/reference/intentir-contract.md)' \
    'INTENTIR_SPEC.md lost its normative book-contract route.'

  for path in "$USER_POINTER" "$EXTRACTION_POINTER" "$GRAPH_POINTER" "$INTENT_POINTER"; do
    reject_literal "$path" '## Current limitation' \
      "${path#"$ROOT"/} restored a current-state mirror instead of remaining a bounded pointer."
    reject_literal "$path" '## Implementation sequence' \
      "${path#"$ROOT"/} restored an implementation-plan mirror instead of remaining a bounded pointer."
    reject_literal "$path" '## Pipeline validation results' \
      "${path#"$ROOT"/} restored a validation snapshot instead of linking the reviewed authority."
    reject_literal "$path" 'Current implementation note:' \
      "${path#"$ROOT"/} restored implementation detail instead of linking the maintained book."
  done
fi

if [ "$fail" -ne 0 ]; then
  printf 'book-current-truth: FAILED — reverify code first, then keep the canonical mdBook statement and pointer aligned.\n' >&2
  exit 1
fi

printf 'book-current-truth: root pointers, product contracts, containment status, actor direction, and constrained extraction match code and canonical book routes.\n'
