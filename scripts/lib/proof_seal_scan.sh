#!/usr/bin/env bash
# scripts/lib/proof_seal_scan.sh — the ONE proof-seal read predicate, and the ONE chain stage table.
#
# SOURCE-IR-REPRODUCIBILITY.16. Three callers need to answer "what ruleset seal does this persisted
# artifact record, and does the current build still accept it?":
#   - scripts/rebuild_stage_cascade.sh   (the remedy — selects the proof-carrying stratum, records
#                                         the before/after seal of every artifact it rebuilds)
#   - scripts/check_proof_seal_currency.sh (the gate — censuses the seal per stage and probes it)
#   - and, transitively, anything that later needs the same question answered the same way.
#
# `.11` established the lesson this file exists to obey, and `.15` paid for it once already: a
# second copy of a predicate is a predicate that can drift. So the seal read lives here ONCE and
# every caller sources it. `scripts/lib/stage_artifact_identity.sh` is its sibling and owns the
# other shared predicate — CONTENT identity. This file owns SEAL identity; they are deliberately
# separate questions, because the whole premise of a re-seal is that the seal moves and the content
# does not.
#
# ── Why the reader is shaped the way it is ─────────────────────────────────────────────────────
# A stage artifact reaches 87 MB, so it is never parsed whole. `.15` shipped a depth-aware streaming
# scan for that reason, and two of its self-tests exist because the FIRST reader — a line matcher —
# went RED on a compactly serialized ledger (reported `none` for a sealed artifact, which would have
# silently dropped that document out of the in-scope stratum) and on a claim digest nested inside
# the ledger (mistaken for the ledger's own seal). Both properties are preserved here exactly.
#
# What `.16` adds is cost, because a gate pays this on every commit where `.15`'s remedy paid it
# once. Two measured changes, both preserving the verdict byte-for-byte (agreement census over all
# 78 persisted `source_ir.json`: 0 mismatches, exit codes included):
#
#   1. The inner loop no longer walks the artifact one Perl-level character at a time. It matches
#      `[^"{}\[\],:]*+` then either a complete string literal or a single structural byte, so the
#      regex engine skips the runs between structurally significant bytes in C, and a long text
#      value is consumed by one match instead of thousands of iterations.
#      Measured on the 24 proof-carrying `source_ir.json`: 6.26 s -> 1.23 s (5.1x).
#
#   2. `proof_ledger_candidates` is a NECESSARY-CONDITION prefilter: an artifact whose bytes do not
#      contain `"proof_ledger"` at all cannot carry a top-level `proof_ledger` member, so the exact
#      scan never runs on it. This is the leaf's third design constraint — answer "this artifact
#      carries no ledger" without paying a JSON state machine for it. The prefilter still reads
#      every byte (proving absence requires that), but with `index()` rather than a parser:
#      measured 0.24 s for one process over all 78 files, against 2.96 s for the exact scan over the
#      54 proofless ones, which read to EOF looking for a ledger that is not there.
#      It is a PREFILTER, never an authority: a candidate is still decided by the exact scan, so a
#      digest-shaped neighbour, a nested claim, or a `proof_ledger` string appearing anywhere other
#      than as a top-level member is rejected exactly as before.
#
# ── Sourcing contract ─────────────────────────────────────────────────────────────────────────
# This file defines functions only. It sets no options, changes no directory, and writes nothing,
# so a caller's `set` flags and traps survive it. Two caller-scope variables are read AT CALL TIME
# and are optional:
#   GENERATED_ROOT — the corpus root the stage table resolves against (default `generated`).
#   BIN            — the `specforge` binary the read-only probe invokes. Without it the probe
#                    reports "no binary" rather than guessing one.
# Bash-3.2-safe (no associative arrays, no mapfile), matching every entrypoint that sources it.

# ── The seal read ───────────────────────────────────────────────────────────
# proof_ledger_candidates <artifact.json>...
# Prints, one per line and in argument order, those inputs whose bytes contain the `"proof_ledger"`
# key. An input that is unreadable or absent is simply not a candidate. ONE process for the whole
# list — the point is to spend one cheap pass instead of N parser passes.
proof_ledger_candidates() {
  [ "$#" -gt 0 ] || return 0
  perl - "$@" <<'PERL'
use strict;
use warnings;

my $NEEDLE = '"proof_ledger"';
my $KEEP   = length($NEEDLE) - 1;   # the most a needle can straddle a read boundary

FILE: for my $path (@ARGV) {
    open my $handle, '<:raw', $path or next FILE;
    my $carry = '';
    my $chunk;
    while (read($handle, $chunk, 1 << 20)) {
        my $window = $carry . $chunk;   # carry the tail so a needle cannot straddle a read boundary
        if (index($window, $NEEDLE) >= 0) {
            close $handle;
            print "$path\n";
            next FILE;
        }
        $carry = length($window) > $KEEP ? substr($window, -$KEEP) : $window;
    }
    close $handle;
}
PERL
}

# scan_proof_ledger <artifact.json>
# Prints the recorded seal, `none` when the ledger has no `ruleset_sha256`, or nothing at all when
# there is no top-level proof ledger. Exits 0 when a top-level `proof_ledger` object exists, 1
# otherwise. Streaming and depth-aware: it reads the TOP-LEVEL `proof_ledger` object's DIRECT
# `ruleset_sha256` member and cannot be fooled by a digest-shaped field elsewhere in the artifact.
scan_proof_ledger() {
  perl - "$1" <<'PERL'
use strict;
use warnings;

open my $handle, '<:raw', $ARGV[0] or exit 1;
binmode $handle;

my $BLOCK = 1 << 20;
my $buf   = '';
my $eof   = 0;

my $depth        = 0;      # brace/bracket depth; top-level object members sit at depth 1
my $pending_key  = undef;  # the most recent string seen in key position
my $in_ledger    = 0;      # depth at which the proof_ledger object's members live, or 0
my $found_ledger = 0;
my $expect_value = 0;      # the next string literal closes `"ruleset_sha256":`
my $seal;

# One match consumes either a whole string literal (so structural bytes INSIDE a string can never
# be read as structure) or one structural byte, after skipping the run of bytes that are neither.
# Possessive quantifiers keep an unterminated literal at a read boundary from backtracking: the
# match simply fails, pos() is preserved by /c, and the next read appends the rest.
OUTER: while (1) {
    if (!$eof) {
        my $chunk;
        my $bytes = read($handle, $chunk, $BLOCK);
        if (!defined $bytes || $bytes == 0) { $eof = 1 } else { $buf .= $chunk }
    }
    pos($buf) = 0;
    while ($buf =~ /\G[^"{}\[\],:]*+(?:"((?:[^"\\]++|\\.)*+)"|([{}\[\],:]))/gcs) {
        if (defined $1) {
            if ($expect_value) { $seal = $1; last OUTER }
            $pending_key = $1;
            next;
        }
        my $ch = $2;
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
            last OUTER if $in_ledger && $depth == $in_ledger;
            $depth--;
            $pending_key = undef;
            next;
        }
        if ($ch eq ':') {
            $expect_value = ($in_ledger && $depth == $in_ledger
                             && defined $pending_key && $pending_key eq 'ruleset_sha256') ? 1 : 0;
            next;
        }
        $pending_key = undef;   # ','
    }
    my $consumed = pos($buf) || 0;
    substr($buf, 0, $consumed) = '' if $consumed;
    last OUTER if $eof;
}
close $handle;

exit 1 if !$found_ledger;
print defined $seal ? "$seal\n" : "none\n";
exit 0;
PERL
}

# sealed_artifacts <artifact.json>...
# THE composition of the two readers above, and the only place they are composed: prints one
# `<path>\t<seal>` line per input that carries a top-level proof ledger, in argument order. The
# prefilter runs ONCE for the whole list — batching is the whole point, because a per-file prefilter
# would spend a process to save a parse. `<seal>` is `none` for a ledger that records no
# `ruleset_sha256`; an input with no top-level ledger produces no line at all.
sealed_artifacts() {
  [ "$#" -gt 0 ] || return 0
  local candidate seal
  proof_ledger_candidates "$@" | while IFS= read -r candidate; do
    if seal="$(scan_proof_ledger "$candidate" 2>/dev/null)" && [ -n "$seal" ]; then
      printf '%s\t%s\n' "$candidate" "$seal"
    fi
  done
}

# carries_proof_ledger <artifact.json> — stratum membership.
carries_proof_ledger() {
  [ -n "$(sealed_artifacts "$1")" ]
}

# recorded_ruleset <artifact.json> — the seal the artifact records, or `none` when proofless.
recorded_ruleset() {
  local row
  row="$(sealed_artifacts "$1")"
  if [ -n "$row" ]; then
    printf '%s\n' "${row#*$'\t'}"
  else
    printf 'none\n'
  fi
}

# ── The chain stage table ───────────────────────────────────────────────────
# One linear chain, named by the CLI verb that PRODUCES each artifact (`source-ir` is `ingest`'s).
# Every path resolves against the caller's `GENERATED_ROOT`, so an alternate corpus root — a
# self-test's miniature tree, for instance — needs no second table.

# chain_stages — the chain in producer order, one stage id per line.
chain_stages() {
  printf '%s\n' 'source-ir' 'evidence' 'semantic' 'intent' 'isf-adapter'
}

# chain_stage_artifact_path <stage> <key> — the artifact this stage persists for one document.
chain_stage_artifact_path() {
  local root="${GENERATED_ROOT:-generated}"
  case "$1" in
    source-ir)   printf '%s\n' "$root/source_ir/$2/source_ir.json" ;;
    evidence)    printf '%s\n' "$root/evidence_ir/$2/evidence_ir.json" ;;
    semantic)    printf '%s\n' "$root/semantic_ir/$2/semantic_ir.json" ;;
    intent)      printf '%s\n' "$root/intent_ir/$2/intent_ir.json" ;;
    isf-adapter) printf '%s\n' "$root/adapters/isf/$2/adapter.json" ;;
    *) return 1 ;;
  esac
}

# chain_stage_producer <stage> — the stage whose artifact this stage consumes. The head of the
# chain has none, and says so by exiting 1 with no output.
chain_stage_producer() {
  case "$1" in
    source-ir)   return 1 ;;
    evidence)    printf '%s\n' 'source-ir' ;;
    semantic)    printf '%s\n' 'evidence' ;;
    intent)      printf '%s\n' 'semantic' ;;
    isf-adapter) printf '%s\n' 'intent' ;;
    *) return 1 ;;
  esac
}

# chain_stage_readonly_probe <stage> <artifact> <errfile>
# READ-ONLY canonical acceptance for a persisted artifact: run the stage that CONSUMES it with
# `--dry-run`, which enters the same verified loader `specforge validate` would and writes nothing.
#
# THIS IS THE ONE THING A SEAL READER MUST NOT GET WRONG. `specforge validate` is NOT idempotent:
# each call appends a `validation_backannotation` mutation and moves the artifact's digest, and
# every downstream stage retains its upstream ledger as an exact prefix — so probing a persisted
# corpus with `validate` invalidates the chain below it. `.15` did exactly that by accident and had
# to rebuild the corpus. Measured: a `--dry-run` consumer leaves the artifact byte-identical.
#
# Returns 0 when the loader accepts, 1 when it rejects (diagnostic in <errfile>), 2 when the stage
# is TERMINAL and therefore has no read-only canonical probe at all — reported honestly rather than
# papered over — and 3 when the stage is unknown or no binary is in scope.
chain_stage_readonly_probe() {
  local bin="${BIN:-}"
  case "$1" in
    source-ir|evidence|semantic|intent) [ -x "$bin" ] || return 3 ;;
    isf-adapter) return 2 ;;
    *) return 3 ;;
  esac
  case "$1" in
    source-ir) "$bin" evidence "$2" --dry-run >/dev/null 2>"$3" ;;
    evidence)  "$bin" semantic "$2" --dry-run >/dev/null 2>"$3" ;;
    semantic)  "$bin" intent   "$2" --dry-run >/dev/null 2>"$3" ;;
    intent)    "$bin" adapt    "$2" --target isf --dry-run >/dev/null 2>"$3" ;;
  esac
}
