#!/usr/bin/env bash
# scripts/check_proof_seal_currency.sh — the PROOF-SEAL-CURRENCY doctrine.
#
# SOURCE-IR-REPRODUCIBILITY.16. Make the seal debt visible AT THE MOMENT IT IS CREATED.
#
# ── What went wrong that this check exists to prevent ──────────────────────────────────────────
# Every persisted stage artifact records the `ruleset_sha256` of the rule registry that proved it,
# and every downstream stage records the CUMULATIVE hash of its upstream's ruleset folded with its
# own (`cumulative_ruleset_sha256`, crates/specforge/src/ir/derivation.rs). That digest is
# implementation authority, generated at build time by crates/specforge-core/build.rs from each
# stage's rule-registry constructor and its stage-local production items — so an ordinary edit to a
# stage root un-seals the WHOLE persisted corpus while every artifact's content stays exactly right.
#
# `.14` found the corpus in that state and measured the cost of not seeing it: 24/24 artifacts
# refusing `SourceIR proof verification failed: proof ledger ruleset hash is stale`, undetected for
# 13 days and 54 commits. Nothing was wrong with the artifacts; nothing an ordinary slice ran could
# observe the transition. `CHAIN-CURRENCY` does report it, loudly and unbypassably — but it is
# registered `ci` (DOCTRINE_ENFORCEMENT.md §4.7), so it first speaks at the push boundary, which is
# a deliberate consequence of the CI policy rather than a hole. The director's call, `2026-08-28`,
# on the measured cost — 7.2 s to verify all 24 canonically against ~20 minutes to learn the same
# fact from `check_chain_currency.sh` — was to add a cheap seal-only check at GATE tier. This is it.
#
# ── What it proves, and what it deliberately does not ──────────────────────────────────────────
# PROVES: every persisted artifact in the proof-carrying stratum records a seal the CURRENT build's
# canonical loader still accepts. The census is TOTAL — every in-scope artifact at every stage is
# read — and the canonical probe is REPRESENTATIVE, one per distinct seal per stage. That is not a
# sample: the census is what establishes representativeness, so a per-document seal divergence
# raises the distinct count and earns its own probe rather than hiding behind a homogeneous one.
#
# DOES NOT PROVE: that a persisted artifact is still the CONTENT the current binary reproduces from
# its persisted input. That is a different and much heavier question, it belongs to CHAIN-CURRENCY
# (ADR 0025 decision 2, `scripts/check_chain_currency.sh`), and it stays at CI tier. A seal is
# current here and the artifact can still be content-stale; this check never says otherwise.
#
# Nor does it probe the TERMINAL stage. `adapters/isf` has no consumer, so it has no read-only
# canonical probe — and CHAIN-CURRENCY does not close that gap either, because its content
# comparison excludes the proof surface by construction. The adapter's seal is censused here and
# its unprobed status is REPORTED, never quietly counted as a pass.
#
# ── Archetype: oracle (re-run), not structural ─────────────────────────────────────────────────
# It asks the PRODUCT'S OWN canonical loader rather than reimplementing the digest comparison —
# `.11`'s lesson about a second copy of a predicate that can drift. That is why it builds the
# binary: the question is whether THIS commit's build accepts the persisted seal.
#
# THE READ-ONLY PROBE IS THE ONE THING THIS CHECK MUST NOT GET WRONG. `specforge validate` is NOT
# idempotent: each call appends a `validation_backannotation` mutation and moves the artifact's
# digest, and every downstream stage retains its upstream ledger as an exact prefix — so a gate that
# validated persisted artifacts on every commit would invalidate the chain below them on every
# commit. `.15` did exactly that by accident and had to rebuild the corpus. The read-only way to ask
# the SAME loader is to run the CONSUMING stage with `--dry-run`, measured to leave the artifact
# byte-identical. Self-tests 11 and 12 hold that shut with a recording stub.
#
# ── Cost (measured 2026-08-29, this repository, 24 in-scope documents) ─────────────────────────
#   necessary-condition prefilter, one process over all 78 source_ir.json   0.24 s
#   exact depth-aware seal scan, 24 proof-carrying source_ir.json           1.23 s
#   full five-stage seal census (120 artifacts, ~2.5 GB)                    ~4.5 s
#   four read-only canonical probes                                         ~5.2 s
#   this check, end to end, over the real corpus                            14.1 s
# The gate tier it joins measured 4 m 21 s without this check and 3 m 02 s with it, so run-to-run
# variance dominates that comparison and the honest figure is this check's own 14.1 s. The prefilter
# is what keeps the 54 legacy proofless artifacts from costing 2.96 s of parser time to prove a
# ledger they do not have.
#
# Modes:
#   --check      (default, gate tier) the total census and a SAMPLED probe — one representative per
#                distinct seal per stage. Cheap enough for a pre-commit hook. Writes nothing.
#   --total      (CI tier) the same census with EVERY in-scope document probed at every non-terminal
#                stage. Writes nothing. This is the mode that can see a per-document divergence.
#   --self-test  prove this script's own controls are fail-closed before trusting a PASS.
#
# WHY TWO MODES, measured rather than assumed (SIGNAL-DECLARATION-ROW-DROP.1c). This script used to
# argue that one probe per distinct seal "is not a sample: the census is what establishes
# representativeness". That argument is FALSE, and the counterexample is in this repository's
# history. At `48def695` all 27 evidence artifacts carried ONE seal, so one probe ran and the gate
# reported green — while the canonical loader REFUSED 4 of the 27, every wire-bearing one, for three
# commits. The seal is homogeneous precisely because it is a digest over the RULESET and ignores
# artifact content; the loader also verifies a per-document REPLAY TOPOLOGY
# (`evidence.claim.<field>.root`'s recorded `inputs_sha256`) which does not. One probe per seal can
# therefore never see a content-driven divergence. It is `CLAIM_VERIFICATION.md` §2's fourth row
# exactly: a per-item assertion checked against per-container data, reproducing perfectly while
# getting it wrong. The sampled mode stays because it is what a pre-commit hook can afford — but it
# now SAYS it is a sample and names the class it cannot see, and `--total` is registered CI-tier so
# the class is enforced rather than merely documented.
#
# Skips LOUDLY when the corpus root is absent: `generated/` is untracked, so a fresh clone and a
# hosted CI runner have none and this doctrine does not govern them. Silence would read as a pass.
#
# Knobs: SPECFORGE_PROOF_SEAL_GENERATED_ROOT=<dir> to point at another corpus root.
#        SPECFORGE_PROOF_SEAL_BIN=<path> to point at another `specforge` binary (skips the build).
# Bash-3.2-safe (no associative arrays, no mapfile) so a stock macOS clone runs it.
set -uo pipefail
export LC_ALL=C

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

GENERATED_ROOT="${SPECFORGE_PROOF_SEAL_GENERATED_ROOT:-generated}"

MODE=check
PROBE_SCOPE=default
while [ "$#" -gt 0 ]; do
  case "$1" in
    --check)     MODE=check; PROBE_SCOPE=default ;;
    --total)     MODE=check; PROBE_SCOPE=total ;;
    --self-test) MODE=self-test ;;
    *) printf 'Usage: %s [--check|--total|--self-test]\n' "$0" >&2; exit 2 ;;
  esac
  shift
done

note()      { printf '[proof-seal] %s\n' "$1"; }
fail_note() { printf '[proof-seal] FAIL: %s\n' "$1" >&2; }

# The ONE seal-read predicate and chain stage table, shared with `scripts/rebuild_stage_cascade.sh`
# — the remedy that clears exactly the debt this gate reports. A gate and a remedy that disagreed
# about which artifacts are in scope, which seal they carry, or which loader answers for them would
# leave a debt no compliant work could clear (`.11`).
. "$ROOT/scripts/lib/proof_seal_scan.sh"

# The stale-seal diagnostic the product emits when a persisted ledger was proved under a ruleset the
# current build no longer issues. Matching it is what lets the gate name the exact remedy instead of
# printing a loader error and leaving the reader to classify it. A probe rejection that does NOT
# match is still a breach — it is reported as an unclassified one, never downgraded.
STALE_SEAL_PATTERN='(cumulative )?proof ledger ruleset hash is stale'

# The diagnostic a probe emits when its own INPUT is missing rather than its subject being unsealed.
# A held-out normalized bundle makes `evidence --dry-run` unrunnable for that document; the check
# then has no verdict, and saying so is the honest outcome (never a pass, never a breach).
ABSENT_PRECONDITION_PATTERN='path does not exist'

# remedy_for <stage> — the task-owned command that re-earns a stale seal at this stage.
remedy_for() {
  case "$1" in
    source-ir)
      printf '%s\n' 'cargo run --manifest-path Cargo.toml --features source-proof-migration --example source_proof_migrate -- --write --retained-manifest (proof-only re-seal; SOURCE-IR-REPRODUCIBILITY.14)' ;;
    *)
      printf '%s\n' 'bash scripts/rebuild_stage_cascade.sh --check, then --write (stage rebuild; SOURCE-IR-REPRODUCIBILITY.15) — re-sealing SourceIR moves every downstream cumulative digest, so run it upstream-first' ;;
  esac
}

# ── Self-test: prove the controls are fail-CLOSED before trusting a PASS ────
run_self_test() {
  local work passed=0 total=20 output status
  work="$(mktemp -d)" || { fail_note 'cannot create a repository-local self-test workspace'; return 1; }

  local hex_a hex_b hex_c
  hex_a="$(printf 'a%.0s' $(seq 64))"
  hex_b="$(printf 'b%.0s' $(seq 64))"
  hex_c="$(printf 'c%.0s' $(seq 64))"

  # 1) The shared seal predicate is really sourced here, and really reads a compactly serialized
  #    ledger — the case that sent an earlier line-matching reader RED by reporting `none` for a
  #    sealed artifact, which would have dropped that document out of the in-scope stratum.
  printf '%s' '{"stage":"source_ir","proof_ledger":{"schema_version":1,"ruleset_sha256":"'"$hex_a"'","claims":[]}}' > "$work/compact.json"
  output="$(recorded_ruleset "$work/compact.json")"
  if [ "$output" = "$hex_a" ] && carries_proof_ledger "$work/compact.json"; then passed=$((passed + 1))
  else fail_note "self-test 1: a compactly serialized ledger was misread (got '$output')"; fi

  # 2) A legacy proofless artifact is NOT in the stratum and borrows no neighbour's seal.
  printf '%s\n' '{' '  "stage": "source_ir",' '  "content_elements": []' '}' > "$work/proofless.json"
  output="$(recorded_ruleset "$work/proofless.json")"
  if [ "$output" = 'none' ] && ! carries_proof_ledger "$work/proofless.json"; then passed=$((passed + 1))
  else fail_note "self-test 2: a proofless artifact was selected or reported a seal (got '$output')"; fi

  # 3) A digest-shaped member nested inside the ledger's claims is not the ledger's own seal. The
  #    scan is depth-aware precisely so a claim cannot impersonate the ruleset it was proved under.
  printf '%s' '{"stage":"source_ir","proof_ledger":{"claims":[{"ruleset_sha256":"'"$hex_b"'"}],"ruleset_sha256":"'"$hex_c"'"}}' > "$work/nested.json"
  output="$(recorded_ruleset "$work/nested.json")"
  if [ "$output" = "$hex_c" ]; then passed=$((passed + 1))
  else fail_note "self-test 3: a nested claim digest was mistaken for the ledger seal (got '$output')"; fi

  # 4) The prefilter is a SUPERSET, never an authority. An artifact carrying a NESTED `proof_ledger`
  #    — bytes the prefilter cannot tell from the real thing — is a candidate, and the exact scan
  #    must still reject it, because only a TOP-LEVEL ledger is a seal. Without this control the
  #    cheap pass would be free to promote a document into a stratum it does not belong to.
  printf '%s' '{"stage":"source_ir","upstream":{"proof_ledger":{"ruleset_sha256":"'"$hex_b"'"}}}' > "$work/nested_ledger.json"
  if [ -n "$(proof_ledger_candidates "$work/nested_ledger.json")" ] \
     && ! carries_proof_ledger "$work/nested_ledger.json" \
     && [ "$(recorded_ruleset "$work/nested_ledger.json")" = 'none' ]; then passed=$((passed + 1))
  else fail_note 'self-test 4: a nested proof_ledger was accepted as a top-level seal, or the prefilter missed it'; fi

  # 5) ...and the batched composition the census actually uses agrees with the per-file predicate on
  #    a mixed list: 4 inputs -> 3 prefilter candidates -> 2 sealed. The prefilter never drops a
  #    sealed artifact, and its one extra candidate never survives the exact scan.
  local candidates sealed_rows
  candidates="$(proof_ledger_candidates "$work/proofless.json" "$work/compact.json" \
                  "$work/nested.json" "$work/nested_ledger.json" | wc -l | tr -d ' ')"
  sealed_rows="$(sealed_artifacts "$work/proofless.json" "$work/compact.json" \
                   "$work/nested.json" "$work/nested_ledger.json" | wc -l | tr -d ' ')"
  if [ "$candidates" = '3' ] && [ "$sealed_rows" = '2' ]; then passed=$((passed + 1))
  else fail_note "self-test 5: the batched census disagreed with the per-file predicate ($candidates candidates, $sealed_rows sealed; expected 3 and 2)"; fi

  # 6) The shared stage table is a CHAIN: every stage consumes exactly what its producer wrote.
  #    A table that drifted would census one artifact and probe another and still look plausible.
  local chain_ok=1 previous='' this producer
  for this in $(chain_stages); do
    if producer="$(chain_stage_producer "$this")"; then
      [ "$(chain_stage_artifact_path "$producer" k)" = "$previous" ] || chain_ok=0
    else
      [ -z "$previous" ] || chain_ok=0
    fi
    previous="$(chain_stage_artifact_path "$this" k)" || chain_ok=0
  done
  if [ "$chain_ok" -eq 1 ]; then passed=$((passed + 1))
  else fail_note 'self-test 6: the shared stage table is not a chain'; fi

  # 7) An unknown stage is refused rather than silently resolving to a path.
  if chain_stage_artifact_path bogus k >/dev/null 2>&1; then
    fail_note 'self-test 7: an unknown stage resolved to an artifact path'
  else passed=$((passed + 1)); fi

  # 8) The terminal stage reports "no read-only canonical probe" (2) rather than a pass (0). This is
  #    the honest-gap control: `adapters/isf` has no consumer, and the only loader that would answer
  #    for it is `specforge validate`, which mutates.
  if BIN=/nonexistent chain_stage_readonly_probe isf-adapter "$work/compact.json" "$work/probe.err"; then
    fail_note 'self-test 8: the terminal stage reported a canonical pass it cannot have'
  else
    status=$?
    if [ "$status" -eq 2 ]; then passed=$((passed + 1))
    else fail_note "self-test 8: the terminal stage did not report an absent probe (status $status)"; fi
  fi

  # A miniature corpus: three documents, all sealed. `doc_c` carries the SAME seal as `doc_a`, which
  # is what makes the sampled/total distinction testable at all — a homogeneous neighbour is exactly
  # where a divergent document hides.
  local mini="$work/mini"
  mkdir -p "$mini/source_ir/doc_a" "$mini/source_ir/doc_b" "$mini/source_ir/doc_c"
  cp "$work/compact.json" "$mini/source_ir/doc_a/source_ir.json"
  cp "$work/nested.json"  "$mini/source_ir/doc_b/source_ir.json"
  cp "$work/compact.json" "$mini/source_ir/doc_c/source_ir.json"
  # CORPUS-CHAIN-CURRENCY.6 — the mini corpus reaches the SEMANTIC stage as well, because the probe
  # scope is now a per-stage decision and a corpus that stops at source-ir cannot exercise it. All
  # three semantic artifacts carry the SAME seal, which is the condition the sampling control needs.
  mkdir -p "$mini/semantic_ir/doc_a" "$mini/semantic_ir/doc_b" "$mini/semantic_ir/doc_c"
  cp "$work/compact.json" "$mini/semantic_ir/doc_a/semantic_ir.json"
  cp "$work/compact.json" "$mini/semantic_ir/doc_b/semantic_ir.json"
  cp "$work/compact.json" "$mini/semantic_ir/doc_c/semantic_ir.json"

  # A recording stub that ACCEPTS: every probe succeeds, so the gate must pass and must have asked.
  local accept_stub="$work/accepting-specforge"
  printf '%s\n' '#!/usr/bin/env bash' 'printf "%s\n" "$*" >> "$SPECFORGE_STUB_LOG"' 'exit 0' > "$accept_stub"
  chmod +x "$accept_stub"
  : > "$work/accept.log"
  output="$(SPECFORGE_STUB_LOG="$work/accept.log" \
            SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$mini" \
            SPECFORGE_PROOF_SEAL_BIN="$accept_stub" \
            "$ROOT/scripts/check_proof_seal_currency.sh" --check 2>&1)"; status=$?

  # 9) A corpus whose seals the loader accepts passes.
  if [ "$status" -eq 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 9: an accepted corpus did not pass (status $status, output '$output')"; fi

  # 10) The census is TOTAL, not sampled: two documents with DIFFERENT seals must be reported as two
  #     distinct seals, which is what makes one probe per seal representative rather than hopeful.
  case "$output" in
    *'2 distinct seal'*) passed=$((passed + 1)) ;;
    *) fail_note "self-test 10: two divergent seals were not both censused (output '$output')" ;;
  esac

  # 11) ...and each distinct seal earns its OWN probe, so a divergent document cannot hide behind a
  #     homogeneous neighbour. Two seals at `source-ir` means two `evidence --dry-run` probes.
  output="$(grep -c -- '--dry-run' "$work/accept.log" || true)"
  if [ "$output" -ge 2 ]; then passed=$((passed + 1))
  else fail_note "self-test 11: divergent seals did not each earn a probe ($output dry-run probes)"; fi

  # 12) The gate must NEVER ask the binary to `validate`. This is the control that would have caught
  #     `.15`'s corpus-corrupting pre-flight: `validate` appends a mutation and moves the artifact's
  #     digest, so a "read-only" gate running on every commit would invalidate the chain each time.
  if grep -q '^validate ' "$work/accept.log"; then
    fail_note "self-test 12: the gate invoked 'specforge validate', which mutates the artifact it claims only to read"
  else passed=$((passed + 1)); fi

  # 13) `--check` writes NOTHING. Proved end to end by digesting the whole corpus root before/after.
  local before after
  before="$(find "$mini" -type f -exec shasum -a 256 {} + | sort | shasum -a 256)"
  SPECFORGE_STUB_LOG="$work/accept.log" \
    SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$mini" \
    SPECFORGE_PROOF_SEAL_BIN="$accept_stub" \
    "$ROOT/scripts/check_proof_seal_currency.sh" --check >/dev/null 2>&1
  after="$(find "$mini" -type f -exec shasum -a 256 {} + | sort | shasum -a 256)"
  if [ "$before" = "$after" ]; then passed=$((passed + 1))
  else fail_note 'self-test 13: --check modified the corpus root'; fi

  # 14) THE FAIL-CLOSED CONTROL. A loader that rejects a persisted seal must FAIL the gate and must
  #     name the owning remedy — otherwise this whole check is decoration.
  local reject_stub="$work/rejecting-specforge"
  printf '%s\n' '#!/usr/bin/env bash' \
    'printf "SourceIR proof verification failed: proof ledger ruleset hash is stale\n" >&2' \
    'exit 1' > "$reject_stub"
  chmod +x "$reject_stub"
  output="$(SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$mini" \
            SPECFORGE_PROOF_SEAL_BIN="$reject_stub" \
            "$ROOT/scripts/check_proof_seal_currency.sh" --check 2>&1)"; status=$?
  if [ "$status" -ne 0 ]; then passed=$((passed + 1))
  else fail_note 'self-test 14: a rejected persisted seal did not fail the gate'; fi

  # 15) ...and the rejection is CLASSIFIED as the stale-seal class, so the reader gets the remedy
  #     rather than a raw loader error to interpret.
  case "$output" in
    *'source_proof_migrate'*) passed=$((passed + 1)) ;;
    *) fail_note "self-test 15: a stale seal was not classified with its remedy (output '$output')" ;;
  esac

  # 16) An absent corpus root SKIPS loudly and PASSES, end to end. `generated/` is untracked, so a
  #     fresh clone and hosted CI have none — a gate that failed there would block every clean tree.
  output="$(SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$work/absent-corpus" \
            "$ROOT/scripts/check_proof_seal_currency.sh" --check 2>&1)"; status=$?
  case "$output" in *'SKIP'*) : ;; *) status=99 ;; esac
  if [ "$status" -eq 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 16: an absent corpus root did not skip loudly (status $status, output '$output')"; fi

  # 17) THE SAMPLING CONTROL — now stated PER STAGE, because the scope is a per-stage decision
  #     (CORPUS-CHAIN-CURRENCY.6). At a SAMPLED stage a loader that accepts `doc_a` and REFUSES
  #     `doc_c` — same seal — is still invisible, and the check must keep saying so rather than
  #     pretend to a coverage it does not pay for. This is `.1b`'s shape in miniature: one seal, a
  #     divergent document, a gate green over it for three commits.
  #     The stub refuses `doc_c` only when the probe reads it from `source_ir/`, so the divergence
  #     lives at the sampled stage alone.
  local selective_stub="$work/selective-specforge"
  printf '%s\n' '#!/usr/bin/env bash' \
    'case "$*" in' \
    '  *source_ir/doc_c*) printf "EvidenceIR proof verification failed: registered derivation output or input topology is stale\n" >&2; exit 1 ;;' \
    'esac' \
    'exit 0' > "$selective_stub"
  chmod +x "$selective_stub"
  output="$(SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$mini" \
            SPECFORGE_PROOF_SEAL_BIN="$selective_stub" \
            "$ROOT/scripts/check_proof_seal_currency.sh" --check 2>&1)"; status=$?
  if [ "$status" -eq 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 17: the SAMPLED stage was expected to MISS a divergent same-seal document (status $status)"; fi

  # 17b) ...and at a TOTAL stage the same divergence is CAUGHT by `--check`, by name. The stage set
  #      is passed explicitly because the default ships EMPTY until the corpus repair lands, so this
  #      control proves the mechanism rather than the current default.
  #      This is the property the corpus measurement bought: one distinct seal across the whole
  #      stratum made the sampled probe a 1-in-27 sample, while a refusal at `semantic` or `intent`
  #      costs ~1.2 s per document to find. Same stub shape, same seal, different stage.
  local semantic_stub="$work/semantic-selective-specforge"
  printf '%s\n' '#!/usr/bin/env bash' \
    'case "$*" in' \
    '  *semantic_ir/doc_c*) printf "SemanticIR proof verification failed: registered derivation output or input topology is stale\n" >&2; exit 1 ;;' \
    'esac' \
    'exit 0' > "$semantic_stub"
  chmod +x "$semantic_stub"
  output="$(SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$mini" \
            SPECFORGE_PROOF_SEAL_BIN="$semantic_stub" \
            SPECFORGE_PROOF_SEAL_TOTAL_STAGES='semantic intent' \
            "$ROOT/scripts/check_proof_seal_currency.sh" --check 2>&1)"; status=$?
  case "$status:$output" in
    0:*) fail_note 'self-test 17b: a TOTAL stage passed over a document its own loader refuses' ;;
    *doc_c*) passed=$((passed + 1)) ;;
    *) fail_note "self-test 17b: a TOTAL stage failed without naming the divergent document (output '$output')" ;;
  esac

  # 18) ...and `--total` catches exactly that document, by name.
  output="$(SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$mini" \
            SPECFORGE_PROOF_SEAL_BIN="$selective_stub" \
            "$ROOT/scripts/check_proof_seal_currency.sh" --total 2>&1)"; status=$?
  case "$status:$output" in
    0:*) fail_note 'self-test 18: --total passed over a document its own loader refuses' ;;
    *doc_c*) passed=$((passed + 1)) ;;
    *) fail_note "self-test 18: --total failed without naming the divergent document (output '$output')" ;;
  esac

  # 19) A probe whose own INPUT is absent yields NO VERDICT — never an acceptance, never a breach.
  #     The three wire golds' normalized bundles are held out, so their `evidence --dry-run` probe
  #     cannot run; counting that as a pass would be the same error this leaf exists to remove.
  local absent_stub="$work/absent-input-specforge"
  printf '%s\n' '#!/usr/bin/env bash' \
    'printf "error: path does not exist: /nope/normalized/doc.md\n" >&2' \
    'exit 1' > "$absent_stub"
  chmod +x "$absent_stub"
  output="$(SPECFORGE_PROOF_SEAL_GENERATED_ROOT="$mini" \
            SPECFORGE_PROOF_SEAL_BIN="$absent_stub" \
            "$ROOT/scripts/check_proof_seal_currency.sh" --total 2>&1)"; status=$?
  case "$status:$output" in
    0:*'with no verdict'*) passed=$((passed + 1)) ;;
    0:*) fail_note "self-test 19: an absent probe input passed WITHOUT being reported as no-verdict (output '$output')" ;;
    *) fail_note "self-test 19: an absent probe input was reported as a seal breach (status $status)" ;;
  esac

  rm -rf "$work"
  if [ "$passed" -eq "$total" ]; then note "self-test $total/$total passed."; return 0; fi
  fail_note "self-test $passed/$total passed"
  return 1
}

if [ "$MODE" = 'self-test' ]; then
  run_self_test
  exit "$?"
fi

# ── Which stages are probed TOTALLY, and why it is a per-stage question ──────
# CORPUS-CHAIN-CURRENCY.6. The sampled tier probes one representative per distinct seal, and its
# stated bet is that a divergent document raises the distinct count and so earns its own probe.
# `.4` measured the corpus and the bet does not hold: there is ONE distinct seal per stage across
# all 27 proof-carrying artifacts, so the sample is 1 in 27 — and it passed in a tree where the
# `intent` stage refused a wire gold outright.
#
# `.5` measured the obvious repair and refuted it: a census key that carries the recorded derivation
# topology gives 27 distinct keys for 27 documents at every stage (root derivations included, because
# each root's output/inputs digests are taken over its own document's content). Such a key IS
# `--total`, which is 18m45s, so it buys nothing the tier does not already offer.
#
# What `--total`'s cost actually is, measured per stage rather than as one number: an accepted
# `intent --dry-run` is 1.2 s and a refusal 0.24 s, because a refusal stops at the loader. The
# expensive probes are `source-ir` and `evidence`, which replay EXTRACTION from the normalized
# bundle. Probing every one of the 27 at the two cheap stages costs 30.3 s + 35.5 s = 66 s and finds
# every refusal the corpus currently has.
#
# So sampling is right at source-ir and evidence and wrong at semantic and intent. The blindness the
# sampled tier documents about itself is KEPT where it is paid for and REMOVED where it is not.
#
# ── WHY THIS SHIPS INERT, AND WHAT TURNS IT ON ──────────────────────────────
# The mechanism is here, self-tested (17 and 17b), and measured; the default is EMPTY. Turning it on
# today would fail the gate on every commit, because the two documents it catches are genuinely
# broken and their repair is blocked: APB-e's semantic artifact carries the regression tracked as
# `SIGNAL-DECLARATION-ROW-DROP.4c`, and rebuilding it before that leaf lands would publish the
# regression into a wire-gold chain; I2C's normalized bundle is reclaimed, so it needs a re-ingest
# rather than a replay. A gate that fails closed over a known-broken corpus is CORRECT and is also
# unlandable — so the mechanism ships and the activation waits for the repair, rather than the
# contract being widened to accommodate a failure.
#
# ACTIVATION, when `.4c` has landed and both documents are rebuilt: set the default below to
# 'semantic intent'. That is the whole change; the controls already assert both halves.
TOTAL_PROBE_STAGES="${SPECFORGE_PROOF_SEAL_TOTAL_STAGES-}"

# probe_scope_for <stage> — 'total' or 'sample', honouring an explicit --total for every stage.
probe_scope_for() {
  if [ "$PROBE_SCOPE" = 'total' ]; then
    printf 'total\n'
    return 0
  fi
  case " $TOTAL_PROBE_STAGES " in
    *" $1 "*) printf 'total\n' ;;
    *)        printf 'sample\n' ;;
  esac
}

# ── Corpus presence ─────────────────────────────────────────────────────────
if [ ! -d "$GENERATED_ROOT/source_ir" ]; then
  note "SKIP: no corpus at $GENERATED_ROOT/source_ir — \`generated/\` is untracked, so a fresh clone"
  note 'SKIP: and a hosted CI runner have no persisted seal to check. Nothing is claimed.'
  exit 0
fi

WORK="$(mktemp -d)" || { fail_note 'cannot create a repository-local workspace'; exit 1; }
trap 'rm -rf "$WORK"' EXIT

# ── The in-scope stratum ────────────────────────────────────────────────────
# A document is in scope when its persisted `source_ir.json` carries a proof ledger — the same
# stratum `scripts/rebuild_stage_cascade.sh` rebuilds, read with the same shared predicate. A legacy
# proofless artifact has no current seal to be stale, is already explicitly unmeasurable under ADR
# 0025, and is reported as a count rather than silently dropped.
SOURCE_ARGS=()
for source_ir in "$GENERATED_ROOT"/source_ir/*/source_ir.json; do
  [ -f "$source_ir" ] && SOURCE_ARGS+=("$source_ir")
done
persisted_documents="${#SOURCE_ARGS[@]}"
KEYS="$WORK/in_scope_keys.txt"
: > "$KEYS"
if [ "$persisted_documents" -gt 0 ]; then
  sealed_artifacts "${SOURCE_ARGS[@]}" | cut -f1 | while IFS= read -r artifact; do
    basename "$(dirname "$artifact")"
  done > "$KEYS"
fi
in_scope="$(wc -l < "$KEYS" | tr -d ' ')"
proofless=$((persisted_documents - in_scope))
note "corpus root: $GENERATED_ROOT — $in_scope proof-carrying document(s) in scope, $proofless legacy proofless document(s) explicitly out of scope"
if [ "$in_scope" -eq 0 ]; then
  note 'SKIP: no persisted document carries a current seal, so there is none to be stale.'
  exit 0
fi

# ── The binary ──────────────────────────────────────────────────────────────
# The question is whether THIS commit's build accepts the persisted seal, so the build is part of
# the check. A tree that does not compile cannot answer it, and saying so is fail-closed.
BIN="${SPECFORGE_PROOF_SEAL_BIN:-}"
if [ -z "$BIN" ]; then
  if ! cargo build --manifest-path Cargo.toml --bin specforge >"$WORK/build.log" 2>&1; then
    fail_note 'cargo could not build the specforge binary, so the persisted seal cannot be asked of'
    fail_note 'the current build. Fix the build first; this doctrine has no verdict without it.'
    cat "$WORK/build.log" >&2
    exit 1
  fi
  BIN="${CARGO_TARGET_DIR:-$ROOT/target}/debug/specforge"
fi
if [ ! -x "$BIN" ]; then
  fail_note "$BIN is not an executable specforge binary"
  exit 1
fi

# ── The census, and the current build's verdict on it ───────────────────────
fail=0
for stage in $(chain_stages); do
  ARTIFACT_ARGS=()
  present=0
  while IFS= read -r key; do
    artifact="$(chain_stage_artifact_path "$stage" "$key")" || continue
    if [ -f "$artifact" ]; then
      present=$((present + 1))
      ARTIFACT_ARGS+=("$artifact")
    fi
  done < "$KEYS"

  if [ "$present" -eq 0 ]; then
    note "$stage — 0/$in_scope persisted; the chain does not reach this stage, so there is no seal to check"
    continue
  fi

  # TOTAL census: every in-scope artifact at this stage is read, so the distinct-seal count is a
  # fact about the whole stratum rather than about whichever document got probed.
  SEALS="$WORK/seals.$stage"
  sealed_artifacts "${ARTIFACT_ARGS[@]}" > "$SEALS"
  sealed="$(wc -l < "$SEALS" | tr -d ' ')"
  if [ "$sealed" -ne "$present" ]; then
    fail_note "$stage — $((present - sealed)) of $present persisted artifact(s) carry NO proof ledger while"
    fail_note "$stage — their SourceIR does. A stage that lost its seal cannot be verified; rebuild it:"
    fail_note "$stage — $(remedy_for "$stage")"
    fail=1
    continue
  fi

  # Which artifacts get asked. SAMPLE: one representative per distinct seal — cheap, and blind to a
  # divergence the seal cannot express. TOTAL: every sealed artifact at this stage.
  REPS="$WORK/reps.$stage"
  distinct="$(awk -F'\t' '!seen[$2]++' "$SEALS" | wc -l | tr -d ' ')"
  stage_scope="$(probe_scope_for "$stage")"
  if [ "$stage_scope" = 'total' ]; then
    awk -F'\t' '{ print $2 "\t" $1 }' "$SEALS" > "$REPS"
  else
    awk -F'\t' '!seen[$2]++ { print $2 "\t" $1 }' "$SEALS" > "$REPS"
  fi
  probes="$(wc -l < "$REPS" | tr -d ' ')"
  summary="$stage — $sealed/$in_scope persisted and sealed, $distinct distinct seal(s)"

  accepted=0
  rejected=0
  unprobed=0
  precondition_absent=0
  while IFS=$'\t' read -r seal representative; do
    chain_stage_readonly_probe "$stage" "$representative" "$WORK/probe.err"
    case "$?" in
      0)
        accepted=$((accepted + 1))
        ;;
      2)
        unprobed=$((unprobed + 1))
        ;;
      3)
        fail_note "$stage — no read-only probe could be resolved for this stage; the check has no verdict"
        fail=1
        ;;
      *)
        diagnostic="$(tr '\n' ' ' < "$WORK/probe.err" | sed 's/  */ /g' | cut -c1-200)"
        # A probe whose own INPUT is absent has no verdict to give. The three wire golds' normalized
        # bundles are deliberately held out of the corpus (`retained-bundle-population-is-frozen`),
        # so the `evidence --dry-run` probe for them cannot run at all. That is reported as an
        # absent verdict — never as an acceptance, and never as a seal breach it is not evidence of.
        if printf '%s' "$diagnostic" | grep -Eq "$ABSENT_PRECONDITION_PATTERN"; then
          precondition_absent=$((precondition_absent + 1))
          note "$stage — NO VERDICT for $representative: the probe's own input is absent"
          note "$stage — ($diagnostic). Censused and sealed, but unprobeable here."
          continue
        fi
        rejected=$((rejected + 1))
        fail_note "$stage — the current build REFUSES the persisted seal $(printf '%.8s' "$seal")… carried by"
        fail_note "$stage — $representative"
        fail_note "$stage — loader: $diagnostic"
        if printf '%s' "$diagnostic" | grep -Eq "$STALE_SEAL_PATTERN"; then
          fail_note "$stage — this is a STALE SEAL, not a content problem: the ruleset digest is over the"
          fail_note "$stage — implementation, so an ordinary edit to a stage root un-seals every persisted"
          fail_note "$stage — artifact while its content stays exactly right. Remedy (task-owned):"
          fail_note "$stage —   $(remedy_for "$stage")"
        else
          fail_note "$stage — this is NOT the stale-seal diagnostic, so the remedy above may not apply."
          fail_note "$stage — Do not re-seal past it; find out why the loader refuses this artifact."
        fi
        fail=1
        ;;
    esac
  done < "$REPS"

  if [ "$unprobed" -gt 0 ]; then
    note "$summary; TERMINAL stage — censused but NOT probed: it has no consumer, so no read-only"
    note "$stage — canonical probe exists, and CHAIN-CURRENCY does not close the gap either (its"
    note "$stage — content comparison excludes the proof surface). The only loader that would answer"
    note "$stage — for it is \`specforge validate\`, which mutates the artifact and the chain above it."
  elif [ "$stage_scope" = 'total' ]; then
    note "$summary; TOTAL probe: $accepted of $sealed accepted, $rejected refused, $precondition_absent with no verdict"
  else
    note "$summary; SAMPLED probe ($probes of $sealed documents, one per distinct seal): $accepted accepted, $rejected refused, $precondition_absent with no verdict"
  fi
done

if [ "$fail" -eq 0 ]; then
  if [ "$PROBE_SCOPE" = 'total' ]; then
    note 'every probeable persisted artifact is accepted by the current build, asked one document at a'
    note 'time. CHAIN-CURRENCY (CI tier) still owns whether those artifacts are the CONTENT the'
    note 'current binary reproduces.'
  elif [ -n "$TOTAL_PROBE_STAGES" ]; then
    note "every persisted artifact at [$TOTAL_PROBE_STAGES] is accepted by the current build, asked one"
    note 'document at a time; the other stages are SAMPLED, one document per distinct seal, and cannot'
    note 'see a per-document replay-topology divergence there — measured once at 23 accepted / 4'
    note 'refused under a single seal. Their probes replay extraction and are the expensive ones, which'
    note 'is why the tier is per stage (CORPUS-CHAIN-CURRENCY.5/.6). Run --total for the per-document'
    note 'verdict at every stage, and CHAIN-CURRENCY for whether the content reproduces.'
  else
    note 'the sampled persisted artifacts carry seals the current build accepts. THIS IS A SAMPLE: one'
    note 'document per distinct seal, so it cannot see a per-document replay-topology divergence —'
    note 'measured once at 23 accepted / 4 refused under a single seal, and the corpus currently'
    note 'carries ONE seal per stage across 27 artifacts, so the sample is 1 in 27. The per-stage'
    note 'TOTAL tier that closes this at semantic and intent is built and self-tested here and ships'
    note 'INERT until CORPUS-CHAIN-CURRENCY.6 activates it. Run --total (CI tier) for the per-document'
    note 'verdict, and CHAIN-CURRENCY for whether the content still reproduces.'
  fi
else
  fail_note 'the persisted corpus is out of seal with the current build. Re-seal it under its owning'
  fail_note 'task leaf with before/after evidence — never as a side effect of an unrelated slice.'
fi
exit "$fail"
