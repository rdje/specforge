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
- latest_commit_hash: `235197f`
- latest_commit_brief_message: `docs(memory): sync word-edge validator baseline`
- note: the latest committed baseline records the previous slice where the trailing word-edge validator proof became self-contained across exact and bounded rising/falling word-edge timing

## Recent commit chain (last 6)
- `235197f` docs(memory): sync word-edge validator baseline
- `2461c63` test(temporal): lock word-edge validator family
- `7ec2155` docs(memory): sync shorthand validator baseline
- `610bfde` test(temporal): lock exact shorthand validator symmetry
- `2f20610` docs(memory): sync exact falling shorthand baseline
- `91d05ef` test(temporal): lock exact falling shorthand edge

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 14` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/ir/semantic.rs`
- the next feature slice is in progress; no new commit has been created yet in this slice
- `LIVE_ACHIEVEMENT_STATUS.md` stays unchanged for this slice

## Current in-flight slice
- objective:
  - make the signal-leading clock proof surface symmetric on the falling side without widening the model
- planned code changes:
  - `crates/specforge/src/ir/semantic.rs`
    - widen the existing signal-leading semantic regression so it proves both `HCLK rising edge` and `HCLK falling edge`
  - `crates/specforge/src/commands/validate.rs`
    - widen the existing signal-leading validator regression so it proves both `HCLK posedge` and `HCLK negedge`
- tracker effect:
  - unchanged, because this is reliability hardening inside an already-done temporal capability row
- current tracked KG-quality suite size before validation:
  - still `105` fixtures, because this slice does not add or change tracked fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge derives_explicit_clock_signal_from_signal_leading_edge_text` passed
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_signal_leading_clock_text` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline after verification:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked files for the feature commit
- create the feature commit, truncate `git_message_brief.txt`, and verify post-conditions
- refresh `MEMORY.md` to the new committed baseline, create the required continuity commit, and do not push because the branch will remain below the `25`-commit threshold
