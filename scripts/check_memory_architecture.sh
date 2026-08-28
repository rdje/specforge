#!/usr/bin/env bash
#
# check_memory_architecture.sh — the single source of truth for the memory-architecture
# invariants (layer E2 of MEMORY_ARCHITECTURE.md). Exits NONZERO on any breach. Called
# by the git pre-commit hook (.githooks/pre-commit) and by CI (scripts/run_ci.sh).
#
# Adapt only the knobs below to port to another repo.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT_DIR"
cd "${ROOT_DIR}"

# ── Knobs (everything else is project-neutral) ──────────────────────────────
MEMORY_POINTER_LINE_CAP="${MEMORY_POINTER_LINE_CAP:-50}"    # reviewed survivor: 28 lines
MEMORY_POINTER_BYTE_CAP="${MEMORY_POINTER_BYTE_CAP:-32768}" # fixed one-read ceiling: 32 * 1024 B
MEMORY_POINTER_LINE_BYTE_CAP="${MEMORY_POINTER_LINE_BYTE_CAP:-160}" # survivor max: 93
# The pointer's line cap is shared between prose that never changes and the "Current state" block that
# carries the whole resume signal. Left ungoverned the fixed half wins: it was 19 of 50 lines in every
# one of the last 30 commits while the mutable half grew 17 -> 28, i.e. 90% of what was left. This bound
# is DERIVED from the line cap rather than chosen, so the mutable half always keeps at least the
# remaining share, and the remedy for a breach is routing, never a bigger cap.
MEMORY_POINTER_FIXED_SHARE_DIVISOR="${MEMORY_POINTER_FIXED_SHARE_DIVISOR:-4}"
MEMORY_POINTER_MUTABLE_MARKER="${MEMORY_POINTER_MUTABLE_MARKER:-## Current state}"
TASKS_DIR="docs/tasks"                                      # task-trees (layer B)
DECISIONS_DIR="docs/decisions"                              # decision records (layer C)
BOOTSTRAP_FILES=("AGENTS.md" "CLAUDE.md")                   # tool-neutral entrypoints (E1)

fail=0
note() { printf '[memory-arch] FAIL: %s\n' "$1" >&2; fail=1; }
ok()   { printf '[memory-arch] ok:   %s\n' "$1"; }

# Lines from the top of a resume pointer up to and including its mutable marker. Prints 0 when the
# marker is absent, which is itself a breach: a pointer with no overwritable block has no resume signal.
pointer_fixed_lines() {
  LC_ALL=C awk -v marker="${MEMORY_POINTER_MUTABLE_MARKER}" '
    { n++ }
    index($0, marker) == 1 { print n; found = 1; exit }
    END { if (!found) print 0 }
  ' "$1"
}

# 1) The standard itself must be present (it documents the whole system).
if [[ -f MEMORY_ARCHITECTURE.md ]]; then ok "MEMORY_ARCHITECTURE.md present"
else note "MEMORY_ARCHITECTURE.md is missing (the memory standard)"; fi

# 2) MEMORY.md must exist and stay a bounded resume pointer (layer A).
if [[ -f MEMORY.md ]]; then
  lines="$(wc -l < MEMORY.md | tr -d ' ')"
  bytes="$(wc -c < MEMORY.md | tr -d ' ')"
  max_line_bytes="$(LC_ALL=C awk '{ sub(/\r$/, ""); if (length > max) max = length } END { print max + 0 }' MEMORY.md)"
  if [[ "${lines}" -le "${MEMORY_POINTER_LINE_CAP}" ]]; then
    ok "MEMORY.md is ${lines} lines (<= cap ${MEMORY_POINTER_LINE_CAP})"
  else
    note "MEMORY.md is ${lines} lines (> cap ${MEMORY_POINTER_LINE_CAP}) — it is the resume pointer; move content to ${TASKS_DIR}/ or ${DECISIONS_DIR}/"
  fi
  if [[ "${bytes}" -le "${MEMORY_POINTER_BYTE_CAP}" ]]; then
    ok "MEMORY.md is ${bytes} bytes (<= cap ${MEMORY_POINTER_BYTE_CAP})"
  else
    note "MEMORY.md is ${bytes} bytes (> cap ${MEMORY_POINTER_BYTE_CAP}) — move detail to its canonical layer"
  fi
  if [[ "${max_line_bytes}" -le "${MEMORY_POINTER_LINE_BYTE_CAP}" ]]; then
    ok "MEMORY.md max content line is ${max_line_bytes} bytes (<= cap ${MEMORY_POINTER_LINE_BYTE_CAP})"
  else
    note "MEMORY.md max content line is ${max_line_bytes} bytes (> cap ${MEMORY_POINTER_LINE_BYTE_CAP}) — wrap or route dense content"
  fi

  fixed_cap=$(( MEMORY_POINTER_LINE_CAP / MEMORY_POINTER_FIXED_SHARE_DIVISOR ))
  fixed_lines="$(pointer_fixed_lines MEMORY.md)"
  if [[ "${fixed_lines}" -eq 0 ]]; then
    note "MEMORY.md has no '${MEMORY_POINTER_MUTABLE_MARKER}' block — the pointer carries no overwritable resume signal"
  elif [[ "${fixed_lines}" -le "${fixed_cap}" ]]; then
    ok "MEMORY.md fixed prose is ${fixed_lines} lines (<= derived cap ${fixed_cap} = ${MEMORY_POINTER_LINE_CAP}/${MEMORY_POINTER_FIXED_SHARE_DIVISOR}); $(( MEMORY_POINTER_LINE_CAP - fixed_lines )) lines left for current state"
  else
    note "MEMORY.md fixed prose is ${fixed_lines} lines (> derived cap ${fixed_cap} = ${MEMORY_POINTER_LINE_CAP}/${MEMORY_POINTER_FIXED_SHARE_DIVISOR}) — it is spending the resume budget on prose that never changes; route it to AGENTS.md or MEMORY_ARCHITECTURE.md, do not raise the cap"
  fi

  for required in "Active unit:" "Next action:" "In-flight uncommitted:" "Blockers:"; do
    if grep -Fq -- "- ${required}" MEMORY.md; then ok "MEMORY.md declares ${required}"
    else note "MEMORY.md is missing required resume field '${required}'"; fi
  done
  if grep -Eiq 'latest_commit|latest commit' MEMORY.md; then
    note "MEMORY.md mirrors HEAD as a latest-commit field — query 'git rev-parse HEAD' on resume instead"
  else
    ok "MEMORY.md does not mirror HEAD in a latest-commit shadow field"
  fi
else
  note "MEMORY.md (the resume pointer) is missing"
fi

# 3) Tool-neutral bootstrap pointers must exist and route to the standard (E1).
for f in "${BOOTSTRAP_FILES[@]}"; do
  if [[ -f "${f}" ]]; then
    if grep -q "MEMORY_ARCHITECTURE.md" "${f}"; then ok "${f} points at MEMORY_ARCHITECTURE.md"
    else note "${f} exists but does not point at MEMORY_ARCHITECTURE.md"; fi
  else
    note "${f} bootstrap pointer is missing"
  fi
done

# 4) Layer-C decision store must exist with an index.
if [[ -d "${DECISIONS_DIR}" && -f "${DECISIONS_DIR}/INDEX.md" ]]; then
  ok "${DECISIONS_DIR}/ + INDEX.md present"
else
  note "${DECISIONS_DIR}/INDEX.md is missing (layer C — durable decision records)"
fi

# 5) Layer-B task-trees must exist with an index.
if [[ -d "${TASKS_DIR}" && -f "docs/TASK_TREE.md" ]]; then
  ok "${TASKS_DIR}/ + docs/TASK_TREE.md present"
else
  note "${TASKS_DIR}/ or docs/TASK_TREE.md is missing (layer B — task-trees)"
fi

if [[ "${fail}" -ne 0 ]]; then
  printf '[memory-arch] memory-architecture check FAILED — see MEMORY_ARCHITECTURE.md\n' >&2
  exit 1
fi
printf '[memory-arch] all memory-architecture invariants hold\n'
