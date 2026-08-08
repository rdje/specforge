#!/usr/bin/env bash
# check_knowledge_map.sh — validate facts and derive-and-diff the complete projection set.
# Portable runtime: Bash 3.2+, POSIX awk/find/sort/diff, and the paired generator.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || pwd)"

[ -f "$ROOT/.knowledge_map.conf" ] && . "$ROOT/.knowledge_map.conf"
[ -f "$SCRIPT_DIR/knowledge_map.conf" ] && . "$SCRIPT_DIR/knowledge_map.conf"
: "${KM_SCAN_DIRS:=docs/knowledge docs/decisions}"
: "${KM_OUTPUT:=KNOWLEDGE_MAP.md}"
: "${KM_SHARD_DIR:=docs/knowledge-map}"
: "${KM_SHARD_PREFIX:=questions-}"

cd "$ROOT"
fail=0
note() { printf 'knowledge-map: %s\n' "$1" >&2; fail=1; }

check_rel="generated/.knowledge-map-check.$$"
check_abs="$ROOT/$check_rel"
case "$check_abs" in
  "$ROOT"/generated/.knowledge-map-check.[0-9]*) ;;
  *) printf 'knowledge-map: unsafe check workspace\n' >&2; exit 1 ;;
esac
[ ! -e "$check_abs" ] || { printf 'knowledge-map: check workspace already exists\n' >&2; exit 1; }
mkdir -p "$check_abs"
cleanup() { rm -rf "$check_abs"; }
trap cleanup EXIT HUP INT TERM

ids_file="$check_abs/fact-ids.tsv"
: > "$ids_file"

# Validate the constrained front matter before asking the generator to render it.
for directory in $KM_SCAN_DIRS; do
  [ -d "$directory" ] || continue
  for file in "$directory"/*.md; do
    [ -f "$file" ] || continue
    result="$(LC_ALL=C awk '
      BEGIN { infm=0; started=0; hasans=0; curkey="" }
      NR==1 { if ($0 ~ /^---[ \t]*$/) { infm=1; started=1; next } }
      infm==1 && $0 ~ /^---[ \t]*$/ { infm=0; next }
      infm==1 {
        if ($0 ~ /^[ \t]+-[ \t]+/) {
          if (curkey=="answers") { value=$0; sub(/^[ \t]+-[ \t]+/,"",value); if (value!="") hasans=1 }
          next
        }
        if (match($0, /^[A-Za-z_][A-Za-z0-9_]*:/)) {
          key=substr($0,1,RLENGTH-1); rest=substr($0,RLENGTH+1)
          gsub(/^[ \t]+/,"",rest); gsub(/[ \t]+$/,"",rest); curkey=key
          if (key=="answers" && rest ~ /\[.*[^][ \t].*\]/) hasans=1
          if (key=="id" && rest!="") { gsub(/^"|"$/,"",rest); vid=rest }
          if (key=="title" && rest!="") okt=1
          if (key=="date" && rest ~ /^[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]$/) okd=1
          if (key=="evidence" && rest!="") oke=1
          if (key=="reverify" && rest!="") okr=1
          if (rest!="") curkey=""
        }
      }
      END {
        if (!started || !hasans) { print "NOTFACT"; exit }
        missing=""
        if (vid=="") missing=missing " id"
        if (!okt) missing=missing " title"
        if (!okd) missing=missing " date"
        if (!oke && !okr) missing=missing " evidence-or-reverify"
        print "FACT\t" vid "\t" missing
      }
    ' "$file")"
    case "$result" in
      FACT*)
        fact_id="$(printf '%s' "$result" | cut -f2)"
        missing="$(printf '%s' "$result" | cut -f3)"
        [ -n "$fact_id" ] && printf '%s\t%s\n' "$fact_id" "$file" >> "$ids_file"
        [ -z "$(printf '%s' "$missing" | tr -d ' ')" ] \
          || note "$file: KM fact missing required front matter:$missing"
        ;;
      *) : ;;
    esac
  done
done

if [ -s "$ids_file" ]; then
  duplicate_ids="$(cut -f1 "$ids_file" | LC_ALL=C sort | uniq -d)"
  if [ -n "$duplicate_ids" ]; then
    while IFS= read -r duplicate; do
      [ -n "$duplicate" ] && note "duplicate fact id '$duplicate'"
    done <<EOF
$duplicate_ids
EOF
  fi
fi

# Regenerate the same logical paths beneath a repository-local comparison root.
if ! KM_DEST_ROOT="$check_rel" bash "$SCRIPT_DIR/gen_knowledge_map.sh" >/dev/null 2>&1; then
  note "gen_knowledge_map.sh failed (invalid inputs, collision, or output bound)"
else
  expected_root="$check_abs/$KM_OUTPUT"
  if [ ! -f "$KM_OUTPUT" ]; then
    note "$KM_OUTPUT does not exist — run knowledge-map/scripts/gen_knowledge_map.sh"
  elif ! diff -q "$KM_OUTPUT" "$expected_root" >/dev/null 2>&1; then
    note "$KM_OUTPUT is out of sync — regenerate and commit the complete projection set"
  fi

  actual_names="$check_abs/actual-shards.txt"
  expected_names="$check_abs/expected-shards.txt"
  : > "$actual_names"; : > "$expected_names"
  if [ -d "$KM_SHARD_DIR" ]; then
    find "$KM_SHARD_DIR" -maxdepth 1 -type f -name "${KM_SHARD_PREFIX}*.md" -exec basename {} \; \
      | LC_ALL=C sort > "$actual_names"
  fi
  expected_dir="$check_abs/$KM_SHARD_DIR"
  if [ -d "$expected_dir" ]; then
    find "$expected_dir" -maxdepth 1 -type f -name "${KM_SHARD_PREFIX}*.md" -exec basename {} \; \
      | LC_ALL=C sort > "$expected_names"
  fi
  while IFS= read -r name; do
    case "$name" in
      "$KM_SHARD_PREFIX"[0-9][0-9][0-9][0-9].md) ;;
      *) note "unexpected generated shard name '$KM_SHARD_DIR/$name'" ;;
    esac
  done < "$actual_names"
  if ! diff -q "$actual_names" "$expected_names" >/dev/null 2>&1; then
    note "generated shard membership is stale — regenerate the complete projection set"
  else
    while IFS= read -r name; do
      [ -n "$name" ] || continue
      if ! diff -q "$KM_SHARD_DIR/$name" "$expected_dir/$name" >/dev/null 2>&1; then
        note "$KM_SHARD_DIR/$name is out of sync with canonical questions"
      fi
    done < "$expected_names"
  fi
fi

[ "$fail" -eq 0 ] && printf 'knowledge-map: OK (facts valid; ids/questions unique; bounded landing/shards in sync)\n' >&2
exit "$fail"
