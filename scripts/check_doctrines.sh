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
# mirror is docs/DOCTRINE_INSTANCE.md §10 (kept in lockstep). To add a doctrine: write a
# scripts/check_<id>.sh obeying the §4 contract, add one line here, and add a §10 row there.
#
# TIERS (DOCTRINE_ENFORCEMENT.md §4.7 — "fast, or deferred"): a check too heavy for a pre-commit hook
# stays IN this registry and is marked `ci`, so it is enforced but not paid for on every commit. The
# default run executes the `gate` tier and REPORTS every deferred doctrine as DEFER — a CI-tier
# doctrine is never silently absent from the report. `--all` runs every tier and is what
# scripts/run_ci.sh invokes.
set -uo pipefail   # deliberately NOT `-e`: run ALL checks, collect every result, then report.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

# COMMIT-GATE-SINGLE-RUN.1 — `--only` exists so a caller never has to invoke a gate script by hand.
# Hand-invocation is where a check silently becomes a no-op: there is no flag convention across the 42
# `scripts/check_*.pl|sh` gates (executed, not grepped: 27 accept `--check`, 15 reject it), the Perl ones
# answer a wrong flag with usage on stderr and a non-zero status that `cmd | grep | head` then hides
# behind `head`'s own 0, and eight shell ones used to ignore an unknown argument entirely and exit 0
# (all eight are now guarded; the population re-swept at 42 of 42 REFUSE). Selecting through the driver
# removes the choice: it runs each enforcer exactly as the pre-commit hook does, with no arguments, and
# asserts the exit status itself. An unknown id is refused rather than matching nothing.
#
# COMMIT-GATE-SINGLE-RUN.2 — `--fast` is the manual leg's early signal, and it is a SUBSET by exclusion.
# `COMMIT.md` step 8 used to mandate a full manual run that `.githooks/pre-commit` then repeated, so every
# slice paid the complete gate twice. The hook is the leg that cannot be skipped, so the redundancy came
# off the manual side: step 8 now runs `--fast` on EVERY slice and the full driver by hand on none, and
# the hook runs the complete driver once at commit.
#
# COMMIT-GATE-SINGLE-RUN.3 — "every slice", not "every slice that touches no Rust", and the arithmetic is
# why. With G the cost of one gate, a manual full run costs G+G when it passes against G for letting the
# hook be the only full run, and both cost G+fix+G when it fails: never cheaper, whatever the slice
# touched. A Rust slice instead runs what this driver never runs at all — it neither compiles nor tests —
# plus `--only PRODUCTION-GENERICITY` when the producer graph could move.
RUN_TIER=gate
ONLY=''
FAST=''
while [ "$#" -gt 0 ]; do
  case "$1" in
    --all)  RUN_TIER=all ;;
    --gate) RUN_TIER=gate ;;
    --list) RUN_TIER=list ;;
    --fast) FAST=1 ;;
    --only)
      shift
      [ "$#" -gt 0 ] || { printf '%s: --only requires ID[,ID...]\n' "$0" >&2; exit 2; }
      ONLY="${ONLY}${ONLY:+,}$1"
      ;;
    --only=*) ONLY="${ONLY}${ONLY:+,}${1#--only=}" ;;
    *) printf 'Usage: %s [--gate|--all|--list] [--fast] [--only ID[,ID...]]\n' "$0" >&2; exit 2 ;;
  esac
  shift
done
if [ -n "$FAST" ] && [ -n "$ONLY" ]; then
  printf '%s: --fast and --only are two different selections; pass one.\n' "$0" >&2
  exit 2
fi

# Each entry: "ID|tier|what it proves|relative/path/to/check.sh"
#   tier `gate` = cheap enough for the pre-commit hook (and CI); tier `ci` = deferred to CI only.
# A meta-check below asserts every registered enforcer exists + is executable, so a registry entry
# can never be a dangling promise — deferred entries are meta-checked too.
DOCTRINES=(
  "MEMORY-ARCH|gate|durable 4-layer memory architecture invariants (MEMORY_ARCHITECTURE.md §9)|scripts/check_memory_architecture.sh"
  "KNOWLEDGE-MAP|gate|the bounded Knowledge Map landing and question shards are in sync|knowledge-map/scripts/check_knowledge_map.sh"
  "TASK-ACCEPTANCE|gate|a staged Rust code change is owned by a staged task-tree leaf passing the evidence-backed acceptance checklist (decision 0003 / TOOLBOX.md)|scripts/check_task_acceptance.sh"
  "README-POLICY|gate|the landing README and its reader/author routes satisfy the repository-owned bounded-entrypoint contract|scripts/check_readme_policy.sh"
  "LIVE-DOC-SIZE|gate|every tracked Markdown surface and declared current-state field satisfies its registered lifecycle, authority, coverage, locality, size, currency, and route contracts|scripts/check_live_document_size.sh"
  "SECTION-ANCHORS|gate|every qualified cross-document section reference resolves to a real heading in the file it names, so a content-preserving move cannot silently break a route|scripts/check_section_anchors.pl"
  "PROJECT-DATA-LOCALITY|gate|project-owned temp, cache, dependency, artifact, and subprocess seams resolve from the repository root|scripts/check_project_data_locality.sh"
  "PRODUCTION-GENERICITY|gate|the product package boundary, complete inventories, compiled information flow, and proof-only canonical authority hold|scripts/check_production_genericity.sh"
  "CORPUS-FRONTIER|gate|the corpus refresh frontier is derived from persisted evidence and agrees exactly with its declaration, retention, and the root task file|scripts/check_corpus_frontier.sh"
  "CLAIM-VERIFICATION|gate|bounded claim records, tracked current evidence, executed source/RED controls, complete stale coverage, and publication IDs resolve together|scripts/check_claim_verification.pl"
  "PUBLISHED-ASSERTIONS|gate|every published value in a governed region re-derives against its named producer field, or is gated by a control with a known-bad case, authored by a decision, or anchored to a revision — and no two surfaces disagree on one field (CLAIM-VERIFICATION-ADOPTION.7)|scripts/check_published_assertions.pl"
  "RESIDUAL-ACTIONABILITY|gate|the frozen required-residual contract still pins the tracked current result, re-derives its own cell decomposition and published ratio from it, and resolves every typed cause to a real production carrier (SPEC-TO-INTENT-ALIGNMENT.8a/.9d)|scripts/validate_residual_actionability_contract.py"
  "TASK-NODE-RETENTION|gate|a task node may be renamed into descendants or retired through a declared removal, but it may not vanish from every tracked task surface — layer B of MEMORY_ARCHITECTURE.md is the only record of why a closed leaf closed the way it did (TASK-NODE-RETENTION.0)|scripts/check_task_node_retention.py"
  "OWNERSHIP-CITATIONS|gate|every work unit cited inside a declared ownership region is classified, and every citation classified as a current owner names a unit whose own Status line is still open — it proves the owner is open, never that the open owner is the right one (LIVE-DOCUMENT-PRESSURE-HEADROOM.15)|scripts/check_ownership_citations.pl"
  "CONSTRAINT-PART-SPAN|gate|the kind classifier's call-site topology still matches the stratification the constraint-part-span census is derived from, and only the typed gateway reaches its untyped arm (EXTRACTION-QUALITY-GAUGE.3k/.3k.2e/.3k.2j)|scripts/check_constraint_part_span.sh"
  "PROOF-SEAL-CURRENCY|gate|every persisted corpus artifact records a proof seal the current build's own canonical loader still accepts, censused across the whole proof-carrying stratum and probed read-only|scripts/check_proof_seal_currency.sh"
  "PROOF-SEAL-TOTAL|ci|every in-scope persisted artifact is probed INDIVIDUALLY against the current build's canonical loader, because one probe per distinct seal cannot see a per-document replay-topology divergence (SIGNAL-DECLARATION-ROW-DROP.1c)|scripts/check_proof_seal_total.sh"
  "CHAIN-CURRENCY|ci|every persisted corpus artifact is exactly what the current binary reproduces from its persisted input (ADR 0025)|scripts/check_chain_currency.sh"
)

# COMMIT-GATE-SINGLE-RUN.2 — the doctrines `--fast` LEAVES OUT. This is an EXCLUSION list on purpose:
# a doctrine added to the registry above is in the fast set automatically, and taking it out costs a
# deliberate edit here. An inclusion list would have the opposite failure — a newly registered doctrine
# would be silently outside the fast set, which is the exact drift `.2` was opened to prevent.
#
# Membership is the costliest four measured by `scripts/measure_doctrine_cost.sh`
# (COMMIT-GATE-SINGLE-RUN.0). That leaf's SHARES are withdrawn — they were taken at load average 12.95
# and three runs of the same tree totalled 335s -> 429s -> 524s — but the MEMBERSHIP of the costliest
# four was identical in all three, which is the only part a subset needs. `.0a` owns the re-measurement
# on a machine proved idle; if it moves the membership, it moves this list and nothing else.
#
# What this subset does NOT cover, stated here so it cannot be forgotten at the call site: of the five
# doctrines evidenced blocking a commit in the 2026-09-17 session, `--fast` runs CLAIM-VERIFICATION,
# PUBLISHED-ASSERTIONS and KNOWLEDGE-MAP but NOT LIVE-DOC-SIZE or PROJECT-DATA-LOCALITY. A green
# `--fast` is an early signal, never evidence that the commit will pass.
FAST_EXCLUDE=(
  "LIVE-DOC-SIZE"
  "PROJECT-DATA-LOCALITY"
  "PROOF-SEAL-CURRENCY"
  "PRODUCTION-GENERICITY"
)

# A dangling exclusion is the same class of defect as a dangling enforcer path, so it is meta-checked
# the same way and on every run: renaming a doctrine in the registry without renaming it here would
# otherwise leave `--fast` quietly running MORE than its declared subset (or, for an inclusion list,
# less). Checked unconditionally, because the run that discovers it should be the next one, not the
# next `--fast`.
fast_excluded() { # id -> 0 when `--fast` must leave it out
  local want="$1" excl
  for excl in "${FAST_EXCLUDE[@]}"; do
    [ "$excl" = "$want" ] && return 0
  done
  return 1
}

fast_unknown=''
for excl in "${FAST_EXCLUDE[@]}"; do
  found=''
  for entry in "${DOCTRINES[@]}"; do
    IFS='|' read -r id tier proves script <<< "$entry"
    [ "$id" = "$excl" ] && found=1 && break
  done
  [ -n "$found" ] || fast_unknown="${fast_unknown}${fast_unknown:+ }${excl}"
done
if [ -n "$fast_unknown" ]; then
  printf '%s: FAST_EXCLUDE names no registered doctrine: %s\n' "$0" "$fast_unknown" >&2
  printf '%s: the fast subset is declared against the registry; fix one or the other.\n' "$0" >&2
  exit 2
fi

# `--fast` is only safe because the pre-commit hook runs the COMPLETE driver afterwards. That leg is a
# per-clone opt-in (`git config core.hooksPath .githooks`), so assert it rather than assume it: in a
# clone where the hook is not wired, a subset run would be the only doctrine enforcement that ever
# happened, and it would report PASS.
if [ -n "$FAST" ]; then
  hooks_path="$(git -C "$ROOT" config --get core.hooksPath 2>/dev/null || true)"
  case "$hooks_path" in
    '')  hook_pre='' ;;
    /*)  hook_pre="$hooks_path/pre-commit" ;;
    *)   hook_pre="$ROOT/$hooks_path/pre-commit" ;;
  esac
  if [ -z "$hook_pre" ] || [ ! -x "$hook_pre" ] || ! grep -q 'check_doctrines.sh' "$hook_pre"; then
    printf '%s: --fast refused — the pre-commit hook that runs the COMPLETE gate is not active.\n' "$0" >&2
    printf '%s: core.hooksPath=%s; expected an executable pre-commit that runs check_doctrines.sh.\n' \
      "$0" "${hooks_path:-<unset>}" >&2
    printf '%s: activate it with `git config core.hooksPath .githooks`, or run the full gate by hand.\n' "$0" >&2
    exit 2
  fi
fi

# Validate the selection against the registry, not against a hand-kept list: an id that names no
# registered doctrine is REFUSED, so `--only` can never quietly select nothing and report success.
selected=''
if [ "$RUN_TIER" = 'list' ]; then
  for entry in "${DOCTRINES[@]}"; do
    IFS='|' read -r id tier proves script <<< "$entry"
    printf '%-24s %-5s %s\n' "$id" "$tier" "$script"
  done
  exit 0
fi
if [ -n "$ONLY" ]; then
  unknown=''
  for want in $(printf '%s' "$ONLY" | tr ',' ' '); do
    [ -n "$want" ] || continue
    found=''
    for entry in "${DOCTRINES[@]}"; do
      IFS='|' read -r id tier proves script <<< "$entry"
      [ "$id" = "$want" ] && found=1 && break
    done
    if [ -z "$found" ]; then
      unknown="${unknown}${unknown:+ }${want}"
    else
      selected="${selected}${selected:+ }${want}"
    fi
  done
  if [ -n "$unknown" ]; then
    printf '%s: --only names no registered doctrine: %s\n' "$0" "$unknown" >&2
    printf '%s: run `%s --list` for the registered ids.\n' "$0" "$0" >&2
    exit 2
  fi
fi

fail=0
declare -a report=()
declare -a warned=()
declare -a slow=()

# ── Stall legibility ──────────────────────────────────────────────────────────────────────────
# GATE-FIXTURE-EXEC-STALL.3. Each doctrine's output is captured below, and the report is printed only
# when every doctrine has finished — so a step that blocks produces NOTHING, and a blocked gate is
# indistinguishable from a hung one. On `2026-09-19` that ambiguity cost a session forty minutes, an
# aborted commit and a wrong first diagnosis: the actual cause was the host assessing newly created
# executables on first exec, which the gate triggers because its fixtures ARE new executables, and the
# whole process tree sat at ~0% CPU while a security daemon worked.
#
# Two cheap things make that legible, and neither changes a verdict: say which doctrine is running
# before running it, and if one is still going after a generous interval, name the condition and the
# command that confirms or excludes it. Progress goes to stderr so the report on stdout is unchanged.
STALL_NOTICE_SECONDS="${SPECFORGE_DOCTRINE_STALL_SECONDS:-120}"

# stall_watch <id> — the caller BACKGROUNDS this; it must not background itself. An earlier version
# did, and returned its pid through a command substitution: `pid="$(stall_watch "$id")"`. That hangs,
# and the way it hangs is worth keeping. A command substitution does not return until its stdout pipe
# closes, and a process backgrounded INSIDE it inherits that pipe and holds it open for the whole
# sleep — so the driver waited the full notice interval before every doctrine, and the instrument
# built to reveal a stall became one. It was caught because its own probe excluded the host: steps at
# ~0% CPU while `probe_exec_assessment_latency.sh` reported no assessment stall.
stall_watch() {
  local id="$1"
  sleep "$STALL_NOTICE_SECONDS" 2>/dev/null || return 0
  printf 'doctrines: %s is still running after %ss with no verdict.\n' "$id" "$STALL_NOTICE_SECONDS" >&2
  printf 'doctrines:   This is usually NOT a hang. If the process tree is at ~0%% CPU it is waiting on the\n' >&2
  printf 'doctrines:   host to assess newly created executables, which this gate creates by the hundred.\n' >&2
  printf 'doctrines:   Confirm or exclude it:  bash scripts/probe_exec_assessment_latency.sh\n' >&2
  printf 'doctrines:   Background: docs/tasks/GATE-FIXTURE-EXEC-STALL.md and TOOLBOX.md.\n' >&2
}

# LIVE-DOCUMENT-PRESSURE-HEADROOM.18 — a doctrine may have work that belongs to the CI tier without the
# whole doctrine being CI-tier. The claim registry's declared staleness gates re-run producers this same
# pass already runs, so they are deferred on the commit path and executed here; the checker reports
# executed / deferred / self-referential either way, so the output never implies more than it did.
[ "$RUN_TIER" = 'all' ] && export CLAIM_VERIFICATION_EXECUTE_STALE_GATES=1

for entry in "${DOCTRINES[@]}"; do
  IFS='|' read -r id tier proves script <<< "$entry"
  if [ ! -x "$ROOT/$script" ]; then
    report+=("FAIL  ${id} — registered enforcer missing or not executable: ${script}")
    fail=1
    continue
  fi
  # An explicit `--only` selection overrides the tier default: naming a doctrine is a decision to pay
  # for it. Everything not selected is reported as SKIP, never omitted, so a subset run cannot read as
  # a complete one.
  if [ -n "$ONLY" ]; then
    case " $selected " in
      *" $id "*) ;;
      *) report+=("SKIP  ${id} — not selected by --only"); continue ;;
    esac
  else
    if [ "$tier" = 'ci' ] && [ "$RUN_TIER" != 'all' ]; then
      report+=("DEFER ${id} — CI-tier (§4.7); run \`scripts/check_doctrines.sh --all\` or scripts/run_ci.sh")
      continue
    fi
    # `--fast` drops the measured costliest four and nothing else; the hook runs them at commit.
    if [ -n "$FAST" ] && fast_excluded "$id"; then
      report+=("SKIP  ${id} — excluded from --fast (measured costliest four; the hook runs it at commit)")
      continue
    fi
  fi
  printf 'doctrines: running %s …\n' "$id" >&2
  stall_watch "$id" &
  stall_pid=$!
  step_start="$(perl -MTime::HiRes=time -e 'printf "%.3f", time' 2>/dev/null || printf '0')"
  out="$("$ROOT/$script" 2>&1)" && step_ok=1 || step_ok=0
  step_end="$(perl -MTime::HiRes=time -e 'printf "%.3f", time' 2>/dev/null || printf '0')"
  kill "$stall_pid" 2>/dev/null
  wait "$stall_pid" 2>/dev/null
  step_secs="$(perl -e 'printf "%.0f", $ARGV[1] - $ARGV[0]' "$step_start" "$step_end" 2>/dev/null || printf '0')"
  [ "$step_secs" -ge "$STALL_NOTICE_SECONDS" ] 2>/dev/null && slow+=("${id} ${step_secs}s")
  if [ "$step_ok" -eq 1 ]; then
    report+=("PASS  ${id} — ${proves}")
    # COMMIT-GATE-SINGLE-RUN.4 — a PASSING check's output used to be discarded with $out, and the
    # early-warning half of containment went with it. The enforcers DO warn: a direct run of
    # check_live_document_size.sh emits 44 warning lines, one of them `surface 'task_evidence'
    # lines_each is at or above rollover (99.9%) — 2 below its 3000 ceiling`. Through this driver —
    # which is how the hook, run_ci.sh and COMMIT.md step 8 all run it — exactly 0 of them were
    # visible. A bound that only ever speaks by refusing turns scheduled work into a blocked commit,
    # and the author into someone shrinking evidence to land a slice. Surface them instead.
    while IFS= read -r line; do
      [ -n "$line" ] && warned+=("${id}: ${line}")
    done < <(printf '%s\n' "$out" | grep -i 'warning' || true)
  else
    report+=("FAIL  ${id} — ${proves}")
    printf '%s\n' "$out" >&2
    fail=1
  fi
done

printf '\n================ DOCTRINE ENFORCEMENT REPORT ================\n' >&2
for line in "${report[@]}"; do printf '  %s\n' "$line" >&2; done
printf '============================================================\n' >&2
# Pressure is reported on the way UP, not at the stop. These come from checks that PASSED.
if [ "${#warned[@]}" -gt 0 ]; then
  printf '\n---- PRESSURE (%d) — from PASSING checks; this is how a stop gets scheduled ----\n' \
    "${#warned[@]}" >&2
  for line in "${warned[@]}"; do printf '  %s\n' "$line" >&2; done
  printf -- '---- a remedy that requires DELETING evidence is a policy defect, not an author problem ----\n' >&2
fi

# A step that took longer than the stall interval is named with its measured cost, so "the gate is
# slow" becomes "this doctrine took N seconds" without anyone re-running it to find out which.
if [ "${#slow[@]}" -gt 0 ]; then
  printf '\n---- SLOW (%d) — steps past the %ss notice interval ----\n' "${#slow[@]}" "$STALL_NOTICE_SECONDS" >&2
  for line in "${slow[@]}"; do printf '  %s\n' "$line" >&2; done
  printf -- '---- wall clock far above CPU time is the host assessing new executables, not this repository:\n' >&2
  printf -- '     bash scripts/probe_exec_assessment_latency.sh (GATE-FIXTURE-EXEC-STALL) ----\n' >&2
fi
if [ "$fail" -eq 0 ]; then
  executed=0
  for line in "${report[@]}"; do
    case "$line" in DEFER*|SKIP*) ;; *) executed=$((executed + 1)) ;; esac
  done
  if [ -n "$ONLY" ]; then
    # Never let a subset run print the sentence a full run prints.
    printf 'doctrines: SUBSET ONLY — %d of %d registered doctrines executed and PASS; this is NOT the gate.\n' \
      "$executed" "${#DOCTRINES[@]}" >&2
  elif [ -n "$FAST" ]; then
    # The same rule, plus the membership: a reader who copies this line into a task leaf carries the
    # caveat with it, so "fast PASS" can never be transcribed as "gate PASS".
    skipped=''
    for excl in "${FAST_EXCLUDE[@]}"; do skipped="${skipped}${skipped:+, }${excl}"; done
    printf 'doctrines: FAST SUBSET — %d of %d registered doctrines executed and PASS; this is NOT the gate.\n' \
      "$executed" "${#DOCTRINES[@]}" >&2
    printf 'doctrines: NOT run by --fast: %s. The pre-commit hook runs the complete driver at commit.\n' \
      "$skipped" >&2
  else
    printf 'doctrines: ALL %d executed doctrines PASS (%d registered, tier=%s).\n' \
      "$executed" "${#DOCTRINES[@]}" "$RUN_TIER" >&2
  fi
else
  printf 'doctrines: one or more doctrines FAILED — commit/merge blocked. Fix above, do not bypass.\n' >&2
fi
exit "$fail"
