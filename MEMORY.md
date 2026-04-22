# MEMORY
## Purpose
- maintain a compact, actionable continuity record for interrupted sessions
- preserve enough context to resume cleanly after session loss, tool restart, machine crash, or model handoff

## Current project identity
- repository name: `specforge`
- CLI/binary name: `specforge`
- implementation language: Rust
- canonical deliverable: `IntentIR`
- stage model: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## User-required continuity rules
- follow `COMMIT.md` strictly after every completed slice
- sync all relevant tracked live docs before commit, not as an afterthought
- `git_message_brief.txt` must stay untracked and be truncated to `0` bytes after each commit
- every completion message must report the commit id, exact commit message, full tracked-file list, current live-status snapshot, and whether that snapshot changed
- do not push unless the user asks or the branch reaches `25` local commits since the last push
- keep repo-internal paths in tracked markdown relative, never checkout-specific absolute paths

## Latest committed baseline
- latest_commit_hash: `4e2dc71`
- latest_commit_brief_message: `docs(memory): sync default zero-cycle lexical baseline`
- note: the latest committed baseline records the post-zero-cycle-lexical continuity checkpoint immediately after the default-clock zero-cycle lexical hardening slice landed

## Recent commit chain (last 6)
- `4e2dc71` docs(memory): sync default zero-cycle lexical baseline
- `ce35d8a` test(kg-bench): harden default zero-cycle lexical coverage
- `755055a` docs(memory): sync msrv baseline
- `c739c95` build(msrv): raise rust floor to 1.95
- `4719d62` docs(memory): sync named zero-cycle lexical baseline
- `9e45c1f` test(kg-bench): harden named zero-cycle lexical coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 22` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/source.md`
- the default-clock zero-cycle edge hardening slice is verified locally and ready for the feature commit

## Current in-flight slice
- objective:
  - harden the tracked default-clock zero-cycle family so it proves the full supported zero-cycle edge lane
- tracker effect:
  - no live-status row change is expected because this deepens an existing benchmark family rather than closing a new roadmap row
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge extracts_zero_cycle_window_from_same_cycle_phrases` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- stage the verified zero-cycle edge benchmark, parser-test, and live-doc updates for the feature commit
- create the feature commit for default-clock zero-cycle edge hardening
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked
- refresh `MEMORY.md` so it records the new feature commit as the latest committed baseline
- create the continuity commit
- do not push because the branch will remain below the `25`-commit threshold after the two-commit slice
