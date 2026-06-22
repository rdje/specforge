#!/usr/bin/env bash
# scripts/check_task_acceptance.sh
# DOCTRINE-ENFORCEMENT-ADOPT.1 — mechanize SpecForge's flagship doctrine (decision 0003, owner-restated
# repeatedly): NO code change without an owning task-tree leaf first, and the change must be PROVABLY
# taken through the procedure (diagnose -> address -> no regression), not "trust me".
#
# This gate enforces, for any STAGED Rust behavioral code change, that a STAGED owning task-tree leaf
# (docs/tasks/<TREE>.md) carries the REQUIRED ACCEPTANCE CHECKLIST, each required box TICKED ([x]) and
# backed by real SpecForge tool-output evidence. A missing owning leaf, or an unticked / unbacked
# required box, BLOCKS the commit. Exits NONZERO on any breach. See DOCTRINE_ENFORCEMENT.md (§3
# evidence archetype, §6 reasoned-from-evidence) and TOOLBOX.md (the checklist template + tool catalog).
#
# Required checklist (label keywords are flexible; the [x] + the keyword + the evidence are what matter):
#   - [x] ROOT CAUSE (WHY + WHERE) ... backed by a SpecForge DIAGNOSIS signature (a validate finding/
#         metric, an adapt --target isf blocking reason, a kg-bench fixture diagnostic, a failing
#         `cargo test <name>`, an evidence/--dry-run measurement, a file:line, a measured N->M).
#   - [x] ADDRESSED (verified) ....... the change does what it should, measured per item (before->after,
#         a per-document count, a recovered surface).
#   - [x] NO REGRESSION ............. backed by a SpecForge ORACLE signature: kg-bench 156/156,
#         WIRE-BASED-100 = 1.000, byte-identical golds, run_ci.sh / cargo fmt|clippy|test green.
#
# Why "not trust-me": (1) a ticked box must co-occur with the real tool-output signature SpecForge's
# tools / golds emit; (2) the DETERMINISTIC oracles (kg-bench, the WIRE-BASED-100 golds, the
# byte-identical evidence/.isf checks, cargo) RE-RUN in scripts/run_ci.sh / CI, so a fabricated
# NO-REGRESSION claim does not reproduce and fails there (DOCTRINE_ENFORCEMENT.md §6.1 leg 3). Honest
# limit: a local hook can only check the boxes are ticked + evidence is present; the un-fakeable re-run
# is at run_ci.sh / CI (currently manual-only).
#
# Scope: a CODE change = staged Rust source / in-crate tests / gold fixtures
# (crates/**/*.rs, crates/**/test_data/**). Docs-only, scripts-only, mdBook-only, and .githooks-only
# commits are EXEMPT (they carry their own gates) so this never false-blocks continuity work.
#
# Knobs: SPECFORGE_TASK_ACCEPTANCE_RANGE=<git-range> to check a range instead of the staged set;
#        SPECFORGE_TASK_ACCEPTANCE_WAIVER="<reason>" to record a loud, never-silent exception (CI still
#        re-checks). Bash-3.2-safe (no mapfile) so a fresh clone on stock macOS works.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"

# Staged (or ranged) change set.
if [ -n "${SPECFORGE_TASK_ACCEPTANCE_RANGE:-}" ]; then
  staged="$(git diff --name-only --diff-filter=ACMR "$SPECFORGE_TASK_ACCEPTANCE_RANGE" 2>/dev/null || true)"
else
  staged="$(git diff --cached --name-only --diff-filter=ACMR 2>/dev/null || true)"
fi

# Is this a Rust behavioral CODE change? (`*` matches `/` in case patterns, so these reach any depth.)
code_changed=0
while IFS= read -r f; do
  [ -z "$f" ] && continue
  case "$f" in
    crates/*.rs)          code_changed=1 ;;
  esac
  case "$f" in
    crates/*/test_data/*) code_changed=1 ;;
  esac
done <<EOF
$staged
EOF

if [ "$code_changed" -eq 0 ]; then
  echo "task-acceptance: OK (no Rust code change staged; acceptance checklist not required)"
  exit 0
fi

if [ -n "${SPECFORGE_TASK_ACCEPTANCE_WAIVER:-}" ]; then
  printf 'task-acceptance: WAIVED by SPECFORGE_TASK_ACCEPTANCE_WAIVER="%s" — exception recorded; CI still re-checks.\n' \
    "$SPECFORGE_TASK_ACCEPTANCE_WAIVER" >&2
  exit 0
fi

# A code change must be owned by a staged task-tree leaf carrying the acceptance checklist.
staged_tasks="$(printf '%s\n' "$staged" | grep -E '^docs/tasks/.*\.md$' || true)"
if [ -z "$staged_tasks" ]; then
  cat >&2 <<'MSG'
task-acceptance: a Rust CODE change is staged but NO owning task-tree leaf (docs/tasks/*.md) is staged.
  Doctrine (docs/decisions/0003): no code change without an owning task-tree leaf first. Stage the
  owning docs/tasks/<TREE>.md carrying the ACCEPTANCE CHECKLIST (template in TOOLBOX.md).
MSG
  exit 1
fi

# Concatenate the staged leaves once for scanning (working-tree content == staged at commit time).
leaf_text=""
while IFS= read -r t; do
  [ -z "$t" ] && continue
  [ -f "$t" ] && leaf_text="$leaf_text
$(cat "$t")"
done <<EOF
$staged_tasks
EOF

# A checked / unchecked checklist box mentioning a category keyword.
checked()   { printf '%s\n' "$leaf_text" | grep -Eiq "^[[:space:]]*[-*][[:space:]]*\[[xX]\][[:space:]].*($1)"; }
unchecked() { printf '%s\n' "$leaf_text" | grep -Eiq "^[[:space:]]*[-*][[:space:]]*\[[[:space:]]\][[:space:]].*($1)"; }

# SpecForge tool-output signatures that must BACK the ticked boxes.
DIAGNOSIS_SIG='validate|--dry-run|adapt --target isf|blocking_reason|--strict --check|kg-bench|kg_quality|\binspect\b|\.rs:[0-9]+|rationale:|document_class|evidence_[a-z_]+|semantic_[a-z_]+|intent_[a-z_]+|measure_isf_completeness|[0-9]+ *(->|→) *[0-9]+'
NOREGRESS_SIG='kg-bench|156/156|WIRE-BASED-100|1\.000|byte-identical|run_ci|cargo (test|clippy|fmt)|lib 1[0-9]{3}|orthogonal|gold'

fails=""
add_fail() { fails="$fails
  - $1"; }

# Required box 1 — ROOT CAUSE (WHY + WHERE) ticked + a SpecForge diagnosis signature present.
if   unchecked 'root cause|why ?\+ ?where|\bwhy\b|diagnos'; then add_fail "ROOT CAUSE box is present but UNTICKED ([ ]) — the cause is not yet established."
elif ! checked 'root cause|why ?\+ ?where|\bwhy\b|diagnos'; then add_fail "ROOT CAUSE (WHY+WHERE) box is MISSING/unticked from the acceptance checklist."
elif ! printf '%s\n' "$leaf_text" | grep -Eq "$DIAGNOSIS_SIG"; then
  add_fail "ROOT CAUSE box is ticked but NOT backed by a SpecForge tool signature (validate finding/metric, adapt blocking_reason, kg-bench diagnostic, cargo test, --dry-run measurement, file:line, N->M)."
fi

# Required box 2 — ADDRESSED (verified) ticked.
if   unchecked 'addressed|verified|resolved|measured|before.{0,5}after'; then add_fail "ADDRESSED/VERIFIED box is present but UNTICKED — the change is not yet confirmed to do what it should."
elif ! checked 'addressed|verified|resolved|measured|before.{0,5}after'; then add_fail "ADDRESSED (verified, measured per-item) box is MISSING/unticked."
fi

# Required box 3 — NO REGRESSION ticked + a SpecForge oracle signature present.
if   unchecked 'no.?regress|regression|orthogonal'; then add_fail "NO REGRESSION box is present but UNTICKED — regressions / gold orthogonality are not yet cleared."
elif ! checked 'no.?regress|regression|orthogonal'; then add_fail "NO REGRESSION box is MISSING/unticked from the acceptance checklist."
elif ! printf '%s\n' "$leaf_text" | grep -Eiq "$NOREGRESS_SIG"; then
  add_fail "NO REGRESSION box is ticked but NOT backed by a SpecForge oracle signature (kg-bench 156/156, WIRE-BASED-100 1.000, byte-identical golds, run_ci / cargo green)."
fi

if [ -n "$fails" ]; then
  cat >&2 <<'MSG'
task-acceptance: the staged task leaf does NOT pass the required ACCEPTANCE CHECKLIST for a Rust code
  change. A change must be PROVABLY taken through the procedure (diagnose -> address -> no regression),
  not "trust me". Add/complete the checklist (template in TOOLBOX.md):
    - [x] ROOT CAUSE (WHY + WHERE)  — backed by a SpecForge tool signature (validate finding/metric,
          adapt --target isf blocking_reason, kg-bench fixture diagnostic, cargo test <name>,
          evidence/--dry-run measurement, file:line, measured N->M)
    - [x] ADDRESSED (verified)      — the change does what it should, measured per item (before->after)
    - [x] NO REGRESSION             — kg-bench 156/156; WIRE-BASED-100 = 1.000; golds byte-identical;
          run_ci.sh / cargo fmt|clippy|test green
  An UNTICKED required box means the task is not done — finish the step, do not bypass. The
  deterministic oracles re-run in run_ci.sh / CI, so the NO-REGRESSION numbers you cite are re-verified.
  Breaches:
MSG
  printf '%s\n' "$fails" >&2
  exit 1
fi

echo "task-acceptance: OK (task leaf passes the acceptance checklist: ROOT CAUSE + ADDRESSED + NO REGRESSION, evidence-backed)"
exit 0
