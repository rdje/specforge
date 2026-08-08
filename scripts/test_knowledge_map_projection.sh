#!/usr/bin/env bash
# Repository-local integration tests for the portable Knowledge Map projection-set bundle.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fixture="$ROOT/generated/knowledge-map-projection-tests.$$"
case "$fixture" in "$ROOT"/generated/knowledge-map-projection-tests.[0-9]*) ;; *) exit 1 ;; esac
cleanup() { rm -rf "$fixture"; }
trap cleanup EXIT HUP INT TERM

fail() { printf 'knowledge-map-projection-tests: %s\n' "$1" >&2; exit 1; }
expect_failure() {
  if "$@" >/dev/null 2>&1; then fail "expected failure: $*"; fi
}
write_config() {
  printf ': "${KM_TARGET_SHARD_LINES:=%s}"\n: "${KM_MAX_PROJECTION_LINES:=%s}"\n' "$1" "$2" \
    > "$fixture/.knowledge_map.conf"
}
write_beta() {
  extra="${1:-}"
  {
    printf '%s\n' '---' 'id: beta' 'title: Beta fact' 'answers:'
    printf '%s\n' '  - "what is beta"' '  - "where is beta"'
    [ -z "$extra" ] || printf '  - "%s"\n' "$extra"
    printf '%s\n' 'date: 2026-08-08' 'evidence: docs/knowledge/beta.md' '---' '' 'Beta.'
  } > "$fixture/docs/knowledge/beta.md"
}

mkdir -p "$fixture/knowledge-map/scripts" "$fixture/docs/knowledge" "$fixture/generated"
cp "$ROOT/knowledge-map/scripts/gen_knowledge_map.sh" "$fixture/knowledge-map/scripts/"
cp "$ROOT/knowledge-map/scripts/check_knowledge_map.sh" "$fixture/knowledge-map/scripts/"
cp "$ROOT/knowledge-map/scripts/knowledge_map.conf" "$fixture/knowledge-map/scripts/"
chmod +x "$fixture/knowledge-map/scripts/gen_knowledge_map.sh" \
  "$fixture/knowledge-map/scripts/check_knowledge_map.sh"
git -C "$fixture" init -q

printf '%s\n' '---' 'id: alpha' 'title: Alpha fact' 'answers:' \
  '  - "what is alpha"' '  - "where is alpha"' '  - "how does alpha work"' \
  'date: 2026-08-08' 'evidence: docs/knowledge/alpha.md' '---' '' 'Alpha.' \
  > "$fixture/docs/knowledge/alpha.md"
write_beta
write_config 8 128

(cd "$fixture" && bash knowledge-map/scripts/gen_knowledge_map.sh >/dev/null)
[ "$(find "$fixture/docs/knowledge-map" -type f -name 'questions-*.md' | wc -l | tr -d ' ')" -eq 3 ] \
  || fail "bounded rollover did not create three shards"
[ "$(wc -l < "$fixture/KNOWLEDGE_MAP.md" | tr -d ' ')" -le 96 ] || fail "landing is not bounded"

(cd "$fixture" && bash knowledge-map/scripts/check_knowledge_map.sh >/dev/null)

printf '\nDRIFT\n' >> "$fixture/docs/knowledge-map/questions-0001.md"
expect_failure bash -c "cd '$fixture' && bash knowledge-map/scripts/check_knowledge_map.sh"
(cd "$fixture" && bash knowledge-map/scripts/gen_knowledge_map.sh >/dev/null)

root_before="$(cksum "$fixture/KNOWLEDGE_MAP.md")"
write_beta 'what is alpha'
expect_failure bash -c "cd '$fixture' && bash knowledge-map/scripts/gen_knowledge_map.sh"
[ "$(cksum "$fixture/KNOWLEDGE_MAP.md")" = "$root_before" ] \
  || fail "collision failure replaced a committed output"
write_beta

git -C "$fixture" add KNOWLEDGE_MAP.md docs/knowledge-map
write_config 32 128
(cd "$fixture" && bash knowledge-map/scripts/gen_knowledge_map.sh >/dev/null)
[ ! -e "$fixture/docs/knowledge-map/questions-0002.md" ] \
  && [ ! -e "$fixture/docs/knowledge-map/questions-0003.md" ] \
  || fail "obsolete generated shards were not removed"

output_paths="$(cd "$fixture" && bash knowledge-map/scripts/gen_knowledge_map.sh --print-output-paths)"
printf '%s\n' "$output_paths" | grep -qx 'docs/knowledge-map/questions-0002.md' \
  || fail "tracked shard deletion was omitted from the staging path set"

root_before="$(cksum "$fixture/KNOWLEDGE_MAP.md")"
write_config 32 5
expect_failure bash -c "cd '$fixture' && bash knowledge-map/scripts/gen_knowledge_map.sh"
[ "$(cksum "$fixture/KNOWLEDGE_MAP.md")" = "$root_before" ] \
  || fail "aggregate-bound failure replaced a committed output"
write_config 32 128

printf '# stale\n' > "$fixture/docs/knowledge-map/questions-invalid.md"
expect_failure bash -c "cd '$fixture' && bash knowledge-map/scripts/check_knowledge_map.sh"
rm -f "$fixture/docs/knowledge-map/questions-invalid.md"
(cd "$fixture" && bash knowledge-map/scripts/check_knowledge_map.sh >/dev/null)

if find "$fixture/docs/knowledge-map" -maxdepth 1 -type f -name '.*.tmp' | grep -q .; then
  fail "generator temporary residue remains"
fi
if find "$fixture/generated" -maxdepth 1 -name '.knowledge-map-check.*' | grep -q .; then
  fail "checker temporary residue remains"
fi

printf 'knowledge-map-projection-tests: 8/8 generation, drift, collision, cleanup, staging, bound, name, and residue checks pass.\n'
