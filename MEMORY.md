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
- latest_commit_hash: `2461c63`
- latest_commit_brief_message: `test(temporal): lock word-edge validator family`
- note: the latest committed baseline makes the trailing word-edge validator proof self-contained by carrying exact and bounded rising/falling word-edge timing in the same direct intent-stage regression

## Recent commit chain (last 6)
- `2461c63` test(temporal): lock word-edge validator family
- `7ec2155` docs(memory): sync shorthand validator baseline
- `610bfde` test(temporal): lock exact shorthand validator symmetry
- `2f20610` docs(memory): sync exact falling shorthand baseline
- `91d05ef` test(temporal): lock exact falling shorthand edge
- `628c124` docs(memory): sync ordinal rising-edge baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 13` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the required continuity refresh commit remains
- `LIVE_ACHIEVEMENT_STATUS.md` stays unchanged for this slice

## Latest landed slice
- outcome:
  - `crates/specforge/src/commands/validate.rs` now widens `validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text` with:
    - `PREADY must be asserted on the third rising edge of HCLK.`
    - `PWAKEUP must be asserted on the third falling edge of HCLK.`
  - the same direct validator lane now covers exact rising/falling and bounded rising/falling word-edge timing together
- tracker effect:
  - unchanged, because this is reliability hardening inside an already-done temporal capability row
- current tracked KG-quality suite size after the feature commit:
  - still `105` fixtures, because this slice does not add or change tracked fixtures
- verification passed:
  - `cargo fmt --all`
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after verification:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- create the required continuity commit for the refreshed `MEMORY.md`
- truncate `git_message_brief.txt` back to `0` bytes and verify the post-conditions
- do not push after the continuity commit unless the user asks or the branch reaches the `25`-commit threshold again
