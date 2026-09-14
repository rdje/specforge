#!/usr/bin/env bash
# scripts/lib/corpus_replay_binary.sh — the ONE `specforge` build every corpus replay asks.
#
# CORPUS-CHAIN-CURRENCY.8. Three entrypoints ask the persisted corpus a question that only a build
# of THIS COMMIT can answer:
#   - scripts/check_chain_currency.sh       (CI)   is the artifact the CONTENT this build reproduces?
#   - scripts/check_proof_seal_currency.sh  (gate) does this build's canonical loader ACCEPT its seal?
#   - scripts/rebuild_stage_cascade.sh      (remedy) rebuild the artifacts that answer NO.
# `.11`'s lesson, paid for once already: a gate and a remedy that disagree about which artifacts are
# in scope, which seal they carry, or WHICH LOADER ANSWERS FOR THEM leave a debt no compliant work
# can clear. The seal read and the chain table already live in scripts/lib/proof_seal_scan.sh and
# content identity in scripts/lib/stage_artifact_identity.sh for that reason; the binary was the
# third shared predicate, and it was copied out three times. This file is its one home.
#
# ── The profile is RELEASE, and it was measured rather than assumed ────────────────────────────
# It was `debug` until `2026-09-14`, on the reasoning that the question is whether THIS COMMIT's
# build accepts the artifact and a debug build is the cheapest way to obtain one. That reasoning
# holds only while the BUILD dominates; it inverted when the probe count did. Measured on this
# machine at `8126a072` over the 27-document proof-carrying stratum:
#
#   | measurement                                             |    debug |  release |
#   | cold build of `--bin specforge`, empty target tree      |   21.0 s |   36.1 s |
#   | that cold target tree                                   |   1.1 GB |   234 MB |
#   | warm no-op freshness check                              |   0.18 s |   0.06 s |
#   | rebuild after a real edit in `ir/evidence.rs`           |    5.3 s |   32.6 s |
#   | one `intent --dry-run`, AXI 39.7 MB                     |   63.1 s |    6.5 s |
#   | one `intent --dry-run`, ADIv6 38.1 MB                   |   52.4 s |    5.2 s |
#   | one `intent --dry-run`, HBM 15.9 KB                     |   0.00 s |   0.00 s |
#   | PROOF-SEAL sampled: 4 probes + the 5-stage census       |   15.9 s |    7.4 s |
#   | the same, probing semantic and intent TOTALLY (54)      | 12m57.9s |   69.8 s |
#   | `check_proof_seal_currency.sh --total` (PROOF-SEAL-TOTAL)|  18m45s |  1m59.2s |
#   | `check_chain_currency.sh` (CHAIN-CURRENCY)              |   28m00s | 12m38.2s |
#
# The two CI-tier doctrines together go from ~47 minutes to 14m37s, with both verdicts unchanged. The
# gain is not uniform and the shape says why: PROOF-SEAL-TOTAL is 9.4x because every probe it makes is
# a deserialize-and-verify over a 38-84 MB artifact, which is what an unoptimized build is worst at;
# CHAIN-CURRENCY is 2.2x because its evidence leg replays extraction from a normalized markdown bundle,
# where the work is text processing and I/O.
#
# Three things that framing had wrong, each measured:
#   1. A cold release build of this workspace is **36 seconds**, not minutes. The dependency set is
#      six crates; there is no cold cliff to pay for.
#   2. It is not paid on a clean checkout or a hosted CI runner AT ALL. Every one of the three
#      callers skips on an absent `generated/` BEFORE it reaches this function — measured at 0.031 s
#      with no cargo invocation — because the corpus is untracked and those machines have none.
#   3. The release tree is the SMALLER one: 234 MB against debug's 1.1 GB.
#
# It also removes an inconsistency that had nothing to do with cost. `TOOLBOX.md` already tells a reader
# to run the CLI as `./target/release/specforge`, and the corpus's own provenance fact cards record the
# hash of "the owning RELEASE binary" for the documents they describe. The persisted corpus was produced
# by release builds while the doctrines interrogating it used debug ones. They now agree.
#
# ── Why this does not weaken any verdict ──────────────────────────────────────────────────────
# Proof verification is digest comparison and ordered-map lookup (`verify_ledger`, `validate_claim`
# in crates/specforge/src/ir/derivation.rs); the workspace has no `cfg(debug_assertions)` at all and
# the only two `debug_assert!`s on the proof path assert COMPILE-TIME CONSTANTS, so they cannot fire
# on a persisted artifact. Measured rather than argued: both profiles report 27 of 27 accepted at
# semantic and at intent in the same working tree, AXI's 43,419,318-byte `intent --dry-run` is
# byte-identical between them (same SHA-256), so is Wishbone's 10,496,700-byte `semantic --dry-run`,
# and a legacy artifact's refusal is the same string. Debug assertions and overflow panics are
# `cargo test`'s job — it runs in the dev profile at CI and keeps that class covered.
#
# ── Sourcing contract ─────────────────────────────────────────────────────────────────────────
# Functions only. Sets no options, changes no directory, and writes nothing but the build log it is
# handed, so a caller's `set` flags and traps survive it. `ROOT` is read AT CALL TIME (default `.`,
# which is correct because every caller `cd`s to the repository root first). Bash-3.2-safe.

# corpus_replay_profile — the cargo profile every corpus replay builds and probes with.
corpus_replay_profile() {
  printf 'release\n'
}

# corpus_replay_build <build-log-path>
# Builds `specforge` for that profile and prints the resulting binary path. Returns nonzero with the
# compiler output left in <build-log-path> when the tree does not compile — a tree that cannot build
# has no verdict to give, and saying so is fail-closed.
corpus_replay_build() {
  local log="${1:?a build log path is required}"
  local root="${ROOT:-.}"
  cargo build --release --manifest-path "$root/Cargo.toml" --bin specforge >"$log" 2>&1 || return 1
  printf '%s\n' "${CARGO_TARGET_DIR:-$root/target}/release/specforge"
}
