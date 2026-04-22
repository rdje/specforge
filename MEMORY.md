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
- latest_commit_hash: `8f4378b`
- latest_commit_brief_message: `docs(memory): sync default zero-cycle edge baseline`
- note: the latest committed baseline records the post-default-zero-cycle-edge continuity checkpoint immediately after the default-clock zero-cycle edge hardening slice landed

## Recent commit chain (last 6)
- `8f4378b` docs(memory): sync default zero-cycle edge baseline
- `dee64f5` test(temporal): harden default zero-cycle edge coverage
- `4e2dc71` docs(memory): sync default zero-cycle lexical baseline
- `ce35d8a` test(kg-bench): harden default zero-cycle lexical coverage
- `755055a` docs(memory): sync msrv baseline
- `c739c95` build(msrv): raise rust floor to 1.95

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 24` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/source.md`
- the named local zero-cycle edge hardening slice is verified locally and ready for the feature commit

## Current in-flight slice
- objective:
  - harden the tracked named local zero-cycle family so it proves the full supported named edge lane
- tracker effect:
  - no live-status row change is expected because this deepens an existing benchmark family rather than closing a new roadmap row
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge derives_named_zero_cycle_edge_variants` passed
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_named_cycle_text` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `474` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- stage the verified named zero-cycle edge benchmark, semantic, validator, and live-doc updates for the feature commit
- create the feature commit for named local zero-cycle edge hardening
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked
- refresh `MEMORY.md` so it records the new feature commit as the latest committed baseline
- create the continuity commit
- push after the slice because the branch will cross the `25`-commit threshold
