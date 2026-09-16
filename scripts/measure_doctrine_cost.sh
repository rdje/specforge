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

# Milliseconds since the epoch. BSD `date` has no %N, so the clock comes from perl, which every gate
# script in this repository already depends on.
now_ms() { perl -MTime::HiRes=time -e 'printf "%.0f", time * 1000'; }

fmt_ms() { # 372451 -> 6m12.5s
  local ms="$1" total_s m s
  total_s=$((ms / 1000))
  m=$((total_s / 60))
  s=$((total_s % 60))
  printf '%dm%02d.%01ds' "$m" "$s" "$(((ms % 1000) / 100))"
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

printf '| Doctrine | Wall clock | Share | Verdict | Enforcer |\n'
printf '| --- | ---: | ---: | --- | --- |\n'
printf '%s' "$rows" | sort -t'|' -k1 -rn | while IFS='|' read -r ms id verdict script; do
  [ -n "$id" ] || continue
  share=$((ms * 1000 / (total_ms > 0 ? total_ms : 1)))
  printf '| `%s` | %s | %d.%01d%% | %s | `%s` |\n' \
    "$id" "$(fmt_ms "$ms")" "$((share / 10))" "$((share % 10))" "$verdict" "$script"
done
printf '| **total (%s tier)** | **%s** | **100.0%%** | %d of %d registered | — |\n' \
  "$RUN_TIER" "$(fmt_ms "$total_ms")" "$measured" "$registered"
