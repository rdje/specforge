#!/usr/bin/env bash
# scripts/rebuild_stage_cascade.sh — restore the downstream chain's proof seal by REBUILDING it.
#
# SOURCE-IR-REPRODUCIBILITY.15. `.14` re-sealed the persisted SourceIR corpus with
# `source_proof_migrate --write`, a PROOF-ONLY path: it re-derives the ledger from each artifact's
# own retained capture and touches no public content. No such path exists for EvidenceIR,
# SemanticIR, IntentIR, or the ISF adapter — only `SourceIr` has `rebuild_from_retained_capture`.
# So when their seals go stale the only remedy is to re-run the stages, which WRITES REAL ARTIFACT
# CONTENT. That is a different and heavier act than a re-seal, and this script exists to make it a
# measured one rather than a hopeful one.
#
# Why a seal goes stale without any content being wrong: a stage's ledger records the
# `ruleset_sha256` of the rule registry that proved it, and every downstream stage records the
# CUMULATIVE hash of its upstream's ruleset folded with its own
# (`cumulative_ruleset_sha256`, crates/specforge/src/ir/derivation.rs). Re-sealing SourceIR
# necessarily moves every downstream cumulative hash, so the whole chain below it reads as stale
# even where nothing about the artifacts changed.
#
# The two properties this script PROVES rather than assumes, per stage:
#   1. content identity — the rebuilt artifact carries the same content as the artifact it replaced,
#      compared with the SAME predicate the CHAIN-CURRENCY oracle uses
#      (scripts/lib/stage_artifact_identity.sh), against a pre-write snapshot taken here;
#   2. canonical acceptance — the rebuilt artifact loads and validates through the product's own
#      `specforge validate`, which is what re-earns the seal and restores the `validation_reports`
#      back-annotation the rebuild necessarily dropped.
# A content delta is NEVER absorbed: the run stops at the stage where it appeared, names the
# differing top-level sections, keeps the snapshot, and hands the attribution to the owning leaf
# (ADR 0025 decision 1). The precedent that makes this non-theoretical: the ADR 0025 reconciliation
# found exactly one real delta across 24 documents (`table_0044` becoming `register_map`).
#
# AUTHORITY: this is a REMEDY, not an oracle. `scripts/check_chain_currency.sh` remains the
# CHAIN-CURRENCY doctrine and the only thing that may declare the corpus current. This script
# borrows its comparison predicate solely to NAME what moved.
#
# Scope: the proof-carrying stratum — documents whose persisted `source_ir.json` has a
# `proof_ledger`. A legacy proofless artifact has no current seal to restore and is already
# explicitly unmeasurable under ADR 0025; those are reported as a count and skipped, never silently.
#
# Order: stage-major (all of `evidence`, then `semantic`, then `intent`, then `isf-adapter`), and
# within a document the rebuild is immediately followed by its `validate`, so the next stage reads a
# validated upstream exactly as the persisted corpus was built. A stage that reports any delta or
# failure STOPS the cascade before the next stage is built on top of it.
#
# THAT ORDER IS LOAD-BEARING, and the reason is measured, not stylistic: `specforge validate` is NOT
# IDEMPOTENT. Every invocation appends one `validation_backannotation` mutation to the artifact's proof
# context and changes its digest (measured 4 -> 5 -> 6 mutations over three consecutive calls on one
# EvidenceIR). Because each downstream stage retains its upstream's ledger as an EXACT PREFIX,
# validating an upstream artifact a second time invalidates every downstream artifact that was built
# before that validation — they then fail to load with `cumulative proof ledger does not retain the
# exact verified upstream prefix`. So each artifact here is validated EXACTLY ONCE, strictly
# upstream-to-downstream. Never re-validate an upstream stage without rebuilding everything below it.
#
# The corollary governs every READ-ONLY caller, including this script's own `--check`: asking the
# product's canonical loader with `specforge validate` MUTATES the corpus. The read-only way to ask the
# same loader the same question is to run the DOWNSTREAM stage with `--dry-run`: it enters the identical
# verified-load path and leaves the artifact byte-identical (measured: digest unchanged across a
# `semantic --dry-run`, while a `validate` moved it). `--check` uses that probe and never validates.
#
# Modes:
#   --check      (default) read-only pre-flight: the in-scope stratum, the recorded per-stage
#                ruleset-seal census, and one canonical `validate` per stage. Writes nothing.
#   --write      perform the cascade. This overwrites persisted artifacts under the corpus root.
#   --self-test  prove this script's own controls are fail-closed before trusting a PASS.
#
# Skips LOUDLY when the corpus root is absent: a fresh clone and hosted CI have no `generated/`.
#
# Knobs: SPECFORGE_REBUILD_CASCADE_GENERATED_ROOT=<dir> to point at another corpus root.
#        SPECFORGE_REBUILD_CASCADE_BIN=<path> to point at another `specforge` binary.
#        SPECFORGE_REBUILD_CASCADE_WORK=<dir> to keep the pre-write snapshot at a known path.
# Bash-3.2-safe (no associative arrays, no mapfile) so a stock macOS clone runs it.
set -uo pipefail
export LC_ALL=C

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

GENERATED_ROOT="${SPECFORGE_REBUILD_CASCADE_GENERATED_ROOT:-generated}"

MODE=check
while [ "$#" -gt 0 ]; do
  case "$1" in
    --check)     MODE=check ;;
    --write)     MODE=write ;;
    --self-test) MODE=self-test ;;
    *) printf 'Usage: %s [--check|--write|--self-test]\n' "$0" >&2; exit 2 ;;
  esac
  shift
done

note()      { printf '[rebuild-cascade] %s\n' "$1"; }
fail_note() { printf '[rebuild-cascade] FAIL: %s\n' "$1" >&2; }

# The ONE content-identity predicate, shared with the CHAIN-CURRENCY oracle so a remedy can never
# certify itself with a comparison the gate would not make (`.11`).
. "$ROOT/scripts/lib/stage_artifact_identity.sh"

# ── Stratum ─────────────────────────────────────────────────────────────────
# A document is in scope when its persisted artifact carries a current proof ledger, and the seal
# it carries is that ledger's own `ruleset_sha256`. Both questions are answered by ONE streaming,
# depth-aware scan (`scan_proof_ledger`) rather than by matching indented lines: a stage artifact
# can reach 87 MB, so the file is never parsed whole, but a format-dependent line match would read a
# compactly-serialized ledger as absent — which would silently drop a sealed document out of scope.
# The scanner tracks JSON string/escape state and brace depth, so it reads the TOP-LEVEL
# `proof_ledger` object's DIRECT `ruleset_sha256` member and cannot be fooled by a digest-shaped
# field elsewhere in the artifact.
#
# scan_proof_ledger <artifact.json> — prints the recorded seal, `none` when the ledger has no
# `ruleset_sha256`, or nothing at all when there is no top-level proof ledger. Exits 0 when a
# top-level `proof_ledger` object exists, 1 otherwise.
scan_proof_ledger() {
  perl - "$1" <<'PERL'
use strict;
use warnings;

open my $handle, '<:raw', $ARGV[0] or exit 1;
binmode $handle;

my $depth        = 0;      # brace/bracket depth; top-level object members sit at depth 1
my $in_string    = 0;
my $escaped      = 0;
my $token        = '';     # the string literal currently being accumulated
my $pending_key  = undef;  # the most recent string seen in key position
my $in_ledger    = 0;      # depth at which the proof_ledger object's members live, or 0
my $found_ledger = 0;
my $seal;
my $expect_value = 0;      # the next literal closes `"ruleset_sha256":`

my $chunk;
CHUNK: while (read($handle, $chunk, 65536)) {
    for my $ch (split //, $chunk) {
        if ($in_string) {
            if ($escaped)        { $escaped = 0; $token .= $ch; next }
            if ($ch eq '\\')     { $escaped = 1; $token .= $ch; next }
            if ($ch ne '"')      { $token .= $ch; next }
            $in_string = 0;
            if ($expect_value) { $seal = $token; last CHUNK }
            $pending_key = $token;
            next;
        }
        if ($ch eq '"') { $in_string = 1; $token = ''; next }
        if ($ch eq '{' || $ch eq '[') {
            $depth++;
            if (!$found_ledger && $ch eq '{' && $depth == 2
                && defined $pending_key && $pending_key eq 'proof_ledger') {
                $found_ledger = 1;
                $in_ledger    = $depth;
            }
            $pending_key = undef;
            next;
        }
        if ($ch eq '}' || $ch eq ']') {
            last CHUNK if $in_ledger && $depth == $in_ledger;
            $depth--;
            $pending_key = undef;
            next;
        }
        if ($ch eq ':') {
            $expect_value = ($in_ledger && $depth == $in_ledger
                             && defined $pending_key && $pending_key eq 'ruleset_sha256') ? 1 : 0;
            next;
        }
        $pending_key = undef if $ch eq ',';
    }
}
close $handle;

exit 1 if !$found_ledger;
print defined $seal ? "$seal\n" : "none\n";
exit 0;
PERL
}

# carries_proof_ledger <artifact.json> — stratum membership.
carries_proof_ledger() {
  scan_proof_ledger "$1" >/dev/null 2>&1
}

# recorded_ruleset <artifact.json> — the seal the artifact records, or `none` when proofless.
recorded_ruleset() {
  local seal
  if seal="$(scan_proof_ledger "$1" 2>/dev/null)" && [ -n "$seal" ]; then
    printf '%s\n' "$seal"
  else
    printf 'none\n'
  fi
}

# ── Stage table ─────────────────────────────────────────────────────────────
# stage_input_path / stage_output_path <stage> <key>: the fixed input the stage reads and the
# persisted artifact it writes. Mirrors scripts/check_chain_currency.sh's stage table exactly.
stage_input_path() {
  case "$1" in
    evidence)    printf '%s\n' "$GENERATED_ROOT/source_ir/$2/source_ir.json" ;;
    semantic)    printf '%s\n' "$GENERATED_ROOT/evidence_ir/$2/evidence_ir.json" ;;
    intent)      printf '%s\n' "$GENERATED_ROOT/semantic_ir/$2/semantic_ir.json" ;;
    isf-adapter) printf '%s\n' "$GENERATED_ROOT/intent_ir/$2/intent_ir.json" ;;
    *) return 1 ;;
  esac
}
stage_output_path() {
  case "$1" in
    evidence)    printf '%s\n' "$GENERATED_ROOT/evidence_ir/$2/evidence_ir.json" ;;
    semantic)    printf '%s\n' "$GENERATED_ROOT/semantic_ir/$2/semantic_ir.json" ;;
    intent)      printf '%s\n' "$GENERATED_ROOT/intent_ir/$2/intent_ir.json" ;;
    isf-adapter) printf '%s\n' "$GENERATED_ROOT/adapters/isf/$2/adapter.json" ;;
    *) return 1 ;;
  esac
}

# downstream_probe <stage> <artifact> <errfile> — READ-ONLY canonical acceptance for a persisted
# artifact: run the stage that CONSUMES it with `--dry-run`, which enters the same verified loader
# `specforge validate` would and writes nothing. Returns 2 for the terminal stage, which has no
# consumer and therefore no read-only canonical probe — reported honestly rather than papered over
# with a `validate` that would mutate the artifact.
downstream_probe() {
  case "$1" in
    evidence)    "$BIN" semantic "$2" --dry-run >/dev/null 2>"$3" ;;
    semantic)    "$BIN" intent   "$2" --dry-run >/dev/null 2>"$3" ;;
    intent)      "$BIN" adapt    "$2" --target isf --dry-run >/dev/null 2>"$3" ;;
    isf-adapter) return 2 ;;
    *) return 2 ;;
  esac
}

# run_stage <stage> <input> <errfile> — invoke the production CLI surface for one stage.
run_stage() {
  case "$1" in
    evidence)    "$BIN" evidence "$2" >/dev/null 2>"$3" ;;
    semantic)    "$BIN" semantic "$2" >/dev/null 2>"$3" ;;
    intent)      "$BIN" intent   "$2" >/dev/null 2>"$3" ;;
    isf-adapter) "$BIN" adapt    "$2" --target isf >/dev/null 2>"$3" ;;
    *) return 2 ;;
  esac
}

# ── Self-test: prove the controls are fail-CLOSED before trusting a PASS ────
run_self_test() {
  local work passed=0 output status
  work="$(mktemp -d)" || { fail_note 'cannot create a repository-local self-test workspace'; return 1; }

  # 1) The shared identity predicate is really sourced here, and really discriminates.
  printf '%s' '{"stage":"evidence_ir","spans":[1,2],"validation_reports":[]}' > "$work/base.json"
  printf '%s' '{"stage":"evidence_ir","spans":[1],"validation_reports":[]}'   > "$work/shrunk.json"
  output="$(compare_stage_artifact "$work/base.json" "$work/shrunk.json")"; status=$?
  if [ "$status" -ne 0 ] && [ "$output" = 'spans(2->1)' ]; then passed=$((passed + 1))
  else fail_note "self-test 1: the shared predicate did not name a shrunken section (got '$output')"; fi

  # 2) ...and a seal-only difference is NOT a content delta, which is the whole premise of this
  #    remedy: re-sealing moves `proof_ledger` and nothing else.
  printf '%s' '{"stage":"evidence_ir","spans":[1,2],"validation_reports":[{"r":1}],"proof_ledger":{"ruleset_sha256":"a"}}' > "$work/sealed_a.json"
  printf '%s' '{"stage":"evidence_ir","spans":[1,2],"validation_reports":[],"proof_ledger":{"ruleset_sha256":"b"}}'        > "$work/sealed_b.json"
  if compare_stage_artifact "$work/sealed_a.json" "$work/sealed_b.json" >/dev/null; then passed=$((passed + 1))
  else fail_note 'self-test 2: a seal-and-report-only difference was reported as a content delta'; fi

  # 3) Stratum selection is structural: a proof-carrying artifact is in scope.
  printf '%s\n' '{' '  "stage": "source_ir",' '  "proof_ledger": {' '    "ruleset_sha256": "'"$(printf 'a%.0s' $(seq 64))"'"' '  }' '}' > "$work/proved.json"
  if carries_proof_ledger "$work/proved.json"; then passed=$((passed + 1))
  else fail_note 'self-test 3: a proof-carrying artifact was not selected into the stratum'; fi

  # 4) ...and a legacy proofless artifact is NOT, so a rebuild can never be claimed for a document
  #    that has no current seal to restore.
  printf '%s\n' '{' '  "stage": "source_ir",' '  "content_elements": []' '}' > "$work/proofless.json"
  if carries_proof_ledger "$work/proofless.json"; then
    fail_note 'self-test 4: a legacy proofless artifact was selected into the stratum'
  else passed=$((passed + 1)); fi

  # 5) The recorded seal is read from the ledger, not from any other 64-hex field that happens to
  #    precede it — a digest-shaped neighbour must not be mistaken for the seal.
  printf '%s\n' '{' '  "document_identity": { "sha256": "'"$(printf 'b%.0s' $(seq 64))"'" },' \
    '  "proof_ledger": { "schema_version": 1, "ruleset_sha256": "'"$(printf 'c%.0s' $(seq 64))"'" }' '}' \
    > "$work/two_digests.json"
  output="$(recorded_ruleset "$work/two_digests.json")"
  if [ "$output" = "$(printf 'c%.0s' $(seq 64))" ]; then passed=$((passed + 1))
  else fail_note "self-test 5: the recorded seal was misread (got '$output')"; fi

  # 6) ...and a proofless artifact records no seal rather than borrowing a neighbour's.
  output="$(recorded_ruleset "$work/proofless.json")"
  if [ "$output" = 'none' ]; then passed=$((passed + 1))
  else fail_note "self-test 6: a proofless artifact reported a seal (got '$output')"; fi

  # 7) The stage table agrees with the pipeline: every stage's OUTPUT is the next stage's INPUT.
  #    A table that drifts would rebuild a stage from the wrong artifact and still look plausible.
  local chain_ok=1 previous='' this_input
  for stage in evidence semantic intent isf-adapter; do
    this_input="$(stage_input_path "$stage" k)" || chain_ok=0
    if [ -n "$previous" ] && [ "$this_input" != "$previous" ]; then chain_ok=0; fi
    previous="$(stage_output_path "$stage" k)" || chain_ok=0
  done
  if [ "$chain_ok" -eq 1 ]; then passed=$((passed + 1))
  else fail_note 'self-test 7: the stage table is not a chain — some stage does not read its predecessor'; fi

  # 8) An unknown stage is refused rather than silently skipped.
  if stage_input_path bogus k >/dev/null 2>&1; then
    fail_note 'self-test 8: an unknown stage resolved to an input path'
  else passed=$((passed + 1)); fi

  # 11) A COMPACTLY serialized ledger is found and read. This is the case that sent the earlier
  #     line-matching reader RED: it reported `none` for a sealed artifact, which would have
  #     silently dropped that document out of the in-scope stratum.
  printf '%s' '{"stage":"source_ir","proof_ledger":{"schema_version":1,"ruleset_sha256":"'"$(printf 'd%.0s' $(seq 64))"'","claims":[]}}' > "$work/compact.json"
  output="$(recorded_ruleset "$work/compact.json")"
  if [ "$output" = "$(printf 'd%.0s' $(seq 64))" ] && carries_proof_ledger "$work/compact.json"; then passed=$((passed + 1))
  else fail_note "self-test 11: a compactly serialized ledger was misread (got '$output')"; fi

  # 12) A digest-shaped member NESTED inside the ledger's claims is not the ledger's own seal. The
  #     scan is depth-aware precisely so a claim cannot impersonate the ruleset it was proved under.
  printf '%s' '{"stage":"source_ir","proof_ledger":{"claims":[{"ruleset_sha256":"'"$(printf 'e%.0s' $(seq 64))"'"}],"ruleset_sha256":"'"$(printf 'f%.0s' $(seq 64))"'"}}' > "$work/nested.json"
  output="$(recorded_ruleset "$work/nested.json")"
  if [ "$output" = "$(printf 'f%.0s' $(seq 64))" ]; then passed=$((passed + 1))
  else fail_note "self-test 12: a nested claim digest was mistaken for the ledger seal (got '$output')"; fi

  # 9) An absent corpus root SKIPS loudly and passes, end to end through this script.
  output="$(SPECFORGE_REBUILD_CASCADE_GENERATED_ROOT="$work/absent-corpus" "$ROOT/scripts/rebuild_stage_cascade.sh" --check 2>&1)"; status=$?
  case "$output" in *'SKIP'*) : ;; *) status=99 ;; esac
  if [ "$status" -eq 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 9: an absent corpus root did not skip loudly (status $status, output '$output')"; fi

  # 10) `--check` writes NOTHING. Proved end to end against a real miniature corpus root, by
  #     digesting the whole tree before and after.
  local mini="$work/mini" before after
  mkdir -p "$mini/source_ir/d" "$mini/evidence_ir/d"
  cp "$work/proved.json" "$mini/source_ir/d/source_ir.json"
  cp "$work/proved.json" "$mini/evidence_ir/d/evidence_ir.json"
  before="$(find "$mini" -type f -exec shasum -a 256 {} + | sort | shasum -a 256)"
  SPECFORGE_REBUILD_CASCADE_GENERATED_ROOT="$mini" "$ROOT/scripts/rebuild_stage_cascade.sh" --check >/dev/null 2>&1
  after="$(find "$mini" -type f -exec shasum -a 256 {} + | sort | shasum -a 256)"
  if [ "$before" = "$after" ]; then passed=$((passed + 1))
  else fail_note 'self-test 10: --check modified the corpus root'; fi

  # 13) `--check` must NEVER ask the binary to `validate`. This is the control that today's first
  #     implementation would have FAILED: it probed each stage with `specforge validate`, which appends
  #     a validation_backannotation mutation and moves the artifact's digest — so a "read-only"
  #     pre-flight silently invalidated every downstream artifact's retained upstream prefix. Self-test
  #     10's digest check could not see it, because a miniature corpus's artifacts do not load at all.
  #     A recording stub makes the question behavioural instead of structural.
  local stub="$work/recording-specforge"
  printf '%s\n' '#!/usr/bin/env bash' 'printf "%s\n" "$*" >> "$SPECFORGE_STUB_LOG"' 'exit 0' > "$stub"
  chmod +x "$stub"
  : > "$work/stub.log"
  SPECFORGE_STUB_LOG="$work/stub.log" \
    SPECFORGE_REBUILD_CASCADE_GENERATED_ROOT="$mini" \
    SPECFORGE_REBUILD_CASCADE_BIN="$stub" \
    "$ROOT/scripts/rebuild_stage_cascade.sh" --check >/dev/null 2>&1
  if grep -q '^validate ' "$work/stub.log"; then
    fail_note "self-test 13: --check invoked 'specforge validate', which mutates the artifact it claims only to read"
  else
    passed=$((passed + 1))
  fi

  # 14) ...and the probe it uses instead really is the downstream stage in --dry-run form, so the
  #     read-only claim rests on the canonical loader rather than on skipping the question entirely.
  if grep -Eq '^(semantic|intent|adapt) .*--dry-run' "$work/stub.log"; then passed=$((passed + 1))
  else fail_note "self-test 14: --check asked no canonical loader at all (stub log: $(tr '\n' ';' < "$work/stub.log"))"; fi

  rm -rf "$work"
  if [ "$passed" -eq 14 ]; then note "self-test 14/14 passed."; return 0; fi
  fail_note "self-test $passed/14 passed"
  return 1
}

if [ "$MODE" = 'self-test' ]; then
  run_self_test
  exit "$?"
fi

# ── Corpus presence ─────────────────────────────────────────────────────────
if [ ! -d "$GENERATED_ROOT/source_ir" ]; then
  note "SKIP: no corpus at $GENERATED_ROOT/source_ir — a fresh clone and hosted CI have none, so"
  note 'SKIP: there is no persisted chain to rebuild. Nothing was written; nothing is claimed.'
  exit 0
fi

# ── The in-scope stratum ────────────────────────────────────────────────────
WORK="${SPECFORGE_REBUILD_CASCADE_WORK:-}"
if [ -z "$WORK" ]; then
  WORK="$(mktemp -d)" || { fail_note 'cannot create a repository-local workspace'; exit 1; }
else
  mkdir -p "$WORK" || { fail_note "cannot create workspace $WORK"; exit 1; }
fi
KEYS="$WORK/in_scope_keys.txt"
: > "$KEYS"
proofless=0
for source_ir in "$GENERATED_ROOT"/source_ir/*/source_ir.json; do
  [ -f "$source_ir" ] || continue
  key="$(basename "$(dirname "$source_ir")")"
  if carries_proof_ledger "$source_ir"; then
    printf '%s\n' "$key" >> "$KEYS"
  else
    proofless=$((proofless + 1))
  fi
done
in_scope="$(wc -l < "$KEYS" | tr -d ' ')"
note "corpus root: $GENERATED_ROOT — $in_scope proof-carrying document(s) in scope, $proofless legacy proofless document(s) explicitly out of scope"
if [ "$in_scope" -eq 0 ]; then
  note 'SKIP: no proof-carrying document has a seal to restore. Nothing was written; nothing is claimed.'
  rm -rf "$WORK"
  exit 0
fi

# ── The binary ──────────────────────────────────────────────────────────────
BIN="${SPECFORGE_REBUILD_CASCADE_BIN:-}"
if [ -z "$BIN" ]; then
  BUILD_LOG="$WORK/build.log"
  if ! cargo build --manifest-path Cargo.toml --bin specforge >"$BUILD_LOG" 2>&1; then
    fail_note 'cargo could not build the specforge binary, so no rebuild is possible:'
    cat "$BUILD_LOG" >&2
    exit 1
  fi
  BIN="${CARGO_TARGET_DIR:-$ROOT/target}/debug/specforge"
fi
if [ ! -x "$BIN" ]; then
  fail_note "$BIN is not an executable specforge binary"
  exit 1
fi

# ── Pre-flight (--check): the seal census, and the product's own verdict ────
if [ "$MODE" = 'check' ]; then
  sample="$(head -1 "$KEYS")"
  for stage in evidence semantic intent isf-adapter; do
    distinct_file="$WORK/seals.$stage"; : > "$distinct_file"
    present=0
    while read -r key; do
      artifact="$(stage_output_path "$stage" "$key")"
      [ -f "$artifact" ] || continue
      present=$((present + 1))
      recorded_ruleset "$artifact" >> "$distinct_file"
    done < "$KEYS"
    distinct="$(sort -u "$distinct_file" | wc -l | tr -d ' ')"
    seal="$(sort -u "$distinct_file" | head -1)"
    artifact="$(stage_output_path "$stage" "$sample")"
    verdict='no persisted artifact'
    if [ -f "$artifact" ]; then
      downstream_probe "$stage" "$artifact" "$WORK/probe.err"; probe_status=$?
      case "$probe_status" in
        0) verdict='canonical loader ACCEPTS (read-only downstream --dry-run probe)' ;;
        2) verdict='terminal stage — no consumer, so no read-only canonical probe exists' ;;
        *) verdict="canonical loader REJECTS: $(tr '\n' ' ' < "$WORK/probe.err" | sed 's/  */ /g' | cut -c1-120)" ;;
      esac
    fi
    note "$stage — $present/$in_scope persisted, $distinct distinct seal(s) (${seal:-none}); sample '$sample': $verdict"
  done
  note 'pre-flight only: nothing was written. Run with --write to perform the cascade.'
  [ -n "${SPECFORGE_REBUILD_CASCADE_WORK:-}" ] || rm -rf "$WORK"
  exit 0
fi

# ── The cascade (--write) ───────────────────────────────────────────────────
note 'WRITE MODE: this overwrites persisted artifacts. A pre-write snapshot is taken per artifact.'
SNAPSHOT="$WORK/pre_write_snapshot"
mkdir -p "$SNAPSHOT"
fail=0
for stage in evidence semantic intent isf-adapter; do
  rebuilt=0; identical=0; changed=0; validated=0; failed=0; missing=0
  while read -r key; do
    input="$(stage_input_path "$stage" "$key")"
    output="$(stage_output_path "$stage" "$key")"
    if [ ! -f "$input" ]; then
      fail_note "$key $stage — the stage input is absent, so nothing can be rebuilt: $input"
      failed=$((failed + 1)); fail=1; continue
    fi
    if [ ! -f "$output" ]; then
      # Nothing persisted here: the chain does not reach this stage for this document. Rebuilding
      # would EXTEND the corpus, which is a different act than restoring a seal, and not this one's.
      missing=$((missing + 1)); continue
    fi

    snapshot_dir="$SNAPSHOT/$stage/$key"
    mkdir -p "$snapshot_dir"
    cp "$output" "$snapshot_dir/$(basename "$output")" || {
      fail_note "$key $stage — could not snapshot the persisted artifact before overwriting it"
      failed=$((failed + 1)); fail=1; continue
    }
    if [ "$stage" = 'isf-adapter' ]; then
      for sibling in "$(dirname "$output")"/*.isf; do
        [ -f "$sibling" ] && cp "$sibling" "$snapshot_dir/"
      done
    fi

    before_seal="$(recorded_ruleset "$snapshot_dir/$(basename "$output")")"
    if ! run_stage "$stage" "$input" "$WORK/stage.err"; then
      fail_note "$key $stage — the rebuild failed: $(tr '\n' ' ' < "$WORK/stage.err" | sed 's/  */ /g' | cut -c1-200)"
      failed=$((failed + 1)); fail=1; continue
    fi
    rebuilt=$((rebuilt + 1))

    if differing="$(compare_stage_artifact "$snapshot_dir/$(basename "$output")" "$output")"; then
      identical=$((identical + 1))
    else
      fail_note "$key $stage — the rebuilt artifact is NOT content-identical to the one it replaced: $differing"
      changed=$((changed + 1)); fail=1
    fi
    if [ "$stage" = 'isf-adapter' ]; then
      if ! breach="$(compare_emitted_isf "$output" "$snapshot_dir")"; then
        fail_note "$key isf-emit — the rebuilt adapter's emission differs from the snapshot: $breach"
        changed=$((changed + 1)); fail=1
      fi
    fi

    if "$BIN" validate "$output" >/dev/null 2>"$WORK/validate.err"; then
      validated=$((validated + 1))
    else
      fail_note "$key $stage — the rebuilt artifact does not validate: $(tr '\n' ' ' < "$WORK/validate.err" | sed 's/  */ /g' | cut -c1-200)"
      failed=$((failed + 1)); fail=1
    fi
    after_seal="$(recorded_ruleset "$output")"
    printf '%s\t%s\t%s\t%s\n' "$stage" "$key" "$before_seal" "$after_seal" >> "$WORK/seal_transitions.tsv"
  done < "$KEYS"

  note "$stage — $rebuilt rebuilt, $identical content-identical, $changed content-changed, $validated validated, $failed failed, $missing not persisted at this stage"
  if [ "$fail" -ne 0 ]; then
    fail_note "$stage — stopping the cascade here. A later stage must not be rebuilt on top of an"
    fail_note "$stage — unattributed delta (ADR 0025 decision 1). Pre-write snapshot retained at: $WORK"
    exit 1
  fi
done

if [ -f "$WORK/seal_transitions.tsv" ]; then
  note "seal transitions (stage: distinct before -> distinct after):"
  for stage in evidence semantic intent isf-adapter; do
    before_n="$(awk -F'\t' -v s="$stage" '$1==s {print $3}' "$WORK/seal_transitions.tsv" | sort -u | tr '\n' ' ')"
    after_n="$(awk -F'\t' -v s="$stage" '$1==s {print $4}' "$WORK/seal_transitions.tsv" | sort -u | tr '\n' ' ')"
    [ -n "$before_n" ] && note "  $stage: ${before_n%% } -> ${after_n%% }"
  done
fi
note 'cascade complete: every rebuilt artifact is content-identical to the one it replaced and'
note 'validates through the product loader. CHAIN-CURRENCY remains the oracle — run'
note 'scripts/check_chain_currency.sh to certify the corpus current.'
[ -n "${SPECFORGE_REBUILD_CASCADE_WORK:-}" ] || rm -rf "$WORK"
exit 0
