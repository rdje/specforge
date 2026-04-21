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
- latest_commit_hash: `91d05ef`
- latest_commit_brief_message: `test(temporal): lock exact falling shorthand edge`
- note: the latest committed baseline closes the last exact shorthand-token asymmetry inside the trailing `of <clock>` temporal family by benchmark-locking `the third negedge of HCLK` alongside the already-landed exact `posedge`, bounded token, and exact word-edge forms

## Recent commit chain (last 6)
- `91d05ef` test(temporal): lock exact falling shorthand edge
- `628c124` docs(memory): sync ordinal rising-edge baseline
- `6cbf11f` test(temporal): lock ordinal rising edge benchmark
- `cb24e3b` docs(memory): sync ordinal falling-edge baseline
- `b7c0a0e` test(temporal): lock ordinal falling edge symmetry
- `9cf8a28` docs(memory): sync trailing falling edge-word baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 9` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the required continuity refresh commit remains
- `LIVE_ACHIEVEMENT_STATUS.md` stays unchanged for this slice

## Latest landed slice
- outcome:
  - `crates/specforge/src/ir/semantic.rs` now extracts `HCLK` from `the third negedge of HCLK` and adds a direct semantic regression proving `PLOCK must be asserted on the third negedge of HCLK.` lowers to `clock_signal = HCLK`, `edge = falling`, `cycle_window = 3..3`
  - `crates/specforge/src/commands/validate.rs` now widens the existing trailing shorthand-edge validator regression so the exact falling shorthand-token form stays grounded alongside the already-locked bounded token pair
  - `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` now carries `PLOCK must be asserted on the third negedge of HCLK.`, lifting the tracked fixture family from `7` to `8` temporal rules without adding a new fixture family
- tracker effect:
  - unchanged, because this is reliability hardening inside an already-done temporal capability row
- current tracked KG-quality suite size after the feature commit:
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
- current full local CI baseline after the feature commit:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- create the required continuity commit for the refreshed `MEMORY.md`
- truncate `git_message_brief.txt` back to `0` bytes and verify the post-conditions
- do not push after the continuity commit unless the user asks or the branch reaches the `25`-commit threshold again
