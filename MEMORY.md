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
- latest_commit_hash: `b46b341`
- latest_commit_brief_message: `test(temporal): lock trailing falling edge-word symmetry`
- note: the latest committed baseline deepens the existing trailing shorthand-edge fixture family so bounded word-based falling-edge prose is benchmark-locked alongside the already-landed rising-side twin and token shorthand forms

## Recent commit chain (last 6)
- `b46b341` test(temporal): lock trailing falling edge-word symmetry
- `3859295` docs(memory): sync trailing edge-word baseline
- `f782b64` test(temporal): lock trailing edge-word timing symmetry
- `9db08cf` docs(memory): sync shorthand-edge symmetry baseline
- `d327ca8` test(temporal): lock plural shorthand-edge symmetry
- `58d0333` docs(memory): sync plural shorthand-edge baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 3` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the required continuity refresh commit remains
- `LIVE_ACHIEVEMENT_STATUS.md` stays unchanged for this slice

## Latest landed slice
- outcome:
  - the existing tracked fixture `trailing_shorthand_edge_timing_gold` now also proves bounded word-based falling-edge timing through `PWRITE must be asserted within 2 falling edges of HCLK.`
  - the explicit local-clock extraction regression for trailing `of <clock>` edge phrasing now covers `within 2 falling edges of HCLK`
  - focused semantic coverage now directly proves the falling-side word-edge phrase preserves `clock_signal = HCLK`, `edge = falling`, and `cycle_window.max_cycles = 2`
  - the existing validator regression for trailing word-edge timing now proves both rising and falling word-edge rules stay fully grounded together
  - the tracked KG fixture count stays at `105` because this is another hardening pass inside the same temporal fixture family, not a new fixture family
- verification passed:
  - `cargo fmt --all`
  - `cargo test -p specforge derives_falling_edge_from_trailing_of_word_edge_text`
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text`
  - `cargo test -p specforge trailing_of_word_edge`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after the feature commit:
  - `472` Rust tests plus warning-deny rustdoc and the mdBook build
- tracker effect:
  - unchanged, because this is reliability hardening inside an existing temporal capability row
- current tracked KG-quality suite size after the feature commit:
  - still `105` fixtures, because the existing trailing shorthand-edge family is being deepened rather than expanded with a new fixture family

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- create the continuity commit with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify the post-conditions
- do not push after the continuity commit unless the user asks or the branch reaches the `25`-commit threshold again
