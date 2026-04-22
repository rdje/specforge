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
- latest_commit_hash: `9f46e35`
- latest_commit_brief_message: `docs(memory): sync named diagram-edge baseline`
- note: the latest committed baseline records the continuity refresh immediately after the named diagram-edge hardening checkpoint that expanded the tracked and direct proof lane to cover the signal-leading `HCLK edge T4` form alongside the canonical trailing `edge T3 of HCLK` spelling

## Recent commit chain (last 6)
- `9f46e35` docs(memory): sync named diagram-edge baseline
- `07006f5` test(temporal): harden named diagram-edge coverage
- `1629861` docs(memory): sync unit-first diagram baseline
- `53ba2f6` test(temporal): harden unit-first diagram coverage
- `3185061` docs(memory): sync named one-cycle edge baseline
- `02233ae` test(temporal): harden named one-cycle edge coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 6` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/source.md`
- the named quantified signal-leading ordinal hardening slice is ready to commit once this continuity refresh is staged

## Current in-flight slice
- objective:
  - harden the named quantified proof lane so the tracked benchmark family and direct regressions also prove the supported signal-leading ordinal `third HCLK edge` variant
- tracker effect:
  - no live-status row change is expected because this is a reliability hardening pass inside the already-done named quantified benchmark family
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge derives_named_quantified_edge_variants` passed
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_named_quantified_edge_variants` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `476` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the feature commit for the named quantified signal-leading ordinal hardening slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- refresh `MEMORY.md` so it points at the newly created feature-commit baseline
- create the continuity commit
