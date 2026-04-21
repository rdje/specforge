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
- latest_commit_hash: `2f20610`
- latest_commit_brief_message: `docs(memory): sync exact falling shorthand baseline`
- note: the latest committed baseline captures the previously landed feature slice where the exact shorthand-token family was made benchmark-symmetric through `the third negedge of HCLK`

## Recent commit chain (last 6)
- `2f20610` docs(memory): sync exact falling shorthand baseline
- `91d05ef` test(temporal): lock exact falling shorthand edge
- `628c124` docs(memory): sync ordinal rising-edge baseline
- `6cbf11f` test(temporal): lock ordinal rising edge benchmark
- `cb24e3b` docs(memory): sync ordinal falling-edge baseline
- `b7c0a0e` test(temporal): lock ordinal falling edge symmetry

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 10` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/commands/validate.rs`
- `MEMORY.md`
- the exact shorthand-edge validator-symmetry slice is implemented locally, verified, and awaiting the feature commit
- `LIVE_ACHIEVEMENT_STATUS.md` stays unchanged for this slice

## Current in-flight slice
- goal:
  - close the remaining direct validator asymmetry for the exact trailing shorthand-token pair by adding `the third posedge of HCLK` to the same intent-stage proof lane that already covers exact `negedge` plus the bounded token pair
- implemented locally so far:
  - `crates/specforge/src/commands/validate.rs` now widens `validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text` with `PGRANT must be asserted on the third posedge of HCLK.`
  - the same direct validator lane now covers bounded rising/falling shorthand tokens and exact rising/falling shorthand tokens together
- expected tracker effect:
  - unchanged, because this is reliability hardening inside an already-done temporal capability row
- expected tracked KG-quality suite size after the feature commit:
  - still `105` fixtures, because this slice does not add or change tracked fixtures
- verification passed:
  - `cargo fmt --all`
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text`
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
