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
- latest_commit_hash: `ec0c155`
- latest_commit_brief_message: `docs(memory): sync clock-edge-of-clock baseline`
- note: the latest committed baseline records the continuity refresh immediately after the clock-edge-of-clock hardening checkpoint that expanded the tracked and direct proof lane to cover the supported signal-leading exact `HCLK clock edge T5` form alongside the existing trailing exact `clock edge T4 of HCLK` and bounded `within 2 clock edges of HCLK` spellings

## Recent commit chain (last 6)
- `ec0c155` docs(memory): sync clock-edge-of-clock baseline
- `37cf085` test(temporal): harden clock-edge-of-clock coverage
- `65943a0` docs(memory): sync named quantified baseline
- `c269c8a` test(temporal): harden named quantified coverage
- `9f46e35` docs(memory): sync named diagram-edge baseline
- `07006f5` test(temporal): harden named diagram-edge coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 10` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/ir/semantic.rs`
- the next-tick direct lexical hardening slice is ready to commit once this continuity refresh is staged

## Current in-flight slice
- objective:
  - harden the direct next-tick proof lane so the extractor, semantic, and validator regressions explicitly prove the supported `next`, `following`, and `subsequent tick` spellings that the tracked benchmark family already expects
- tracker effect:
  - no live-status row change is expected because this is a reliability hardening pass inside the already-done idiomatic next-tick benchmark family
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge extracts_single_cycle_window_from_idiomatic_clock_tick_phrases` passed
  - `cargo test -p specforge derives_next_tick_variants` passed
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_next_tick_variants` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `477` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the feature commit for the next-tick direct lexical hardening slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- refresh `MEMORY.md` so it points at the newly created feature-commit baseline
- create the continuity commit
