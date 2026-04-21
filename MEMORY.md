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
- latest_commit_hash: `628c124`
- latest_commit_brief_message: `docs(memory): sync ordinal rising-edge baseline`
- note: the latest committed baseline captures the previous feature slice where the existing trailing shorthand-edge fixture family was deepened so exact ordinal rising-edge prose is benchmark-locked alongside the already-landed exact falling-edge and bounded edge-word forms

## Recent commit chain (last 6)
- `628c124` docs(memory): sync ordinal rising-edge baseline
- `6cbf11f` test(temporal): lock ordinal rising edge benchmark
- `cb24e3b` docs(memory): sync ordinal falling-edge baseline
- `b7c0a0e` test(temporal): lock ordinal falling edge symmetry
- `9cf8a28` docs(memory): sync trailing falling edge-word baseline
- `b46b341` test(temporal): lock trailing falling edge-word symmetry

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 8` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/source.md`
- `MEMORY.md`
- the exact falling shorthand-token reliability slice is implemented locally, verified, and awaiting the feature commit
- `LIVE_ACHIEVEMENT_STATUS.md` stays unchanged for this slice

## Current in-flight slice
- goal:
  - close the last exact shorthand-token asymmetry inside the trailing `of <clock>` temporal family by benchmark-locking `the third negedge of HCLK`
- implemented locally so far:
  - `crates/specforge/src/ir/semantic.rs` now extracts `HCLK` from `the third negedge of HCLK` and adds a direct semantic regression proving `PLOCK must be asserted on the third negedge of HCLK.` lowers to `clock_signal = HCLK`, `edge = falling`, `cycle_window = 3..3`
  - `crates/specforge/src/commands/validate.rs` now widens the existing trailing shorthand-edge validator regression so the exact falling shorthand-token form stays grounded alongside the already-locked bounded token pair
  - `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` now carries `PLOCK must be asserted on the third negedge of HCLK.`, lifting the tracked fixture family from `7` to `8` temporal rules without adding a new fixture family
- expected tracker effect:
  - unchanged, because this is reliability hardening inside an already-done temporal capability row
- expected tracked KG-quality suite size after the feature commit:
  - still `105` fixtures, because the existing trailing shorthand-edge family is being deepened rather than expanded with a new fixture family
- verification passed:
  - `cargo fmt --all`
  - `cargo test -p specforge extracts_trailing_of_shorthand_edge_clock_phrases`
  - `cargo test -p specforge derives_exact_falling_edge_from_trailing_of_shorthand_edge_text`
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after verification:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked feature files and commit with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify the post-conditions
- refresh `MEMORY.md` so it points at the new feature commit hash/message
- create the required continuity commit for the refreshed `MEMORY.md`
- do not push after the continuity commit unless the user asks or the branch reaches the `25`-commit threshold again
