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
- latest_commit_hash: `26cc2bc`
- latest_commit_brief_message: `test(kg-bench): add signal-leading clock fixture`
- note: the latest committed baseline raises the signal-leading local-clock family into the tracked KG-quality corpus with a dedicated fixture that locks both word-form and token-form timing through `SemanticIR` and `IntentIR`

## Recent commit chain (last 6)
- `26cc2bc` test(kg-bench): add signal-leading clock fixture
- `c55b858` docs(memory): sync signal-leading lexical baseline
- `97b4310` test(temporal): lock signal-leading lexical symmetry
- `d237f12` docs(memory): sync signal-leading clock baseline
- `5de896c` test(temporal): lock signal-leading clock symmetry
- `235197f` docs(memory): sync word-edge validator baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 19` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the required continuity refresh commit remains
- `LIVE_ACHIEVEMENT_STATUS.md` changed by one benchmark-coverage row in the feature commit

## Latest landed slice
- outcome:
  - added `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/`
  - the new tracked fixture locks:
    - `PREADY must be asserted on HCLK rising edge.`
    - `PWAKEUP must be asserted on HCLK falling edge.`
    - `PSEL must be asserted on HCLK posedge.`
    - `PWRITE must be asserted on HCLK negedge.`
  - both `SemanticIR` and `IntentIR` now benchmark-lock the full signal-leading local-clock family with:
    - `clock_signal = HCLK`
    - preserved edge kind
    - `temporal_rules_missing_clock_grounding = 0`
- tracker effect:
  - changed by one row: `KG benchmark harness now includes signal-leading clock timing coverage` is now `Done`
- current tracked KG-quality suite size after the feature commit:
  - `106` fixtures
- verification passed:
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
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
