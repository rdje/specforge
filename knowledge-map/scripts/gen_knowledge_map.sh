#!/usr/bin/env bash
# gen_knowledge_map.sh — derive a bounded question-keyed Knowledge Map projection set.
#
# The output is deterministic and generated: a small landing page plus sequential question
# shards. Canonical fact prose and verification metadata stay in the linked source files.
# Portable runtime: Bash 3.2+, POSIX awk/sort/find, and one SHA-256 provider.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || pwd)"

[ -f "$ROOT/.knowledge_map.conf" ] && . "$ROOT/.knowledge_map.conf"
[ -f "$SCRIPT_DIR/knowledge_map.conf" ] && . "$SCRIPT_DIR/knowledge_map.conf"
: "${KM_SCAN_DIRS:=docs/knowledge docs/decisions}"
: "${KM_OUTPUT:=KNOWLEDGE_MAP.md}"
: "${KM_TITLE:=Knowledge Map}"
: "${KM_SHARD_DIR:=docs/knowledge-map}"
: "${KM_SHARD_PREFIX:=questions-}"
: "${KM_FACT_CATALOG:=}"
: "${KM_MAX_FACTS:=200}"
: "${KM_MAX_QUESTION_KEYS:=2048}"
: "${KM_MAX_QUESTION_BYTES:=2048}"
: "${KM_MAX_ID_BYTES:=96}"
: "${KM_MAX_PATH_BYTES:=192}"
: "${KM_MAX_LINK_LINE_BYTES:=512}"
: "${KM_WRAP_LINE_BYTES:=384}"
: "${KM_TARGET_SHARD_LINES:=300}"
: "${KM_TARGET_SHARD_BYTES:=40960}"
: "${KM_MAX_SHARD_LINES:=384}"
: "${KM_MAX_SHARD_BYTES:=49152}"
: "${KM_MAX_SHARDS:=32}"
: "${KM_MAX_PROJECTION_LINES:=4096}"
: "${KM_MAX_PROJECTION_BYTES:=393216}"
: "${KM_MAX_LANDING_LINES:=96}"
: "${KM_MAX_LANDING_BYTES:=8192}"
: "${KM_MAX_LANDING_LINE_BYTES:=256}"
: "${KM_DEST_ROOT:=}"

fail() { printf 'knowledge-map: %s\n' "$1" >&2; exit 1; }
TAB_CHAR="$(printf '\t')"

safe_relative_path() {
  case "$1" in
    ''|/*|*\\*|../*|*/../*|*/..|..|*"$TAB_CHAR"*|*"
"*) return 1 ;;
  esac
  return 0
}

positive_integer() {
  case "$2" in ''|*[!0-9]*|0) fail "$1 must be a positive integer" ;; esac
}

for value in "$KM_OUTPUT" "$KM_SHARD_DIR"; do
  safe_relative_path "$value" || fail "unsafe generated output path '$value'"
done
[ -z "$KM_FACT_CATALOG" ] || safe_relative_path "$KM_FACT_CATALOG" \
  || fail "unsafe fact-catalog path '$KM_FACT_CATALOG'"
[ -z "$KM_DEST_ROOT" ] || safe_relative_path "$KM_DEST_ROOT" \
  || fail "unsafe destination root '$KM_DEST_ROOT'"
case "$KM_SHARD_PREFIX" in
  ''|*[!a-z0-9-]*|[!a-z0-9]*) fail "invalid shard prefix '$KM_SHARD_PREFIX'" ;;
esac
for pair in \
  "KM_MAX_FACTS:$KM_MAX_FACTS" \
  "KM_MAX_QUESTION_KEYS:$KM_MAX_QUESTION_KEYS" \
  "KM_MAX_QUESTION_BYTES:$KM_MAX_QUESTION_BYTES" \
  "KM_MAX_ID_BYTES:$KM_MAX_ID_BYTES" \
  "KM_MAX_PATH_BYTES:$KM_MAX_PATH_BYTES" \
  "KM_MAX_LINK_LINE_BYTES:$KM_MAX_LINK_LINE_BYTES" \
  "KM_WRAP_LINE_BYTES:$KM_WRAP_LINE_BYTES" \
  "KM_TARGET_SHARD_LINES:$KM_TARGET_SHARD_LINES" \
  "KM_TARGET_SHARD_BYTES:$KM_TARGET_SHARD_BYTES" \
  "KM_MAX_SHARD_LINES:$KM_MAX_SHARD_LINES" \
  "KM_MAX_SHARD_BYTES:$KM_MAX_SHARD_BYTES" \
  "KM_MAX_SHARDS:$KM_MAX_SHARDS" \
  "KM_MAX_PROJECTION_LINES:$KM_MAX_PROJECTION_LINES" \
  "KM_MAX_PROJECTION_BYTES:$KM_MAX_PROJECTION_BYTES" \
  "KM_MAX_LANDING_LINES:$KM_MAX_LANDING_LINES" \
  "KM_MAX_LANDING_BYTES:$KM_MAX_LANDING_BYTES" \
  "KM_MAX_LANDING_LINE_BYTES:$KM_MAX_LANDING_LINE_BYTES"
do
  positive_integer "${pair%%:*}" "${pair#*:}"
done
[ "$KM_WRAP_LINE_BYTES" -le "$KM_MAX_LINK_LINE_BYTES" ] \
  || fail "KM_WRAP_LINE_BYTES exceeds KM_MAX_LINK_LINE_BYTES"
[ "$KM_TARGET_SHARD_LINES" -le "$KM_MAX_SHARD_LINES" ] \
  && [ "$KM_TARGET_SHARD_BYTES" -le "$KM_MAX_SHARD_BYTES" ] \
  || fail "target shard bounds exceed hard shard bounds"

physical_path() {
  if [ -n "$KM_DEST_ROOT" ]; then
    printf '%s/%s/%s\n' "$ROOT" "$KM_DEST_ROOT" "$1"
  else
    printf '%s/%s\n' "$ROOT" "$1"
  fi
}

print_output_paths() {
  {
    printf '%s\n' "$KM_OUTPUT"
    shard_abs="$(physical_path "$KM_SHARD_DIR")"
    if [ -d "$shard_abs" ]; then
      find "$shard_abs" -maxdepth 1 -type f -name "${KM_SHARD_PREFIX}[0-9][0-9][0-9][0-9].md" \
        -exec basename {} \; | while IFS= read -r name; do
          printf '%s/%s\n' "$KM_SHARD_DIR" "$name"
        done
    fi
    if [ -z "$KM_DEST_ROOT" ]; then
      git -C "$ROOT" ls-files -- "${KM_SHARD_DIR}/${KM_SHARD_PREFIX}*.md"
    fi
  } | LC_ALL=C sort -u
}

case "${1:-}" in
  --print-map-path) printf '%s\n' "$KM_OUTPUT"; exit 0 ;;
  --print-output-paths) print_output_paths; exit 0 ;;
  --print-contract)
    printf '{"landing_path":"%s","shard_directory":"%s","shard_prefix":"%s","fact_catalog":"%s","limits":{' \
      "$KM_OUTPUT" "$KM_SHARD_DIR" "$KM_SHARD_PREFIX" "$KM_FACT_CATALOG"
    printf '"max_facts":%s,"max_question_keys":%s,"max_question_bytes":%s,' \
      "$KM_MAX_FACTS" "$KM_MAX_QUESTION_KEYS" "$KM_MAX_QUESTION_BYTES"
    printf '"max_id_bytes":%s,"max_path_bytes":%s,"max_link_line_bytes":%s,' \
      "$KM_MAX_ID_BYTES" "$KM_MAX_PATH_BYTES" "$KM_MAX_LINK_LINE_BYTES"
    printf '"wrap_line_bytes":%s,"target_shard_lines":%s,"target_shard_bytes":%s,' \
      "$KM_WRAP_LINE_BYTES" "$KM_TARGET_SHARD_LINES" "$KM_TARGET_SHARD_BYTES"
    printf '"max_shard_lines":%s,"max_shard_bytes":%s,' \
      "$KM_MAX_SHARD_LINES" "$KM_MAX_SHARD_BYTES"
    printf '"max_shards":%s,"max_projection_lines":%s,"max_projection_bytes":%s,' \
      "$KM_MAX_SHARDS" "$KM_MAX_PROJECTION_LINES" "$KM_MAX_PROJECTION_BYTES"
    printf '"max_landing_lines":%s,"max_landing_bytes":%s,' \
      "$KM_MAX_LANDING_LINES" "$KM_MAX_LANDING_BYTES"
    printf '"max_landing_line_bytes":%s}}\n' "$KM_MAX_LANDING_LINE_BYTES"
    exit 0
    ;;
  '') ;;
  *) fail "usage: $0 [--print-map-path|--print-output-paths|--print-contract]" ;;
esac

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 "$1" | awk '{print $NF}'
  else
    fail "no SHA-256 provider found (need sha256sum, shasum, or openssl)"
  fi
}

relative_path_from() {
  LC_ALL=C awk -v from="$1" -v target="$2" '
    BEGIN {
      nf = (from == "." || from == "") ? 0 : split(from, f, "/")
      nt = split(target, t, "/")
      common = 0
      while (common < nf && common < nt && f[common + 1] == t[common + 1]) common++
      out = ""
      for (i = common + 1; i <= nf; i++) out = out "../"
      for (i = common + 1; i <= nt; i++) out = out (i > common + 1 ? "/" : "") t[i]
      print (out == "" ? "." : out)
    }
  '
}

extract_one() {
  LC_ALL=C awk -v rel="$1" -v link="$2" \
    -v maxq="$KM_MAX_QUESTION_BYTES" -v maxi="$KM_MAX_ID_BYTES" '
    BEGIN { infm=0; started=0; nans=0; curkey="" }
    NR==1 { if ($0 ~ /^---[ \t]*$/) { infm=1; started=1; next } }
    infm==1 && $0 ~ /^---[ \t]*$/ { infm=0; next }
    infm==1 {
      if ($0 ~ /^[ \t]+-[ \t]+/) {
        v=$0; sub(/^[ \t]+-[ \t]+/,"",v); v=strip(v)
        if (curkey=="answers" && v!="") ans[++nans]=v
        next
      }
      if (match($0, /^[A-Za-z_][A-Za-z0-9_]*:/)) {
        key=substr($0,1,RLENGTH-1); rest=strip(substr($0,RLENGTH+1)); curkey=key
        if (rest ~ /^\[.*\]$/) {
          inner=substr(rest,2,length(rest)-2); n=split(inner,a,",")
          for (i=1;i<=n;i++) { v=strip(a[i]); if (key=="answers" && v!="") ans[++nans]=v }
          curkey=""
        } else if (rest!="") { vals[key]=rest; curkey="" }
      }
    }
    END {
      if (!started || nans==0) exit 0
      id=vals["id"]
      if (id=="" || id !~ /^[a-z0-9][a-z0-9-]*$/) die("unsafe or missing fact id in " rel)
      if (length(id)>maxi) die("fact id exceeds byte bound in " rel)
      for (i=1;i<=nans;i++) {
        if (index(ans[i], "\t")) die("question contains a tab in " rel)
        if (length(ans[i])>maxq) die("question exceeds byte bound in " rel)
        print "Q\t" ans[i] "\t" id "\t" link
      }
      print "F\t" id "\t" rel
    }
    function strip(s) {
      gsub(/^[ \t]+/,"",s); gsub(/[ \t]+$/,"",s)
      if ((s ~ /^".*"$/) || (s ~ /^\047.*\047$/)) s=substr(s,2,length(s)-2)
      return s
    }
    function die(message) { print "knowledge-map: " message > "/dev/stderr"; exit 2 }
  ' "$3"
}

metrics() {
  LC_ALL=C awk '
    { lines++; bytes += length($0) + 1; if (length($0) > width) width=length($0) }
    END { printf "%d\t%d\t%d\n", lines+0, bytes+0, width+0 }
  ' "$1"
}

cd "$ROOT"
output_root="$(physical_path "$KM_OUTPUT")"
output_shard_dir="$(physical_path "$KM_SHARD_DIR")"
mkdir -p "$(dirname "$output_root")" "$output_shard_dir"

pid="$$"
rows="$output_shard_dir/.${KM_SHARD_PREFIX}rows.$pid.tmp"
paths="$output_shard_dir/.${KM_SHARD_PREFIX}paths.$pid.tmp"
paths_sorted="$output_shard_dir/.${KM_SHARD_PREFIX}paths-sorted.$pid.tmp"
part="$output_shard_dir/.${KM_SHARD_PREFIX}part.$pid.tmp"
identity="$output_shard_dir/.${KM_SHARD_PREFIX}identity.$pid.tmp"
questions="$output_shard_dir/.${KM_SHARD_PREFIX}questions.$pid.tmp"
manifest="$output_shard_dir/.${KM_SHARD_PREFIX}manifest.$pid.tmp"
root_tmp="$(dirname "$output_root")/.$(basename "$output_root").stage.$pid.tmp"

cleanup() {
  rm -f "$rows" "$paths" "$paths_sorted" "$part" "$identity" "$questions" "$manifest" "$root_tmp"
  find "$output_shard_dir" -maxdepth 1 -type f -name ".${KM_SHARD_PREFIX}stage.$pid-*.tmp" \
    -exec rm -f {} \; 2>/dev/null || true
}
trap cleanup EXIT HUP INT TERM
: > "$rows"; : > "$paths"; : > "$identity"

for directory in $KM_SCAN_DIRS; do
  safe_relative_path "$directory" || fail "unsafe scan directory '$directory'"
  [ -d "$directory" ] || continue
  for file in "$directory"/*.md; do
    [ -f "$file" ] || continue
    safe_relative_path "$file" || fail "unsafe fact path '$file'"
    [ "$(printf '%s' "$file" | LC_ALL=C wc -c | tr -d ' ')" -le "$KM_MAX_PATH_BYTES" ] \
      || fail "fact path exceeds byte bound: $file"
    printf '%s\n' "$file" >> "$paths"
  done
done
LC_ALL=C sort -u "$paths" > "$paths_sorted"

while IFS= read -r rel; do
  [ -n "$rel" ] || continue
  link="$(relative_path_from "$KM_SHARD_DIR" "$rel")"
  [ "$(printf '%s' "- [x]($link)" | LC_ALL=C wc -c | tr -d ' ')" -le "$KM_MAX_LINK_LINE_BYTES" ] \
    || fail "fact link exceeds line bound: $rel"
  : > "$part"
  extract_one "$rel" "$link" "$rel" > "$part" || fail "cannot parse fact '$rel'"
  if LC_ALL=C grep -q "^F$(printf '\t')" "$part"; then
    cat "$part" >> "$rows"
    printf '%s\0%s\n' "$rel" "$(sha256_file "$rel")" >> "$identity"
  fi
done < "$paths_sorted"

tab="$(printf '\t')"
facts="$(LC_ALL=C awk -F "$tab" '$1=="F"{n++} END{print n+0}' "$rows")"
[ "$facts" -le "$KM_MAX_FACTS" ] || fail "fact count $facts exceeds $KM_MAX_FACTS"
duplicate_id="$(LC_ALL=C awk -F "$tab" '$1=="F"{print $2}' "$rows" | LC_ALL=C sort | uniq -d | head -1)"
[ -z "$duplicate_id" ] || fail "duplicate fact id '$duplicate_id'"

LC_ALL=C awk -F "$tab" '$1=="Q"{print $2 "\t" $3 "\t" $4}' "$rows" \
  | LC_ALL=C sort -t "$tab" -k1,1 -k2,2 -k3,3 > "$questions"
question_keys="$(LC_ALL=C wc -l < "$questions" | tr -d ' ')"
[ "$question_keys" -le "$KM_MAX_QUESTION_KEYS" ] \
  || fail "question-key count $question_keys exceeds $KM_MAX_QUESTION_KEYS"
collision="$(LC_ALL=C awk -F "$tab" '
  NR>1 && $1==previous { print previous_id "\t" $2 "\t" $1; exit }
  { previous=$1; previous_id=$2 }
' "$questions")"
if [ -n "$collision" ]; then
  fail "question collision between '$(printf '%s' "$collision" | cut -f1)' and '$(printf '%s' "$collision" | cut -f2)': $(printf '%s' "$collision" | cut -f3-)"
fi

[ -s "$identity" ] || printf '\n' > "$identity"
canonical_sha="$(sha256_file "$identity")"
: > "$manifest"

LC_ALL=C awk -F "$tab" -v outdir="$output_shard_dir" -v prefix="$KM_SHARD_PREFIX" \
  -v pid="$pid" -v manifest="$manifest" -v wrap="$KM_WRAP_LINE_BYTES" \
  -v targetlines="$KM_TARGET_SHARD_LINES" -v targetbytes="$KM_TARGET_SHARD_BYTES" \
  -v maxlines="$KM_MAX_SHARD_LINES" -v maxbytes="$KM_MAX_SHARD_BYTES" '
  function begin_shard(    header) {
    shard++
    stage=sprintf(".%sstage.%s-%04d.tmp",prefix,pid,shard)
    final=sprintf("%s%04d.md",prefix,shard)
    path=outdir "/" stage
    header="# Knowledge questions — shard " sprintf("%04d",shard)
    notice="> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter."
    print header > path; print "" > path
    print notice > path
    print "" > path
    lines=4; bytes=length(header)+1+1+length(notice)+1+1; keys=0
  }
  function finish_shard() {
    if (keys==0) return
    print shard "\t" keys "\t" lines "\t" bytes "\t" stage "\t" final >> manifest
    close(path)
  }
  function prepare_entry(question,id,link,    n,i,word,candidate) {
    entry_n=1; entry[1]="- [" id "](" link ")"
    if (length(entry[1])>maxlink) die("question link line exceeds bound")
    n=split(question,words,/ +/); line="  >"
    for (i=1;i<=n;i++) {
      word=words[i]
      if (length("  > " word)>wrap) die("question token exceeds wrap bound")
      candidate=line " " word
      if (length(candidate)>wrap) { entry[++entry_n]=line; line="  > " word }
      else line=candidate
    }
    if (line!="  >") entry[++entry_n]=line
    entry_bytes=0
    for (i=1;i<=entry_n;i++) entry_bytes+=length(entry[i])+1
  }
  function die(message) { print "knowledge-map: " message > "/dev/stderr"; exit 2 }
  BEGIN { shard=0; maxlink=ARGV[1]; ARGV[1]="" }
  {
    if (shard==0) begin_shard()
    prepare_entry($1,$2,$3)
    if (keys>0 && (lines+entry_n>targetlines || bytes+entry_bytes>targetbytes)) {
      finish_shard(); begin_shard()
    }
    if (lines+entry_n>maxlines || bytes+entry_bytes>maxbytes) die("one question entry exceeds shard bounds")
    for (i=1;i<=entry_n;i++) print entry[i] > path
    lines+=entry_n; bytes+=entry_bytes; keys++
  }
  END { if (shard>0) finish_shard() }
' "$KM_MAX_LINK_LINE_BYTES" "$questions" || fail "cannot render bounded question shards"

shards="$(LC_ALL=C wc -l < "$manifest" | tr -d ' ')"
[ "$shards" -le "$KM_MAX_SHARDS" ] || fail "shard count $shards exceeds $KM_MAX_SHARDS"
shard_lines="$(LC_ALL=C awk -F "$tab" '{sum+=$3} END{print sum+0}' "$manifest")"
shard_bytes="$(LC_ALL=C awk -F "$tab" '{sum+=$4} END{print sum+0}' "$manifest")"

landing_dir="$(dirname "$KM_OUTPUT")"
{
  printf '# %s\n\n' "$KM_TITLE"
  printf '> **AUTO-GENERATED — DO NOT EDIT.** Canonical facts live in front-mattered source files.\n\n'
  printf -- '- Facts: **%s**\n' "$facts"
  printf -- '- Unique question keys: **%s**\n' "$question_keys"
  printf -- '- Canonical input SHA-256: `%s`\n' "$canonical_sha"
  if [ -n "$KM_FACT_CATALOG" ]; then
    fact_link="$(relative_path_from "$landing_dir" "$KM_FACT_CATALOG")"
    printf -- '- Browse by id/title: [`%s`](%s)\n' "$KM_FACT_CATALOG" "$fact_link"
  fi
  printf -- "- Search all question shards: \`rg -i --glob '%s*.md' 'terms' %s\`\n\n" \
    "$KM_SHARD_PREFIX" "$KM_SHARD_DIR"
  printf '## Question shards\n\n'
  while IFS="$tab" read -r number keys lines bytes stage final; do
    shard_link="$(relative_path_from "$landing_dir" "$KM_SHARD_DIR/$final")"
    printf -- '- [Shard %04d](%s) — %s keys\n' "$number" "$shard_link" "$keys"
  done < "$manifest"
} > "$root_tmp"

IFS="$tab" read -r landing_lines landing_bytes landing_width <<EOF
$(metrics "$root_tmp")
EOF
[ "$landing_lines" -le "$KM_MAX_LANDING_LINES" ] || fail "landing exceeds line bound"
[ "$landing_bytes" -le "$KM_MAX_LANDING_BYTES" ] || fail "landing exceeds byte bound"
[ "$landing_width" -le "$KM_MAX_LANDING_LINE_BYTES" ] || fail "landing exceeds line-width bound"
[ "$((shard_lines + landing_lines))" -le "$KM_MAX_PROJECTION_LINES" ] \
  || fail "projection set exceeds aggregate line bound"
[ "$((shard_bytes + landing_bytes))" -le "$KM_MAX_PROJECTION_BYTES" ] \
  || fail "projection set exceeds aggregate byte bound"

while IFS="$tab" read -r number keys lines bytes stage final; do
  mv "$output_shard_dir/$stage" "$output_shard_dir/$final"
done < "$manifest"
mv "$root_tmp" "$output_root"

for old in "$output_shard_dir"/"$KM_SHARD_PREFIX"[0-9][0-9][0-9][0-9].md; do
  [ -e "$old" ] || continue
  name="$(basename "$old")"
  if ! LC_ALL=C awk -F "$tab" -v name="$name" '$6==name{found=1} END{exit !found}' "$manifest"; then
    rm -f "$old"
  fi
done

printf 'knowledge-map: wrote %s + %s bounded shards (%s facts, %s unique question keys)\n' \
  "$KM_OUTPUT" "$shards" "$facts" "$question_keys" >&2
