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
- latest_commit_hash: `acacf38`
- latest_commit_brief_message: `feat(temporal): ground named clock cycle phrases`
- note: the latest committed baseline treats named local cycle/tick phrasing like `same ACLK cycle` as fully grounded clock-tick timing with canonical `clock_signal` plus default rising-edge anchoring

## Recent commit chain (last 6)
- `acacf38` feat(temporal): ground named clock cycle phrases
- `237fae3` docs(memory): sync signal-leading clock baseline
- `e78e7ed` feat(temporal): ground signal-leading clock phrases
- `9278b18` docs(memory): sync explicit clock grounding baseline
- `20852a8` feat(temporal): ground explicit clock names from text
- `53ab31b` docs(memory): sync shorthand edge grounding baseline
- `c1670e4` feat(temporal): ground shorthand edge timing
- `eb78219` docs(memory): sync symbolic temporal parsing baseline
- `111cda2` feat(temporal): parse symbolic edge and diagram positions
- `581cc6c` docs(memory): sync quantified temporal parsing baseline
- `912b2f0` feat(temporal): parse quantified tick and edge windows
- `32a9bc6` docs(memory): sync temporal phrase parsing baseline
- `5d30848` feat(temporal): parse later and ordinal timing phrases

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 3` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `MEMORY.md`
- the feature slice is already committed; only the required continuity refresh commit remains
- do not push after this slice unless the user asks or the branch reaches the threshold again

## Latest landed slice
- outcome:
  - named local cycle/tick phrases now become fully grounded temporal rules when the sentence explicitly names the clock signal
  - `same ACLK cycle` now preserves `clock_signal = ACLK`, `edge = rising`, and `cycle_window = 0..0`
  - named cycle/tick text no longer triggers `temporal_rules_missing_clock_grounding` just because the document lacks a separate `Clock ...` declaration
  - the local clock grounding model is now consistent across named edges and named cycle/tick phrasing
- verification passed for the in-flight slice:
  - `cargo fmt --all`
  - `cargo test -p specforge named_cycle_text`
  - `cargo test -p specforge cycle_window`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after the in-flight changes: `445` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `103` fixtures

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- commit the continuity refresh with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- truncate `git_message_brief.txt` back to `0` bytes and verify the worktree is clean except for the expected branch marker
