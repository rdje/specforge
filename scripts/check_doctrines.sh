#!/usr/bin/env bash
# scripts/check_doctrines.sh — THE GENERAL DOCTRINE ENFORCER (driver + registry).
#
# DOCTRINE-ENFORCEMENT-ADOPT.0. Owner directive (2026-06-22): "adopt this doctrine enforcement
# system" → DOCTRINE_ENFORCEMENT.md, the portable standard that makes doctrine compliance provable
# and re-checkable instead of "trust me". This is the single driver that runs EVERY mechanizable
# doctrine check, reports per-doctrine PASS/FAIL, and exits NONZERO on any breach. It unifies the
# previously separate pre-commit / run_ci check stack into one
# registered, self-reporting framework, and it is the seam new doctrine checks are added to.
#
# Enforcement layering (MEMORY_ARCHITECTURE.md §9 / DOCTRINE_ENFORCEMENT.md §7 — defense in depth):
#   E1 discovery   : the doctrine docs (README.md, DOCTRINE_ENFORCEMENT.md, TOOLBOX.md,
#                    docs/decisions/, the mdBook) + the harness bootstrap pointers (AGENTS/CLAUDE).
#   E2 self-check  : THIS script + each registered scripts/check_*.sh (single source of truth).
#   E3 git hook    : .githooks/pre-commit calls this (fast local gate; activated via
#                    `git config core.hooksPath .githooks`).
#   E4 CI          : scripts/run_ci.sh runs the same script server-side (un-bypassable backstop).
#                    HONEST GAP: SpecForge's hosted CI is currently manual-only (workflow_dispatch)
#                    to conserve Actions minutes — so the un-bypassable leg is only as strong as the
#                    next manual/CI run. Re-enabling an auto CI doctrine-gate is the true "no matter
#                    what" backstop (a local hook can be `--no-verify`'d).
#
# What "provable / not trust-me" means here, honestly (DOCTRINE_ENFORCEMENT.md §3):
#   - STRUCTURAL doctrines (memory-arch, knowledge-map) — the check re-derives the invariant from
#     the tree; pass/fail is a fact about the files.
#   - DETERMINISTIC-ORACLE doctrines (kg-bench 156/156, WIRE-BASED-100 golds, cargo fmt/clippy/test)
#     re-RUN the real tool, so a claimed number that does not reproduce FAILS. They are HEAVY, so
#     they run via scripts/run_ci.sh / CI, NOT this pre-commit-fast driver (the strongest leg).
#   - EVIDENCE doctrines (task-acceptance) require a re-checkable artifact (pasted tool output) in
#     the owning task leaf. A hook cannot prove the agent REASONED from it, but it makes landing a
#     Rust code change with no owning task leaf + no pasted tool evidence impossible at the gate.
#
# Registry below = the source of truth for "which doctrines are enforced by what". The human-readable
# mirror is DOCTRINE_ENFORCEMENT.md §10 (kept in lockstep). To add a doctrine: write a
# scripts/check_<id>.sh obeying the §4 contract, add one line here, and add a §10 row.
set -uo pipefail   # deliberately NOT `-e`: run ALL checks, collect every result, then report.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"

# Each entry: "ID|what it proves|relative/path/to/check.sh"
# A meta-check below asserts every registered enforcer exists + is executable, so a registry entry
# can never be a dangling promise.
DOCTRINES=(
  "MEMORY-ARCH|durable 4-layer memory architecture invariants (MEMORY_ARCHITECTURE.md §9)|scripts/check_memory_architecture.sh"
  "KNOWLEDGE-MAP|the derived Knowledge Map is in sync with its fact sources|knowledge-map/scripts/check_knowledge_map.sh"
  "TASK-ACCEPTANCE|a staged Rust code change is owned by a staged task-tree leaf passing the evidence-backed acceptance checklist (decision 0003 / TOOLBOX.md)|scripts/check_task_acceptance.sh"
  "README-POLICY|the landing README and its reader/author routes satisfy the repository-owned bounded-entrypoint contract|scripts/check_readme_policy.sh"
)

fail=0
declare -a report=()

for entry in "${DOCTRINES[@]}"; do
  IFS='|' read -r id proves script <<< "$entry"
  if [ ! -x "$ROOT/$script" ]; then
    report+=("FAIL  ${id} — registered enforcer missing or not executable: ${script}")
    fail=1
    continue
  fi
  if out="$("$ROOT/$script" 2>&1)"; then
    report+=("PASS  ${id} — ${proves}")
  else
    report+=("FAIL  ${id} — ${proves}")
    printf '%s\n' "$out" >&2
    fail=1
  fi
done

printf '\n================ DOCTRINE ENFORCEMENT REPORT ================\n' >&2
for line in "${report[@]}"; do printf '  %s\n' "$line" >&2; done
printf '============================================================\n' >&2
if [ "$fail" -eq 0 ]; then
  printf 'doctrines: ALL %d enforced doctrines PASS.\n' "${#DOCTRINES[@]}" >&2
else
  printf 'doctrines: one or more doctrines FAILED — commit/merge blocked. Fix above, do not bypass.\n' >&2
fi
exit "$fail"
