#!/usr/bin/env bash
# Check every registered active-task-evidence contract. One list, so a registry freshness hook and
# the composed live-size gate cannot disagree about which contracts exist.
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

CONTRACTS=(
  doctrine/live_document_size/active_task_evidence.json
  doctrine/live_document_size/corpus_task_evidence.json
  doctrine/live_document_size/spec_to_intent_task_evidence.json
  doctrine/live_document_size/claim_verification_task_evidence.json
)

fail=0
for contract in "${CONTRACTS[@]}"; do
  perl "$ROOT/scripts/check_active_task_evidence.pl" \
    --root "$ROOT" \
    --contract "$contract" \
    --check || fail=1
done

exit "$fail"
