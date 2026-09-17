#!/usr/bin/env bash
# COMMIT-GATE-SINGLE-RUN.0 — per-doctrine wall-clock cost, derived from the registry rather than listed.
#
# `COMMIT.md` step 8 runs `scripts/check_doctrines.sh` and `.githooks/pre-commit` runs it again, so every
# slice pays the complete gate twice. Choosing what a cheaper first pass should contain needs per-doctrine
# timings, and this produces them. It DERIVES the population by parsing the `DOCTRINES=(...)` registry in
# the driver, so a doctrine added there cannot be missing from the table.
#
# Run it alone. A heavy suite run concurrently with the locality gate distorts both, and this repository has
# already recorded one 13m34s outlier against a 5m57s mean that was nothing but contention.
#
#   bash scripts/measure_doctrine_cost.sh            # gate-tier doctrines, the pre-commit population
#   bash scripts/measure_doctrine_cost.sh --all      # every registered doctrine, including the CI tier
#
# Output is a Markdown table on stdout; per-doctrine progress goes to stderr so a redirect keeps the table
# clean. A doctrine that FAILS is still timed and is reported as FAIL — the cost of a failing check is the
# cost the gate actually pays, and hiding it would bias the table toward the happy path.
set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
DRIVER="$ROOT/scripts/check_doctrines.sh"
RUN_TIER='gate'
[ "${1:-}" = '--all' ] && RUN_TIER='all'
if [ -n "${1:-}" ] && [ "${1:-}" != '--all' ]; then
  printf 'Usage: %s [--all]\n' "$0" >&2
  exit 2
fi

[ -r "$DRIVER" ] || { printf 'measure-doctrine-cost: cannot read %s\n' "$DRIVER" >&2; exit 1; }

# COMMIT-GATE-SINGLE-RUN.0a — assert the precondition instead of asserting it in prose. The leaf that
# commissioned this table said "run it alone on an idle machine", and three runs were published as idle
# without anyone measuring: the machine was at load 12.95, and the totals rose 335s -> 429s -> 524s in
# step with it while a single-threaded doctrine stayed flat at 1.02x. A timing tool that cannot see
# contention produces numbers whose error term is invisible, which is worse than no numbers.
load_now() { uptime | sed -e 's/.*load averages*:[[:space:]]*//' -e 's/,//g' | awk '{print $1}'; }
LOAD_START="$(load_now)"
IDLE_MAX="${IDLE_MAX:-2.0}"
if awk -v l="$LOAD_START" -v m="$IDLE_MAX" 'BEGIN { exit !(l > m) }'; then
  printf 'measure-doctrine-cost: load average is %s (idle threshold %s) — this machine is NOT idle.\n' \
    "$LOAD_START" "$IDLE_MAX" >&2
  printf 'measure-doctrine-cost: a contended run times contention, not the gate. Set IDLE_MAX to override\n' >&2
  printf 'measure-doctrine-cost: deliberately, and say so wherever the numbers are published.\n' >&2
  [ -n "${ALLOW_CONTENDED:-}" ] || exit 3
  printf 'measure-doctrine-cost: ALLOW_CONTENDED set — continuing, numbers are CONTENDED.\n' >&2
fi

# Milliseconds since the epoch. BSD `date` has no %N, so the clock comes from perl, which every gate
# script in this repository already depends on.
now_ms() { perl -MTime::HiRes=time -e 'printf "%.0f", time * 1000'; }

# ROUNDS to a tenth; it used to truncate. That mattered: a reader who sums a truncated column loses up to
# 0.05s per row and reports an aggregate that is biased LOW — COMMIT-GATE-SINGLE-RUN.0 published four such
# figures before the bias was caught. The exact integer is in the `ms` column, and the aggregate footer is
# computed from integers, so no aggregate ever has to be summed from the formatted column.
fmt_ms() { # 372451 -> 6m12.5s
  local ms="$1" tenths m s f
  tenths=$(((ms + 50) / 100))
  s=$((tenths / 10))
  f=$((tenths % 10))
  m=$((s / 60))
  s=$((s % 60))
  printf '%dm%02d.%01ds' "$m" "$s" "$f"
}

pct() { # exact tenths of a percent, rounded: pct NUMERATOR DENOMINATOR
  local n="$1" d="$2" t
  [ "$d" -gt 0 ] || { printf '0.0'; return; }
  t=$(((n * 1000 + d / 2) / d))
  printf '%d.%01d' "$((t / 10))" "$((t % 10))"
}

entries="$(awk '/^DOCTRINES=\(/{inside=1; next} inside && /^\)/{inside=0} inside' "$DRIVER" \
  | sed -e 's/^[[:space:]]*"//' -e 's/"[[:space:]]*$//')"
[ -n "$entries" ] || { printf 'measure-doctrine-cost: registry parsed as empty\n' >&2; exit 1; }

registered=0
measured=0
total_ms=0
rows=''
while IFS='|' read -r id tier proves script; do
  [ -n "$id" ] || continue
  registered=$((registered + 1))
  if [ "$tier" = 'ci' ] && [ "$RUN_TIER" != 'all' ]; then
    printf '  skip  %-24s (CI tier)\n' "$id" >&2
    continue
  fi
  if [ ! -x "$ROOT/$script" ]; then
    printf '  FAIL  %-24s registered enforcer missing or not executable: %s\n' "$id" "$script" >&2
    rows="${rows}| \`${id}\` | — | MISSING | \`${script}\` |"$'\n'
    continue
  fi
  start="$(now_ms)"
  if "$ROOT/$script" >/dev/null 2>&1; then verdict='PASS'; else verdict='FAIL'; fi
  elapsed=$(( $(now_ms) - start ))
  measured=$((measured + 1))
  total_ms=$((total_ms + elapsed))
  printf '  %-4s  %-24s %s\n' "$verdict" "$id" "$(fmt_ms "$elapsed")" >&2
  rows="${rows}${elapsed}|${id}|${verdict}|${script}"$'\n'
done <<EOF
$entries
EOF

sorted_rows="$(printf '%s' "$rows" | sort -t'|' -k1 -rn)"
printf '| Doctrine | Wall clock | ms | Share | Verdict | Enforcer |\n'
printf '| --- | ---: | ---: | ---: | --- | --- |\n'
printed=0
while IFS='|' read -r ms id verdict script; do
  [ -n "$id" ] || continue
  printed=$((printed + 1))
  printf '| `%s` | %s | %d | %s%% | %s | `%s` |\n' \
    "$id" "$(fmt_ms "$ms")" "$ms" "$(pct "$ms" "$total_ms")" "$verdict" "$script"
done <<EOF1
$sorted_rows
EOF1
printf '| **total (%s tier)** | **%s** | **%d** | **100.0%%** | %d of %d registered | — |\n' \
  "$RUN_TIER" "$(fmt_ms "$total_ms")" "$total_ms" "$measured" "$registered"

# Aggregates computed from the integer millisecond column, never from the formatted one. HEAVY_TOP is how
# many of the costliest doctrines to split at; the split is the question this table exists to answer.
HEAVY_TOP="${HEAVY_TOP:-4}"
heavy_ms=0
heavy_names=''
i=0
while IFS='|' read -r ms id verdict script; do
  [ -n "$id" ] || continue
  i=$((i + 1))
  [ "$i" -le "$HEAVY_TOP" ] || continue
  heavy_ms=$((heavy_ms + ms))
  heavy_names="${heavy_names}${heavy_names:+, }${id}"
done <<EOF2
$sorted_rows
EOF2
rest_ms=$((total_ms - heavy_ms))
# A dropped row is the census-reads-zero class this repository keeps meeting; assert the population
# rather than trusting the loop, so the table can never quietly describe fewer doctrines than it timed.
if [ "$printed" -ne "$measured" ]; then
  printf 'measure-doctrine-cost: printed %d rows for %d measured doctrines\n' "$printed" "$measured" >&2
  exit 1
fi

printf '\nExact split, from the ms column (not from the rounded one):\n'
printf -- '- costliest %d (%s): %d ms = %s%% of %d ms\n' \
  "$HEAVY_TOP" "$heavy_names" "$heavy_ms" "$(pct "$heavy_ms" "$total_ms")" "$total_ms"
printf -- '- everything else: %d ms = %s%% of the gate\n' "$rest_ms" "$(pct "$rest_ms" "$total_ms")"
printf -- '- load average %s at start, %s at end (idle threshold %s)\n' \
  "$LOAD_START" "$(load_now)" "$IDLE_MAX"
