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
- latest_commit_hash: `97b4310`
- latest_commit_brief_message: `test(temporal): lock signal-leading lexical symmetry`
- note: the latest committed baseline makes the signal-leading clock family lexically self-contained by proving both word-form and token-form local clock phrasing in the direct semantic and validator lanes

## Recent commit chain (last 6)
- `97b4310` test(temporal): lock signal-leading lexical symmetry
- `d237f12` docs(memory): sync signal-leading clock baseline
- `5de896c` test(temporal): lock signal-leading clock symmetry
- `235197f` docs(memory): sync word-edge validator baseline
- `2461c63` test(temporal): lock word-edge validator family
- `7ec2155` docs(memory): sync shorthand validator baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 17` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the required continuity refresh commit remains
- `LIVE_ACHIEVEMENT_STATUS.md` stays unchanged for this slice

## Latest landed slice
- outcome:
  - `crates/specforge/src/ir/semantic.rs` now widens the existing signal-leading semantic regression so it proves all four direct local-clock variants:
    - `PREADY must be asserted on HCLK rising edge.`
    - `PWAKEUP must be asserted on HCLK falling edge.`
    - `PSEL must be asserted on HCLK posedge.`
    - `PWRITE must be asserted on HCLK negedge.`
  - `crates/specforge/src/commands/validate.rs` now widens the existing signal-leading validator regression so it proves the complementary lexical pair too:
    - `PREADY must be asserted on HCLK posedge.`
    - `PWAKEUP must be asserted on HCLK negedge.`
    - `PSEL must be asserted on HCLK rising edge.`
    - `PWRITE must be asserted on HCLK falling edge.`
- tracker effect:
  - unchanged, because this is reliability hardening inside an already-done temporal capability row
- current tracked KG-quality suite size after the feature commit:
  - still `105` fixtures, because this slice does not add or change tracked fixtures
- verification passed:
  - `cargo fmt --all` passed
  - `cargo test -p specforge derives_explicit_clock_signal_from_signal_leading_edge_text` passed
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_signal_leading_clock_text` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline after verification:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- create the required continuity commit for the refreshed baseline
- truncate `git_message_brief.txt` back to `0` bytes and verify the post-conditions
- do not push after the continuity commit because the branch will remain below the `25`-commit threshold
