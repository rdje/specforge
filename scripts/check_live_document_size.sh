#!/usr/bin/env bash
# SpecForge adapter for the project-neutral live-document containment doctrine.
set -uo pipefail

ADAPTER_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
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
fi

perl "$ROOT/scripts/check_live_document_size.pl" \
  --root "$ROOT" \
  --registry doctrine/live_document_size/surfaces.jsonl \
  --routes doctrine/readme_entrypoint/routed_destinations.tsv \
  --authorities doctrine/live_document_size/ceiling_increase_authorities.jsonl || fail=1

exit "$fail"
