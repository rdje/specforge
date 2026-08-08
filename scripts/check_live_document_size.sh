#!/usr/bin/env bash
# SpecForge adapter for the project-neutral live-document containment doctrine.
set -uo pipefail

ADAPTER_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ADAPTER_ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ADAPTER_ROOT"
ROOT="$ADAPTER_ROOT"
if [ "$#" -gt 0 ]; then
  if [ "$#" -ne 2 ] || [ "$1" != '--root' ] || [ ! -d "$2" ]; then
    printf 'Usage: %s [--root PROJECT_ROOT]\n' "$0" >&2
    exit 2
  fi
  ROOT="$(cd "$2" && pwd)"
fi

fail=0
if [ -x "$ROOT/scripts/check_readme_policy.sh" ]; then
  "$ROOT/scripts/check_readme_policy.sh" || fail=1
else
  printf 'live-document-size: required README route guard is missing or non-executable\n' >&2
  fail=1
fi

if [ "$ROOT" = "$ADAPTER_ROOT" ]; then
  "$ROOT/scripts/check_readme_policy.sh" --self-test || fail=1
  perl "$ROOT/scripts/test_live_document_size.pl" --quiet || fail=1
  perl "$ROOT/scripts/check_rolling_ledger_protocol.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_roadmap_projection_contract.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_fsmgen_feedback_protocol.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_validation_snapshot_currentness.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_source_pdf_registry_currentness.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_corpus_kb_currentness.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_task_tree_catalog.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_task_tree_catalog.pl" --check || fail=1
  perl "$ROOT/scripts/check_fact_card_catalog.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_fact_card_catalog.pl" --check || fail=1
  perl "$ROOT/scripts/check_canonical_collection_catalogs.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_canonical_collection_catalogs.pl" --check || fail=1
  perl "$ROOT/scripts/check_knowledge_map_shard_contract.pl" --self-test || fail=1
  perl "$ROOT/scripts/check_knowledge_map_shard_contract.pl" --check || fail=1
  bash "$ROOT/scripts/test_knowledge_map_projection.sh" || fail=1
fi

perl "$ROOT/scripts/check_rolling_ledger_protocol.pl" \
  --root "$ROOT" \
  --registry doctrine/live_document_size/rolling_ledgers.jsonl || fail=1

perl "$ROOT/scripts/check_roadmap_projection_contract.pl" \
  --root "$ROOT" \
  --contract doctrine/live_document_size/roadmap_projection.json \
  --check || fail=1

perl "$ROOT/scripts/check_fsmgen_feedback_protocol.pl" \
  --root "$ROOT" \
  --contract doctrine/live_document_size/fsmgen_feedback.json \
  --check || fail=1

perl "$ROOT/scripts/check_corpus_kb_currentness.pl" \
  --root "$ROOT" \
  --contract doctrine/live_document_size/corpus_kb.json \
  --check || fail=1

perl "$ROOT/scripts/check_live_document_size.pl" \
  --root "$ROOT" \
  --registry doctrine/live_document_size/surfaces.jsonl \
  --routes doctrine/readme_entrypoint/routed_destinations.tsv \
  --authorities doctrine/live_document_size/ceiling_increase_authorities.jsonl || fail=1

exit "$fail"
