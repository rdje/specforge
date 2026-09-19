#!/usr/bin/env bash
# scripts/probe_held_out_bundle_replay.sh — can a HELD-OUT normalized bundle still rebuild its document?
#
# CORPUS-CHAIN-CURRENCY.10b. Three of the four wire-based golds — AXI, APB and AHB — carry no normalized
# bundle at the root their own SourceIR declares, so `specforge evidence` refuses them and
# `check_chain_currency.sh` reports their EvidenceIR stage UNMEASURABLE. `.10` read that as "these
# documents cannot be re-derived at all" and decided to re-ingest all three from PDF, accepting a
# regression risk on golds held at 1.000. `.10a` measured instead: the bundles exist, held out under
# `generated/preserved/` by WIRE-BASED-100.9b/.9c/.9d/.10 because the retention declaration is frozen
# (RETAINED-BUNDLE-POPULATION-FROZEN), and every one of them replays CONTENT SAME in seconds.
#
# The distinction that nearly cost a destructive act is UNDECLARED vs UNREBUILDABLE, and nothing in
# this repository could answer it without a hand-run. This is that oracle. Its second reader is
# RETAINED-BUNDLE-POPULATION-FROZEN.3, which performs the restore for real: this lets that leaf know
# the answer BEFORE it mutates the corpus instead of discovering it afterwards.
#
# ── Why it is safe to run ─────────────────────────────────────────────────────────────────────
# The probe must put a bundle where the producer reads it, which is a write into a corpus the
# CHAIN-CURRENCY doctrine also watches: a bundle left on disk that no leaf declared fails that gate
# closed. So pre-state restoration is not cleanup, it is the property that makes the probe runnable at
# all — and it is a trap rather than a tail sequence, because it must hold on failure, on a refused
# document, and on an interrupt. The bundle is COPIED, never moved, so the held-out original is never
# exposed. The probe writes no artifact: every replay is `--dry-run`.
#
# ── What it refuses to call success ───────────────────────────────────────────────────────────
# A bundle already installed at the normalized root is SKIPPED, never clobbered — overwriting a
# declared bundle is the one move here with no way back. Skips are counted and reported SEPARATELY
# from proofs, because a summary that says "3 probed, 0 failures" when all three were skipped reads as
# proof while proving nothing, and a check that cannot distinguish those is not a check
# (CLAIM_VERIFICATION.md §2). The proved count is the number this probe's claim binds.
#
# Usage:
#   scripts/probe_held_out_bundle_replay.sh              probe every held-out bundle (stages; ~1 min)
#   scripts/probe_held_out_bundle_replay.sh --census     which bundles exist and resolve (no staging)
#   scripts/probe_held_out_bundle_replay.sh --self-test  exercise the fail-closed cases
#
# Knobs, all guarded by self-test case 16 (CORPUS-CHAIN-CURRENCY.9's lesson: an unguarded default is
# how a check silently stops proving what it claims, and the guard must match BOTH expansion forms):
#   SPECFORGE_PROBE_BIN          replay binary      (default: the corpus-replay release build)
#   SPECFORGE_PROBE_PRESERVED    held-out root      (default: generated/preserved)
#   SPECFORGE_PROBE_GENERATED    corpus root        (default: generated)

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
# Every temp workspace this probe makes is derived from the repository root and stays on the
# repository volume, which is the PROJECT-DATA-LOCALITY contract rather than a convention.
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

. "$ROOT/scripts/lib/stage_artifact_identity.sh"
. "$ROOT/scripts/lib/corpus_replay_binary.sh"

PRESERVED="${SPECFORGE_PROBE_PRESERVED:-generated/preserved}"
GENERATED="${SPECFORGE_PROBE_GENERATED:-generated}"

note() { printf 'held-out-replay: %s\n' "$1"; }

# ── Interrupt safety ──────────────────────────────────────────────────────────────────────────
# A RETURN trap does not fire when the shell is killed, and the one state this probe must never leave
# behind is a staged bundle. The staged path is therefore tracked globally and released on INT/TERM/
# EXIT as well as on return.
PROBE_STAGED=""
probe_release_staged() {
  [ -n "$PROBE_STAGED" ] && rm -rf "$PROBE_STAGED"
  PROBE_STAGED=""
}
trap 'probe_release_staged' INT TERM EXIT

# hires_now — seconds with millisecond resolution. `date +%s` would round APB's 0.30 s to `0s`, which
# is the measurement this probe exists to make; GNU `date +%s%N` is not on a stock macOS and this
# repository's scripts are Bash-3.2-safe by declaration. Perl is a hard dependency of every check here.
hires_now() { perl -MTime::HiRes=time -e 'printf "%.3f", time'; }

# document_key_of <meta.json> — the bundle's own declaration of which document it normalizes.
document_key_of() {
  perl -0777 -ne 'print $1 if /"document_key"\s*:\s*"([^"]+)"/' "$1"
}

# ── The lifecycle core ────────────────────────────────────────────────────────────────────────
# probe_one <bundle-dir> <binary> — prints one verdict line.
#   0 = proved (CONTENT SAME) | 2 = skipped (already installed) | 1 = failed or refused
# Restores pre-state on EVERY path, which is why the removal is a trap and not a final statement.
probe_one() {
  local bundle="$1" bin="$2"
  local meta="" candidate key source_ir persisted target work status output start end

  for candidate in "$bundle"/*.meta.json; do
    [ -f "$candidate" ] && { meta="$candidate"; break; }
  done
  if [ -z "$meta" ]; then
    note "$bundle — REFUSED: no *.meta.json, so the document key cannot be derived"
    return 1
  fi
  key="$(document_key_of "$meta")"
  if [ -z "$key" ]; then
    note "$bundle — REFUSED: meta.json declares no document_key"
    return 1
  fi

  source_ir="$GENERATED/source_ir/$key/source_ir.json"
  persisted="$GENERATED/evidence_ir/$key/evidence_ir.json"
  target="$GENERATED/source_ir/$key/normalized"

  if [ ! -f "$source_ir" ]; then
    note "$key — REFUSED: no persisted SourceIR at $source_ir"
    return 1
  fi
  if [ ! -f "$persisted" ]; then
    note "$key — REFUSED: no persisted EvidenceIR to compare against"
    return 1
  fi
  if [ -e "$target" ]; then
    note "$key — SKIP: normalized root already populated; this bundle is installed, not held out"
    return 2
  fi

  work="$(mktemp -d "$ROOT/.project-data/tmp/held-out-replay.XXXXXX")" || return 1
  PROBE_STAGED="$target"
  # shellcheck disable=SC2064
  trap "probe_release_staged; rm -rf '$work'" RETURN

  cp -R "$bundle" "$target" || { note "$key — REFUSED: could not stage the bundle"; return 1; }

  start="$(hires_now)"
  "$bin" evidence "$source_ir" --dry-run >"$work/raw" 2>"$work/err"
  status=$?
  end="$(hires_now)"
  if [ "$status" -ne 0 ]; then
    note "$key — FAIL: replay exited $status ($(head -1 "$work/err" 2>/dev/null))"
    return 1
  fi

  awk 'flag { print } /_json:$/ { flag = 1 }' "$work/raw" >"$work/replay.json"
  if [ ! -s "$work/replay.json" ]; then
    note "$key — FAIL: the replay emitted no artifact JSON (expected a '*_json:' section)"
    return 1
  fi

  if output="$(compare_stage_artifact "$persisted" "$work/replay.json")"; then
    note "$key — CONTENT SAME in $(perl -e 'printf "%.2f", $ARGV[1] - $ARGV[0]' "$start" "$end")s: held out, not lost"
    return 0
  fi
  note "$key — CONTENT DIFFERS: $output"
  return 1
}

# ── Entry ─────────────────────────────────────────────────────────────────────────────────────
run_probe() {
  local bin bundle log rc proved=0 skipped=0 failed=0 seen=0
  log="$(mktemp "$ROOT/.project-data/tmp/held-out-build.XXXXXX")" || return 1
  if [ -n "${SPECFORGE_PROBE_BIN:-}" ]; then
    bin="$SPECFORGE_PROBE_BIN"
  elif ! bin="$(corpus_replay_build "$log")"; then
    note "REFUSED: the tree does not build, so it has no verdict to give — see $log"
    return 1
  fi
  rm -f "$log"

  for bundle in "$PRESERVED"/*/*-normalized-bundle-held-out; do
    [ -d "$bundle" ] || continue
    seen=$((seen + 1))
    probe_one "$bundle" "$bin"
    rc=$?
    case "$rc" in
      0) proved=$((proved + 1)) ;;
      2) skipped=$((skipped + 1)) ;;
      *) failed=$((failed + 1)) ;;
    esac
  done

  if [ "$seen" -eq 0 ]; then
    note "no held-out bundle found under $PRESERVED — 0 proved, nothing to prove"
    return 0
  fi
  # Proved and skipped are reported separately on purpose: "N probed, 0 failures" cannot tell a run
  # that proved N documents from one that proved none, and the claim this probe carries binds PROVED.
  note "$seen held-out bundle(s): $proved proved CONTENT SAME, $skipped skipped (already installed), $failed failed"
  [ "$failed" -eq 0 ]
}

# ── Census ────────────────────────────────────────────────────────────────────────────────────
# The bare probe stages ~280 MB per preservation point and takes about a minute, which is the right
# cost for a question asked before a corpus mutation and the wrong cost for one asked at every commit.
# `--census` answers the affordable half without staging anything: which held-out bundles exist, which
# document each one declares, and whether that document still has the persisted SourceIR and
# EvidenceIR a replay would need. It proves the bundles are REACHABLE; only the bare probe proves they
# reproduce, and the two are deliberately different questions.
run_census() {
  local bundle meta candidate key source_ir persisted found=0 resolvable=0 digest
  for bundle in "$PRESERVED"/*/*-normalized-bundle-held-out; do
    [ -d "$bundle" ] || continue
    found=$((found + 1))
    meta=""
    for candidate in "$bundle"/*.meta.json; do
      [ -f "$candidate" ] && { meta="$candidate"; break; }
    done
    if [ -z "$meta" ]; then
      note "$bundle — UNRESOLVED: no *.meta.json"
      continue
    fi
    key="$(document_key_of "$meta")"
    source_ir="$GENERATED/source_ir/$key/source_ir.json"
    persisted="$GENERATED/evidence_ir/$key/evidence_ir.json"
    if [ -z "$key" ] || [ ! -f "$source_ir" ] || [ ! -f "$persisted" ]; then
      note "$bundle — UNRESOLVED: key '$key' has no persisted SourceIR/EvidenceIR pair"
      continue
    fi
    digest="$(shasum -a 256 "$bundle/$key.md" 2>/dev/null | cut -d" " -f1)"
    resolvable=$((resolvable + 1))
    note "$key — reachable: bundle $bundle, markdown ${digest:-unreadable}"
  done
  note "census: $found held-out bundle(s), $resolvable resolvable to a persisted SourceIR/EvidenceIR pair"
  [ "$found" -gt 0 ] && [ "$found" -eq "$resolvable" ]
}

# ── Self-test ─────────────────────────────────────────────────────────────────────────────────
# The probe's verdict is only worth the cases it can still go RED on, and two classes matter for
# different reasons. The COMPARISON must reject a document that does not rebuild to the same content.
# The LIFECYCLE must leave the corpus exactly as it found it — on success, on failure, on refusal, and
# without ever clobbering a bundle that is already installed. A probe that proved the first and lost
# the second would be worse than no probe: it would redden CHAIN-CURRENCY for a document it did not
# change. The accounting cases exist because a summary that cannot separate proofs from skips is not
# evidence for a claim that counts proofs.
selftest_bundle() {
  local dir="$1" key="$2"
  mkdir -p "$dir"
  printf '{"document_key": "%s"}\n' "$key" >"$dir/$key.meta.json"
  printf '# %s\n' "$key" >"$dir/$key.md"
}

selftest_stub() {
  local path="$1" mode="$2"
  case "$mode" in
    same)    printf '#!/bin/sh\necho "command: evidence"\necho "evidence_ir_json:"\ncat <<J\n{"schema_version":3,"stage":"evidence_ir","timing_constraints":[1,2],"signal_constraints":[1]}\nJ\n' >"$path" ;;
    differs) printf '#!/bin/sh\necho "command: evidence"\necho "evidence_ir_json:"\ncat <<J\n{"schema_version":3,"stage":"evidence_ir","timing_constraints":[1],"signal_constraints":[1]}\nJ\n' >"$path" ;;
    nojson)  printf '#!/bin/sh\necho "command: evidence"\necho "no artifact section here"\n' >"$path" ;;
    fails)   printf '#!/bin/sh\necho "loader refused" >&2\nexit 1\n' >"$path" ;;
  esac
  chmod +x "$path"
}

SELFTEST_PASSED=0
SELFTEST_TOTAL=0
SELFTEST_KEY=fixture_doc
SELFTEST_PERSISTED='{"schema_version":3,"stage":"evidence_ir","timing_constraints":[1,2],"signal_constraints":[1]}'

selftest_fixture() {
  FX="$SELFTEST_WORK/$1"
  rm -rf "$FX"
  mkdir -p "$FX/source_ir/$SELFTEST_KEY" "$FX/evidence_ir/$SELFTEST_KEY" "$FX/preserved/OWNER"
  printf '{}\n' >"$FX/source_ir/$SELFTEST_KEY/source_ir.json"
  printf '%s\n' "$SELFTEST_PERSISTED" >"$FX/evidence_ir/$SELFTEST_KEY/evidence_ir.json"
  selftest_bundle "$FX/preserved/OWNER/fx-normalized-bundle-held-out" "$SELFTEST_KEY"
  GENERATED="$FX"
  PRESERVED="$FX/preserved"
}

selftest_expect() {  # <n> <label> <expected-rc> <actual-rc> <substring> <output>
  SELFTEST_TOTAL=$((SELFTEST_TOTAL + 1))
  if [ "$4" -ne "$3" ]; then
    printf 'held-out-replay self-test %s FAILED (%s): expected rc %s, got %s\n' "$1" "$2" "$3" "$4"
    return
  fi
  case "$6" in
    *"$5"*) SELFTEST_PASSED=$((SELFTEST_PASSED + 1)) ;;
    *) printf 'held-out-replay self-test %s FAILED (%s): output lacks %s\n' "$1" "$2" "$5" ;;
  esac
}

selftest_assert() {  # <n> <label> <condition-rc>
  SELFTEST_TOTAL=$((SELFTEST_TOTAL + 1))
  if [ "$3" -eq 0 ]; then SELFTEST_PASSED=$((SELFTEST_PASSED + 1))
  else printf 'held-out-replay self-test %s FAILED: %s\n' "$1" "$2"; fi
}

run_self_test() {
  local out rc bundle
  SELFTEST_WORK="$(mktemp -d "$ROOT/.project-data/tmp/held-out-selftest.XXXXXX")" || return 1
  trap 'rm -rf "$SELFTEST_WORK"; probe_release_staged' INT TERM EXIT
  bundle=preserved/OWNER/fx-normalized-bundle-held-out

  # 1) positive — the bundle rebuilds the document to the same content.
  selftest_fixture ok; selftest_stub "$SELFTEST_WORK/bin" same
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 1 'content same' 0 "$rc" 'CONTENT SAME' "$out"

  # 2) …and it left the corpus exactly as it found it. This property is what makes the probe runnable
  #    at all: a staged bundle nobody declared fails CHAIN-CURRENCY closed.
  [ ! -e "$FX/source_ir/$SELFTEST_KEY/normalized" ]
  selftest_assert 2 'a passing probe left the bundle staged' $?

  # 3) the held-out original survives — the probe copies, it never moves.
  [ -f "$FX/$bundle/$SELFTEST_KEY.md" ]
  selftest_assert 3 'the probe consumed the held-out original' $?

  # 4) RED — the document does not rebuild to the same content, and the differing section is named.
  selftest_fixture differs; selftest_stub "$SELFTEST_WORK/bin" differs
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 4 'content differs' 1 "$rc" 'timing_constraints(2->1)' "$out"

  # 5) …and a FAILING probe restores pre-state too. A cleanup that only runs on success is not one.
  [ ! -e "$FX/source_ir/$SELFTEST_KEY/normalized" ]
  selftest_assert 5 'a failing probe left the bundle staged' $?

  # 6) RED — the replay itself refuses.
  selftest_fixture fails; selftest_stub "$SELFTEST_WORK/bin" fails
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 6 'replay refuses' 1 "$rc" 'replay exited 1' "$out"

  # 7) RED — the replay succeeds but emits no artifact section. Without this case an empty comparison
  #    could read as agreement.
  selftest_fixture nojson; selftest_stub "$SELFTEST_WORK/bin" nojson
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 7 'no artifact JSON' 1 "$rc" 'emitted no artifact JSON' "$out"

  # 8) RED — the bundle does not declare which document it normalizes.
  selftest_fixture nokey; selftest_stub "$SELFTEST_WORK/bin" same
  rm -f "$FX/$bundle/$SELFTEST_KEY.meta.json"
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 8 'no meta.json' 1 "$rc" 'document key cannot be derived' "$out"

  # 9) RED — no persisted SourceIR to replay from.
  selftest_fixture nosource; selftest_stub "$SELFTEST_WORK/bin" same
  rm -f "$FX/source_ir/$SELFTEST_KEY/source_ir.json"
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 9 'no SourceIR' 1 "$rc" 'no persisted SourceIR' "$out"

  # 10) RED — nothing to compare the replay against.
  selftest_fixture nopersisted; selftest_stub "$SELFTEST_WORK/bin" same
  rm -f "$FX/evidence_ir/$SELFTEST_KEY/evidence_ir.json"
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 10 'no persisted EvidenceIR' 1 "$rc" 'no persisted EvidenceIR' "$out"

  # 11) an already-installed bundle is SKIPPED with its own code, and 12) is not clobbered.
  #     Overwriting a declared bundle is the one move here with no way back.
  selftest_fixture installed; selftest_stub "$SELFTEST_WORK/bin" same
  mkdir -p "$FX/source_ir/$SELFTEST_KEY/normalized"
  printf 'DECLARED\n' >"$FX/source_ir/$SELFTEST_KEY/normalized/sentinel"
  out="$(probe_one "$FX/$bundle" "$SELFTEST_WORK/bin")"; rc=$?
  selftest_expect 11 'installed bundle skipped' 2 "$rc" 'installed, not held out' "$out"
  [ -f "$FX/source_ir/$SELFTEST_KEY/normalized/sentinel" ]
  selftest_assert 12 'an installed bundle was clobbered' $?

  # 13) ACCOUNTING — a run that proved something says so.
  selftest_fixture accounting_proved; selftest_stub "$SELFTEST_WORK/bin" same
  out="$(SPECFORGE_PROBE_BIN="$SELFTEST_WORK/bin" run_probe)"; rc=$?
  selftest_expect 13 'summary reports proofs' 0 "$rc" '1 proved CONTENT SAME, 0 skipped' "$out"

  # 14) ACCOUNTING, and it is the reason these cases exist — a run in which EVERY bundle was skipped
  #     must not read as proof. "N probed, 0 failures" cannot tell the two apart; PROVED can.
  selftest_fixture accounting_skipped; selftest_stub "$SELFTEST_WORK/bin" same
  mkdir -p "$FX/source_ir/$SELFTEST_KEY/normalized"
  out="$(SPECFORGE_PROBE_BIN="$SELFTEST_WORK/bin" run_probe)"; rc=$?
  selftest_expect 14 'all-skipped run proves nothing' 0 "$rc" '0 proved CONTENT SAME, 1 skipped' "$out"

  # 15) ACCOUNTING — a failing document is counted and the run is RED.
  selftest_fixture accounting_failed; selftest_stub "$SELFTEST_WORK/bin" differs
  out="$(SPECFORGE_PROBE_BIN="$SELFTEST_WORK/bin" run_probe)"; rc=$?
  selftest_expect 15 'failures make the run red' 1 "$rc" '1 failed' "$out"

  # 17) CENSUS — a reachable bundle is reported reachable and the run is green.
  selftest_fixture census_ok
  out="$(run_census)"; rc=$?
  selftest_expect 17 'census resolves a reachable bundle' 0 "$rc" '1 held-out bundle(s), 1 resolvable' "$out"

  # 18) CENSUS RED — a bundle whose document has no persisted pair is UNRESOLVED and reddens the run.
  #     Without this the census could report a bundle as reachable that nothing could ever replay.
  selftest_fixture census_unresolvable
  rm -f "$FX/evidence_ir/$SELFTEST_KEY/evidence_ir.json"
  out="$(run_census)"; rc=$?
  selftest_expect 18 'census refuses an unresolvable bundle' 1 "$rc" 'UNRESOLVED' "$out"

  # 16) every knob this script reads has its default exercised, so a default cannot silently stop
  #     proving what the check claims (CORPUS-CHAIN-CURRENCY.9). Both expansion forms are matched.
  local unguarded
  unguarded="$(grep -oE '\$\{SPECFORGE_PROBE_[A-Z_]+:?-' "$0" | sed 's/:*-$//' | sed 's/^\${//' \
    | sort -u | grep -vE '^SPECFORGE_PROBE_(BIN|PRESERVED|GENERATED)$' || true)"
  [ -z "$unguarded" ]
  selftest_assert 16 "unguarded knob(s): $unguarded" $?

  printf 'held-out-replay: self-test %s/%s comparison, lifecycle, refusal, accounting, and knob cases pass.\n' \
    "$SELFTEST_PASSED" "$SELFTEST_TOTAL"
  [ "$SELFTEST_PASSED" -eq "$SELFTEST_TOTAL" ]
}

case "${1-}" in
  --self-test) run_self_test ;;
  --census)    run_census ;;
  '')          run_probe ;;
  *)           printf 'Usage: %s [--census|--self-test]\n' "$0" >&2; exit 2 ;;
esac
