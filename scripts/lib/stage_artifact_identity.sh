#!/usr/bin/env bash
# scripts/lib/stage_artifact_identity.sh — the ONE stage-artifact content-identity predicate.
#
# SOURCE-IR-REPRODUCIBILITY.15. Two callers need to answer "do these two stage artifacts carry the
# same content?": `scripts/check_chain_currency.sh` (the CHAIN-CURRENCY doctrine oracle, which asks
# it of a persisted artifact against a dry-run replay) and `scripts/rebuild_stage_cascade.sh` (the
# seal-restoration remedy, which asks it of a pre-write snapshot against the rebuilt artifact).
#
# `.11` established the lesson this file exists to obey: a second copy of a predicate is a predicate
# that can drift, and a remedy whose comparator disagrees with the gate's comparator would prove
# nothing. So the definition lives once (in `cargo xtask check chain-currency` since RUSTDOC-AND-XTASK.14.d)
# and every caller reaches it through this file. Authority is unchanged —
# CHAIN-CURRENCY remains the currency ORACLE; the cascade only uses the same predicate to NAME which
# sections moved, so a delta can be attributed per ADR 0025 decision 1 rather than absorbed.
#
# Sourcing contract: this file defines functions only. It sets no options, changes no directory, and
# writes nothing, so a caller's `set` flags and traps survive it.
# Bash-3.2-safe, matching every entrypoint that sources it.

# ── The comparison core ─────────────────────────────────────────────────────
# Native since RUSTDOC-AND-XTASK.14.d: the predicate is defined once, in `cargo xtask check
# chain-currency` (xtask/src/checks/chain_currency.rs), and these functions call it, so the oracle and the
# remedy still compare with the same code. Each keeps its contract: exit 0 when the same, 1 with the
# difference printed, and Perl's `die` status (the failed call's errno, or 255) with the reason on standard
# error when an input cannot be read. The call runs in a subshell at the repository root, where the `cargo
# xtask` alias resolves, and passes the caller's directory, against which a relative path resolves and is
# named; the caller's directory, options and traps are untouched.

# compare_stage_artifact <persisted.json> <replay.json>
# Exit 0 when both carry the same content identity, with `validation_reports`, `proof_context` and `proof_ledger`
# excluded. On a difference, print the differing top-level sections as
# `name(persisted-cardinality->replay-cardinality)` and exit 1.
compare_stage_artifact() {
  _stage_artifact_identity_native --compare-stage-artifact "$1" "$2"
}

# compare_emitted_isf <replay-adapter.json> <persisted-artifact-root>
# Mirrors `AdapterArtifact::write_to_disk` + `reconcile_emitted_isf_files`: the persisted root holds exactly the
# `.isf` the replayed adapter licenses, with exactly its rendered text. Prints the breach and exits 1 on any
# difference.
compare_emitted_isf() {
  _stage_artifact_identity_native --compare-emitted-isf "$1" "$2"
}

_stage_artifact_identity_native() {
  local caller="$PWD" root
  root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
  (cd "$root" && cargo xtask check chain-currency --caller-dir "$caller" "$1" "$2" "$3")
}
