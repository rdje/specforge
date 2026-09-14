#!/usr/bin/env bash
# scripts/check_chain_currency.sh — the CHAIN-CURRENCY doctrine.
#
# CORPUS-CHAIN-CURRENCY.1 / ADR 0025 decision 2. "The persisted corpus chain reflects the current
# binary" must be a MEASURED, gated property, never an assumption. A repair leaf rebuilds only the
# documents its own change touches, so every other completed document absorbs one code delta at a
# time until something re-checks it. This is that re-check.
#
# Archetype: DETERMINISTIC ORACLE (DOCTRINE_ENFORCEMENT.md §3) — it re-EXECUTES the real pipeline
# rather than inspecting a claim. For every persisted artifact under the corpus root, replaying the
# owning stage with the CURRENT binary at the FIXED persisted input must reproduce that artifact.
#
# Stages and their fixed inputs (every replay is `--dry-run`, so this check writes NO artifact):
#   evidence      source_ir/<k>/source_ir.json         -> evidence_ir/<k>/evidence_ir.json
#   semantic      evidence_ir/<k>/evidence_ir.json     -> semantic_ir/<k>/semantic_ir.json
#   intent        semantic_ir/<k>/semantic_ir.json     -> intent_ir/<k>/intent_ir.json
#   isf-adapter   intent_ir/<k>/intent_ir.json         -> adapters/isf/<k>/adapter.json
#                 ...and the replayed adapter's `isf.source_text` -> adapters/isf/<k>/*.isf
#
# Each stage reads the PERSISTED upstream artifact, not the replayed one. That composes correctly:
# once a stage is proven current, persisted and replayed inputs are the same artifact; and when a
# stage is stale, the check already fails there while every later stage still answers its own
# question exactly.
#
# Content identity excludes `validation_reports`: `specforge validate` back-annotates that section
# AFTER the stage has run, and the product's own `*_ir_fingerprint` helpers
# (crates/specforge/src/commands/validate.rs) clear the same field before hashing. A proved stage's
# `proof_context` and `proof_ledger` also attest that post-build validation mutation, so they are
# excluded from the byte comparison with the report. Proof authority is still executed rather than
# trusted: every downstream replay enters through the upstream stage's canonical verified loader;
# a stale or forged current proof therefore fails the replay before comparison, while a legacy
# proof receives only the explicit UNMEASURABLE disposition below.
#
# Measurability is STATED, never implied (ADR 0025): the evidence replay needs the document's
# normalized markdown bundle, so a document whose bundle was reclaimed is UNMEASURABLE at that stage
# and is reported as a count. Every later stage reads a persisted artifact, so the rest of the chain
# stays measurable for the whole corpus.
#
# RETENTION (CORPUS-CHAIN-CURRENCY.2 / ADR 0025 decision 3) is the second leg. Measurability is not
# a fact about luck: a bundle is retained because a refresh kept it. So the retained set is DECLARED
# in `doctrine/chain_currency/retained_bundles.json` and compared, exactly, with what is on disk:
#   - a declared bundle that is gone  = an unauthorised reclamation (the blindness this doctrine ends)
#   - a bundle on disk that is undeclared = a refresh that did not record what it retained
# Both fail closed. Deliberate reclamation stays possible — it is recorded as a `reclamations` entry
# naming its owning leaf and reason, which is what "deliberate, task-owned" means mechanically.
#
# Skips LOUDLY when the corpus root is absent: a fresh clone and a hosted CI runner have no
# `generated/`, and the doctrine does not govern them. Silence would read as a pass.
#
# Cost: a cargo freshness build plus the full replay. That is CI-tier, not pre-commit-tier
# (DOCTRINE_ENFORCEMENT.md §4.7); `scripts/check_doctrines.sh` runs it only under `--all`.
#
# Knobs: SPECFORGE_CHAIN_CURRENCY_GENERATED_ROOT=<dir> to point at another corpus root (used by
#        --self-test to prove the absent-corpus skip end to end).
#        SPECFORGE_CHAIN_CURRENCY_RETENTION_CONTRACT=<file> to point at another retained-bundle
#        declaration (used by --self-test to exercise the retention core).
# Bash-3.2-safe (no associative arrays, no mapfile) so a stock macOS clone runs it.
set -uo pipefail
export LC_ALL=C

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

GENERATED_ROOT="${SPECFORGE_CHAIN_CURRENCY_GENERATED_ROOT:-generated}"
RETENTION_CONTRACT="${SPECFORGE_CHAIN_CURRENCY_RETENTION_CONTRACT:-doctrine/chain_currency/retained_bundles.json}"

MODE=check
while [ "$#" -gt 0 ]; do
  case "$1" in
    --check)     MODE=check ;;
    --self-test) MODE=self-test ;;
    *) printf 'Usage: %s [--check|--self-test]\n' "$0" >&2; exit 2 ;;
  esac
  shift
done

note()      { printf '[chain-currency] %s\n' "$1"; }
fail_note() { printf '[chain-currency] FAIL: %s\n' "$1" >&2; }

# A stage whose persisted input was deliberately quarantined by a proof-schema migration is not a
# replay failure: the current binary is refusing to grant legacy bytes canonical authority. Match
# only the product's closed compatibility diagnostic. A stale/missing current proof, parse error,
# or any other replay failure remains a doctrine breach.
replay_is_legacy_proof_blocked() {
  grep -Eq 'schema version [0-9]+ is legacy/proofless and inspection-only; rebuild it from verified ' "$1"
}

# current_from_counts <compared> <compared-stale> — content-current artifacts cannot be negative.
# Replay failures are stale but never increment `compared`, so callers must not subtract them here.
current_from_counts() {
  case "$1" in ''|*[!0-9]*) return 1 ;; esac
  case "$2" in ''|*[!0-9]*) return 1 ;; esac
  [ "$2" -le "$1" ] || return 1
  printf '%s\n' "$(( $1 - $2 ))"
}

# adapter_emission_summary <checked-adapter-states> <checked-emitted-files>
# A blocked adapter's proven absence is an output state, not an emitted file.
adapter_emission_summary() {
  case "$1" in ''|*[!0-9]*) return 1 ;; esac
  case "$2" in ''|*[!0-9]*) return 1 ;; esac
  [ "$2" -le "$1" ] || return 1
  printf '%s adapter output state(s) checked (%s emitted .isf file(s), %s blocked/no-file state(s))\n' \
    "$1" "$2" "$(( $1 - $2 ))"
}

# ── The comparison core ─────────────────────────────────────────────────────
# `compare_stage_artifact` and `compare_emitted_isf` are defined ONCE, in
# scripts/lib/stage_artifact_identity.sh, because scripts/rebuild_stage_cascade.sh must answer
# the same content-identity question with the same predicate (SOURCE-IR-REPRODUCIBILITY.15; the
# `.11` lesson about a second copy of a predicate that can drift). This script keeps the
# authority: it is the currency ORACLE, and its --self-test below still exercises the predicate
# it sources, so a change to the shared definition fails here first.
. "$ROOT/scripts/lib/stage_artifact_identity.sh"

# ── Which build answers ─────────────────────────────────────────────────────
# Also defined ONCE, in scripts/lib/corpus_replay_binary.sh, for the same `.11` reason: this oracle,
# PROOF-SEAL-CURRENCY and the rebuild remedy all replay the persisted corpus, and a disagreement
# about WHICH BUILD they interrogate would make their verdicts incomparable. That file owns the
# measured reason the profile is `release` (`CORPUS-CHAIN-CURRENCY.8`).
. "$ROOT/scripts/lib/corpus_replay_binary.sh"

# promoted_markdown_path <source_ir.json> — the normalized bundle entry the evidence replay needs.
# Prints the repository-relative path, or nothing when the bundle was reclaimed.
promoted_markdown_path() {
  perl - "$1" <<'PERL'
use strict;
use warnings;
use JSON::PP;

open my $handle, '<:raw', $ARGV[0] or exit 0;
while (my $line = <$handle>) {
    next if $line !~ /^\s*"promoted_markdown_path"\s*:\s*("(?:[^"\\]|\\.)*")\s*,?\s*$/;
    print JSON::PP->new->utf8->allow_nonref->decode($1), "\n";
    last;
}
close $handle;
PERL
}

# compare_retention <declaration.json> <retained-keys-file> <corpus-keys-file>
# The declaration is schema-CLOSED: an unknown, missing, mistyped, duplicated, or unsorted field is
# itself a breach, so the file cannot decay into free-form prose that no longer means anything. Then
# the declared retained set must equal the set measured on disk, exactly. Prints every breach and
# exits 1.
compare_retention() {
  perl - "$1" "$2" "$3" <<'PERL'
use strict;
use warnings;
use JSON::PP;

my ($declaration_path, $retained_path, $corpus_path) = @ARGV;

sub read_key_set {
    my ($path) = @_;
    my %set;
    local $/ = "\n";
    open my $handle, '<:raw', $path or die "cannot read $path: $!\n";
    while (my $line = <$handle>) {
        chomp $line;
        $set{$line} = 1 if $line ne '';
    }
    close $handle;
    return \%set;
}

sub slurp {
    my ($path) = @_;
    local $/;
    open my $handle, '<:raw', $path or die "cannot read $path: $!\n";
    my $text = <$handle>;
    close $handle;
    return defined $text ? $text : '';
}

my @breaches;

my $text = eval { slurp($declaration_path) };
if (!defined $text) {
    print "the retained-bundle declaration $declaration_path cannot be read\n";
    exit 1;
}

my $declaration = eval { JSON::PP->new->utf8->decode($text) };
if (!$declaration || ref $declaration ne 'HASH') {
    print "the retained-bundle declaration is not a JSON object\n";
    exit 1;
}

# ── Schema (closed) ────────────────────────────────────────────────────────
my @required = qw(schema_version contract_id owner_leaf authority declared_on retained reclamations);
my %expected = map { $_ => 1 } @required;
for my $key (sort keys %$declaration) {
    push @breaches, "unknown declaration field: $key" if !$expected{$key};
}
for my $key (@required) {
    push @breaches, "missing declaration field: $key" if !exists $declaration->{$key};
}

sub nonempty_string {
    my ($value) = @_;
    return defined $value && !ref $value && $value =~ /\S/;
}

push @breaches, 'schema_version must be 1'
  if !exists $declaration->{schema_version} || !defined $declaration->{schema_version}
  || ref $declaration->{schema_version} || $declaration->{schema_version} ne '1';
push @breaches, 'contract_id must be chain-currency-retained-bundles'
  if !exists $declaration->{contract_id} || !defined $declaration->{contract_id}
  || ref $declaration->{contract_id} || $declaration->{contract_id} ne 'chain-currency-retained-bundles';
for my $key (qw(owner_leaf authority)) {
    push @breaches, "$key must be a non-empty string"
      if exists $declaration->{$key} && !nonempty_string($declaration->{$key});
}
push @breaches, 'declared_on must be an ISO date (YYYY-MM-DD)'
  if exists $declaration->{declared_on}
  && (!nonempty_string($declaration->{declared_on}) || $declaration->{declared_on} !~ /\A\d{4}-\d{2}-\d{2}\z/);

my @declared;
if (exists $declaration->{retained}) {
    if (ref $declaration->{retained} ne 'ARRAY') {
        push @breaches, 'retained must be an array of document keys';
    } else {
        @declared = @{ $declaration->{retained} };
        my $previous;
        for my $key (@declared) {
            if (!nonempty_string($key) || $key !~ m{\A[A-Za-z0-9._-]+\z}) {
                push @breaches, 'retained holds a value that is not a document key';
                next;
            }
            push @breaches, "retained is not sorted and unique at: $key"
              if defined $previous && $key le $previous;
            $previous = $key;
        }
    }
}

my %reclaimed;
if (exists $declaration->{reclamations}) {
    if (ref $declaration->{reclamations} ne 'ARRAY') {
        push @breaches, 'reclamations must be an array of records';
    } else {
        my @fields = qw(document_key owning_leaf date reason);
        my %allowed = map { $_ => 1 } @fields;
        my $previous;
        for my $record (@{ $declaration->{reclamations} }) {
            if (ref $record ne 'HASH') {
                push @breaches, 'reclamations holds a value that is not a record';
                next;
            }
            for my $field (sort keys %$record) {
                push @breaches, "unknown reclamation field: $field" if !$allowed{$field};
            }
            for my $field (@fields) {
                push @breaches, "reclamation is missing $field" if !exists $record->{$field};
                push @breaches, "reclamation $field must be a non-empty string"
                  if exists $record->{$field} && !nonempty_string($record->{$field});
            }
            push @breaches, 'reclamation date must be an ISO date (YYYY-MM-DD)'
              if nonempty_string($record->{date}) && $record->{date} !~ /\A\d{4}-\d{2}-\d{2}\z/;
            my $key = $record->{document_key};
            next if !nonempty_string($key);
            push @breaches, "reclamations is not sorted and unique at: $key"
              if defined $previous && $key le $previous;
            $previous = $key;
            $reclaimed{$key} = 1;
        }
    }
}
for my $key (@declared) {
    push @breaches, "$key is declared both retained and reclaimed" if $reclaimed{$key};
}

# ── The declared set must equal the measured set, exactly ──────────────────
my $retained = read_key_set($retained_path);
my $corpus   = read_key_set($corpus_path);

my %is_declared = map { $_ => 1 } @declared;
for my $key (@declared) {
    next if $retained->{$key};
    push @breaches, $corpus->{$key}
      ? "$key is declared retained but its normalized bundle is absent — an unauthorised reclamation"
      : "$key is declared retained but is no longer a corpus document";
}
for my $key (sort keys %$retained) {
    push @breaches, "$key retains a normalized bundle that no leaf declared — record it as retained"
      if !$is_declared{$key};
}

exit 0 if !@breaches;
print "$_\n" for @breaches;
exit 1;
PERL
}

# ── Self-test: prove the comparison core is fail-CLOSED before trusting its PASS ─
run_self_test() {
  local work passed=0 output status
  work="$(mktemp -d)" || { fail_note 'cannot create a repository-local self-test workspace'; return 1; }

  printf '%s' '{"stage":"evidence_ir","timing_constraints":[1,2],"actors":{"a":1},"validation_reports":[],"proof_context":{"mutations":[]},"proof_ledger":{"digest":"base"}}' > "$work/base.json"
  printf '%s' '{"stage":"evidence_ir","timing_constraints":[1,2],"actors":{"a":1},"validation_reports":[{"report_id":"r"}],"proof_context":{"mutations":["validation"]},"proof_ledger":{"digest":"validated"}}' > "$work/reported.json"
  printf '%s' '{"stage":"evidence_ir","timing_constraints":[1],"actors":{"a":1},"validation_reports":[]}' > "$work/shrunk.json"
  printf '%s' '{"stage":"evidence_ir","actors":{"a":1},"validation_reports":[]}' > "$work/missing.json"
  printf '%s' '{"actors":{"a":1},"stage":"evidence_ir","validation_reports":[],"timing_constraints":[1,2]}' > "$work/reordered.json"

  # 1) An artifact matches itself.
  if compare_stage_artifact "$work/base.json" "$work/base.json" >/dev/null; then passed=$((passed + 1))
  else fail_note 'self-test 1: an artifact did not match itself'; fi

  # 2) A validation report and its derived proof metadata are NOT stage-content staleness.
  if compare_stage_artifact "$work/reported.json" "$work/base.json" >/dev/null; then passed=$((passed + 1))
  else fail_note 'self-test 2: a validation_reports-only difference was reported as staleness'; fi

  # 3) Key order is not content (the comparison is semantic, not textual).
  if compare_stage_artifact "$work/reordered.json" "$work/base.json" >/dev/null; then passed=$((passed + 1))
  else fail_note 'self-test 3: re-ordered keys were reported as staleness'; fi

  # 4) A shrunken section is caught AND named with its cardinality delta.
  output="$(compare_stage_artifact "$work/base.json" "$work/shrunk.json")"; status=$?
  if [ "$status" -ne 0 ] && [ "$output" = 'timing_constraints(2->1)' ]; then passed=$((passed + 1))
  else fail_note "self-test 4: a shrunken section was not caught as 'timing_constraints(2->1)' (got '$output')"; fi

  # 5) A section that disappears entirely is caught.
  output="$(compare_stage_artifact "$work/base.json" "$work/missing.json")"; status=$?
  if [ "$status" -ne 0 ] && [ "$output" = 'timing_constraints(2->absent)' ]; then passed=$((passed + 1))
  else fail_note "self-test 5: a vanished section was not caught (got '$output')"; fi

  # 6-9) The emitted-.isf leg: match, missing emission, stale sibling, drifted body.
  mkdir -p "$work/isf"
  printf '%s' '{"artifact_layout":{"emitted_target_path":"generated/adapters/isf/d/wiring_patterns.isf"},"isf":{"is_renderable":true,"source_text":"actor d {}\n"}}' > "$work/renderable.json"
  printf '%s' '{"artifact_layout":{"emitted_target_path":null},"isf":{"is_renderable":false,"source_text":""}}' > "$work/blocked.json"

  printf '%s' 'actor d {}
' > "$work/isf/wiring_patterns.isf"
  if compare_emitted_isf "$work/renderable.json" "$work/isf" >/dev/null; then passed=$((passed + 1))
  else fail_note 'self-test 6: a faithfully emitted .isf was reported as drifted'; fi

  if compare_emitted_isf "$work/blocked.json" "$work/isf" >/dev/null; then
    fail_note 'self-test 7: a stale .isf under a non-renderable adapter was not caught'
  else passed=$((passed + 1)); fi

  printf '%s' 'actor d { drifted }
' > "$work/isf/wiring_patterns.isf"
  if compare_emitted_isf "$work/renderable.json" "$work/isf" >/dev/null; then
    fail_note 'self-test 8: a drifted .isf body was not caught'
  else passed=$((passed + 1)); fi

  rm -f "$work/isf/wiring_patterns.isf"
  if compare_emitted_isf "$work/renderable.json" "$work/isf" >/dev/null; then
    fail_note 'self-test 9: a missing emission was not caught'
  else passed=$((passed + 1)); fi

  # 10) An absent corpus root SKIPS loudly and passes, end to end through this script.
  output="$(SPECFORGE_CHAIN_CURRENCY_GENERATED_ROOT="$work/absent-corpus" "$ROOT/scripts/check_chain_currency.sh" --check 2>&1)"; status=$?
  case "$output" in
    *'SKIP'*) : ;;
    *) status=99 ;;
  esac
  if [ "$status" -eq 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 10: an absent corpus root did not skip loudly (status $status, output '$output')"; fi

  # 11-12) Proof-frontier classification is closed: deliberate legacy quarantine is unmeasurable,
  # while a stale current proof remains a hard failure.
  printf '%s\n' 'error: EvidenceIR schema version 2 is legacy/proofless and inspection-only; rebuild it from verified SourceIR' > "$work/legacy-proof.err"
  printf '%s\n' 'error: EvidenceIR proof verification failed: proof ledger ruleset hash is stale' > "$work/stale-proof.err"
  if replay_is_legacy_proof_blocked "$work/legacy-proof.err"; then passed=$((passed + 1))
  else fail_note 'self-test 11: a deliberately quarantined legacy proof was not classified as unmeasurable'; fi
  if replay_is_legacy_proof_blocked "$work/stale-proof.err"; then
    fail_note 'self-test 12: a stale current proof was incorrectly downgraded to unmeasurable'
  else passed=$((passed + 1)); fi

  # 13-14) Currency arithmetic: replay failures cannot produce a negative current population.
  if [ "$(current_from_counts 0 0)" = '0' ]; then passed=$((passed + 1))
  else fail_note 'self-test 13: an empty replay population did not report zero current'; fi
  if current_from_counts 0 24 >/dev/null 2>&1; then
    fail_note 'self-test 14: stale replay failures were allowed to produce a negative current count'
  else passed=$((passed + 1)); fi

  # 15-16) Adapter emission reporting distinguishes checked states from actual files.
  output="$(adapter_emission_summary 24 0)"; status=$?
  if [ "$status" -eq 0 ] && [ "$output" = '24 adapter output state(s) checked (0 emitted .isf file(s), 24 blocked/no-file state(s))' ]; then
    passed=$((passed + 1))
  else fail_note "self-test 15: blocked adapter states were reported as emitted files (got '$output')"; fi
  if adapter_emission_summary 1 2 >/dev/null 2>&1; then
    fail_note 'self-test 16: more emitted files than checked adapter states was accepted'
  else passed=$((passed + 1)); fi

  # 17-22) The retention leg: exact declared-versus-measured agreement over a schema-closed file.
  printf '%s\n' alpha beta gamma > "$work/corpus.txt"
  printf '%s\n' alpha beta       > "$work/retained.txt"
  printf '%s' '{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["alpha","beta"],"reclamations":[{"document_key":"gamma","owning_leaf":"T.2","date":"2026-08-10","reason":"r"}]}' > "$work/retention.json"

  if compare_retention "$work/retention.json" "$work/retained.txt" "$work/corpus.txt" >/dev/null; then passed=$((passed + 1))
  else fail_note 'self-test 17: an exactly-declared retained set was reported as a breach'; fi

  printf '%s\n' alpha > "$work/retained-shrunk.txt"
  output="$(compare_retention "$work/retention.json" "$work/retained-shrunk.txt" "$work/corpus.txt")"; status=$?
  case "$output" in
    *'beta is declared retained but its normalized bundle is absent'*) : ;;
    *) status=0 ;;
  esac
  if [ "$status" -ne 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 18: a reclaimed declared bundle was not caught (got '$output')"; fi

  printf '%s\n' alpha beta gamma > "$work/retained-extra.txt"
  output="$(compare_retention "$work/retention.json" "$work/retained-extra.txt" "$work/corpus.txt")"; status=$?
  case "$output" in
    *'gamma retains a normalized bundle that no leaf declared'*) : ;;
    *) status=0 ;;
  esac
  if [ "$status" -ne 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 19: an undeclared retained bundle was not caught (got '$output')"; fi

  printf '%s' '{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["alpha","beta"],"reclamations":[],"note":"free-form"}' > "$work/retention-unknown.json"
  output="$(compare_retention "$work/retention-unknown.json" "$work/retained.txt" "$work/corpus.txt")"; status=$?
  case "$output" in
    *'unknown declaration field: note'*) : ;;
    *) status=0 ;;
  esac
  if [ "$status" -ne 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 20: an unknown declaration field was not caught (got '$output')"; fi

  printf '%s' '{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["beta","alpha"],"reclamations":[]}' > "$work/retention-unsorted.json"
  output="$(compare_retention "$work/retention-unsorted.json" "$work/retained.txt" "$work/corpus.txt")"; status=$?
  case "$output" in
    *'retained is not sorted and unique at: alpha'*) : ;;
    *) status=0 ;;
  esac
  if [ "$status" -ne 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 21: an unsorted retained list was not caught (got '$output')"; fi

  printf '%s' '{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["alpha","beta"],"reclamations":[{"document_key":"beta","owning_leaf":"T.2","date":"2026-08-10","reason":"r"}]}' > "$work/retention-contradictory.json"
  output="$(compare_retention "$work/retention-contradictory.json" "$work/retained.txt" "$work/corpus.txt")"; status=$?
  case "$output" in
    *'beta is declared both retained and reclaimed'*) : ;;
    *) status=0 ;;
  esac
  if [ "$status" -ne 0 ]; then passed=$((passed + 1))
  else fail_note "self-test 22: a key declared both retained and reclaimed was not caught (got '$output')"; fi

  rm -rf "$work"
  if [ "$passed" -ne 22 ]; then
    fail_note "self-test $passed/22 passed"
    return 1
  fi
  note 'self-test 22/22 passed.'
  return 0
}

if [ "$MODE" = 'self-test' ]; then
  run_self_test
  exit $?
fi

# ── The gate ────────────────────────────────────────────────────────────────
if [ ! -d "$GENERATED_ROOT/source_ir" ]; then
  note "SKIP: no corpus at $GENERATED_ROOT/source_ir — a fresh clone and hosted CI have none, so"
  note 'SKIP: this doctrine does not govern this tree. Nothing was measured; nothing is claimed.'
  exit 0
fi

# Freshness is the toolchain's decision, never an mtime heuristic here.
BUILD_LOG="$(mktemp)" || { fail_note 'cannot create a repository-local build log'; exit 1; }
trap 'rm -f "$BUILD_LOG"' EXIT
if ! BIN="$(corpus_replay_build "$BUILD_LOG")"; then
  fail_note 'cargo could not build the specforge binary, so no replay is possible:'
  cat "$BUILD_LOG" >&2
  exit 1
fi
if [ ! -x "$BIN" ]; then
  fail_note "cargo reported success but $BIN is not an executable"
  exit 1
fi

WORK="$(mktemp -d)" || { fail_note 'cannot create a repository-local replay workspace'; exit 1; }
trap 'rm -f "$BUILD_LOG"; rm -rf "$WORK"' EXIT

fail=0
documents=0
for source_ir in "$GENERATED_ROOT"/source_ir/*/source_ir.json; do
  [ -f "$source_ir" ] && documents=$((documents + 1))
done
note "corpus root: $GENERATED_ROOT — $documents document(s) with a persisted SourceIR"

for stage in evidence semantic intent isf-adapter; do
  compared=0
  not_persisted=0
  unmeasurable=0
  proof_blocked=0
  orphaned=0
  stale=0
  compared_stale=0
  emission_states_checked=0
  emitted_files_checked=0

  for source_ir in "$GENERATED_ROOT"/source_ir/*/source_ir.json; do
    [ -f "$source_ir" ] || continue
    key="$(basename "$(dirname "$source_ir")")"

    case "$stage" in
      evidence)
        input="$source_ir"
        persisted="$GENERATED_ROOT/evidence_ir/$key/evidence_ir.json" ;;
      semantic)
        input="$GENERATED_ROOT/evidence_ir/$key/evidence_ir.json"
        persisted="$GENERATED_ROOT/semantic_ir/$key/semantic_ir.json" ;;
      intent)
        input="$GENERATED_ROOT/semantic_ir/$key/semantic_ir.json"
        persisted="$GENERATED_ROOT/intent_ir/$key/intent_ir.json" ;;
      isf-adapter)
        input="$GENERATED_ROOT/intent_ir/$key/intent_ir.json"
        persisted="$GENERATED_ROOT/adapters/isf/$key/adapter.json" ;;
    esac

    # Nothing persisted for this stage: the chain simply does not reach here for this document.
    if [ ! -f "$persisted" ]; then
      not_persisted=$((not_persisted + 1))
      continue
    fi

    # A persisted output whose own input is gone is an orphan — its currency is unprovable.
    if [ ! -f "$input" ]; then
      fail_note "$key $stage — persisted artifact has no input to replay from: $input"
      orphaned=$((orphaned + 1))
      fail=1
      continue
    fi

    # Only the evidence stage reads the normalized bundle; a reclaimed bundle is unmeasurable there.
    if [ "$stage" = 'evidence' ]; then
      markdown="$(promoted_markdown_path "$source_ir")"
      if [ -z "$markdown" ] || [ ! -f "$markdown" ]; then
        unmeasurable=$((unmeasurable + 1))
        continue
      fi
    fi

    raw="$WORK/replay.out"
    err="$WORK/replay.err"
    replay_status=0
    case "$stage" in
      evidence)    "$BIN" evidence "$input" --dry-run >"$raw" 2>"$err" || replay_status=$? ;;
      semantic)    "$BIN" semantic "$input" --dry-run >"$raw" 2>"$err" || replay_status=$? ;;
      intent)      "$BIN" intent   "$input" --dry-run >"$raw" 2>"$err" || replay_status=$? ;;
      isf-adapter) "$BIN" adapt    "$input" --target isf --dry-run >"$raw" 2>"$err" || replay_status=$? ;;
    esac
    if [ "$replay_status" -ne 0 ]; then
      if replay_is_legacy_proof_blocked "$err"; then
        proof_blocked=$((proof_blocked + 1))
        continue
      fi
      fail_note "$key $stage — the current binary cannot replay the persisted input: $(tr '\n' ' ' < "$err")"
      stale=$((stale + 1))
      fail=1
      continue
    fi

    replay="$WORK/replay.json"
    awk 'flag { print } /_json:$/ { flag = 1 }' "$raw" > "$replay"
    if [ ! -s "$replay" ]; then
      fail_note "$key $stage — the replay emitted no artifact JSON (expected a '*_json:' section)"
      stale=$((stale + 1))
      fail=1
      continue
    fi

    compared=$((compared + 1))
    if ! differing="$(compare_stage_artifact "$persisted" "$replay")"; then
      fail_note "$key $stage — the persisted artifact is NOT what the current binary produces: $differing"
      stale=$((stale + 1))
      compared_stale=$((compared_stale + 1))
      fail=1
      continue
    fi

    if [ "$stage" = 'isf-adapter' ]; then
      emission_states_checked=$((emission_states_checked + 1))
      if jq -e '.artifact_layout.emitted_target_path != null' "$replay" >/dev/null; then
        emitted_files_checked=$((emitted_files_checked + 1))
      fi
      if ! breach="$(compare_emitted_isf "$replay" "$GENERATED_ROOT/adapters/isf/$key")"; then
        fail_note "$key isf-emit — $breach"
        stale=$((stale + 1))
        compared_stale=$((compared_stale + 1))
        fail=1
      fi
    fi
  done

  # `compared` counts every replay that reached a comparison, so the CURRENT population is what is
  # left after the stale ones are subtracted. Never let a compared-count read as a current-count.
  # Replay failures are stale but never entered `compared`; content/render mismatches did. Subtract
  # only stale artifacts that reached comparison, so the current count cannot become negative.
  if ! current="$(current_from_counts "$compared" "$compared_stale")"; then
    fail_note "$stage — internal currency counts are inconsistent: $compared compared / $compared_stale comparison-stale"
    current=0
    fail=1
  fi
  summary="$stage: $compared replayed, $current current, $stale stale, $not_persisted not persisted"
  if [ "$stage" = 'evidence' ]; then
    summary="$summary, $unmeasurable UNMEASURABLE (normalized bundle reclaimed — needs re-ingest)"
  fi
  if [ "$proof_blocked" -ne 0 ]; then
    summary="$summary, $proof_blocked UNMEASURABLE (legacy/proofless upstream is inspection-only)"
  fi
  if [ "$orphaned" -ne 0 ]; then
    summary="$summary, $orphaned orphaned"
  fi
  if [ "$stage" = 'isf-adapter' ]; then
    if ! emission_summary="$(adapter_emission_summary "$emission_states_checked" "$emitted_files_checked")"; then
      fail_note "$stage — internal emission counts are inconsistent: $emission_states_checked states / $emitted_files_checked files"
      emission_summary='invalid adapter emission counts'
      fail=1
    fi
    summary="$summary; $emission_summary"
  fi
  note "$summary"
done

if [ "$fail" -ne 0 ]; then
  fail_note 'the persisted corpus chain is NOT current with this binary.'
  fail_note 'Rebuild every drifted document under its owning CORPUS-COVERAGE leaf and attribute each'
  fail_note 'delta to this change or to a named earlier leaf (ADR 0025 decision 1). Do not bypass.'
fi

# ── Retention (ADR 0025 decision 3) ─────────────────────────────────────────
# What is measurable above is a consequence of what earlier refreshes chose to keep. Declared and
# measured must agree exactly, so neither a silent reclamation nor an unrecorded retention survives.
if [ ! -f "$RETENTION_CONTRACT" ]; then
  fail_note "the retained-bundle declaration is missing: $RETENTION_CONTRACT"
  fail_note 'It is this doctrine data, not a convention, that makes bundle retention checkable.'
  exit 1
fi

for key_file in "$WORK/retained.keys" "$WORK/corpus.keys"; do : > "$key_file"; done
for source_ir in "$GENERATED_ROOT"/source_ir/*/source_ir.json; do
  [ -f "$source_ir" ] || continue
  key="$(basename "$(dirname "$source_ir")")"
  printf '%s\n' "$key" >> "$WORK/corpus.keys"
  markdown="$(promoted_markdown_path "$source_ir")"
  if [ -n "$markdown" ] && [ -f "$markdown" ]; then
    printf '%s\n' "$key" >> "$WORK/retained.keys"
  fi
done
retained_count="$(grep -c '^' "$WORK/retained.keys")"

if ! breaches="$(compare_retention "$RETENTION_CONTRACT" "$WORK/retained.keys" "$WORK/corpus.keys")"; then
  while IFS= read -r breach; do
    [ -n "$breach" ] && fail_note "retention — $breach"
  done <<EOF
$breaches
EOF
  fail_note "Retention is deliberate and task-owned (ADR 0025 decision 3): a refresh keeps its bundle"
  fail_note "and records it in $RETENTION_CONTRACT; a reclamation is recorded there with its owning"
  fail_note 'leaf and reason. Do not reconcile by deleting evidence or by widening the declaration.'
  fail=1
else
  note "retention: $retained_count normalized bundle(s) on disk — exactly the declared retained set."
fi

if [ "$fail" -ne 0 ]; then
  exit 1
fi

note 'every measurable persisted artifact is exactly what the current binary produces.'
exit 0
