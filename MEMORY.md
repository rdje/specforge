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
- latest_commit_hash: `328c30e`
- latest_commit_brief_message: `feat(temporal): ground trailing shorthand edge clocks`
- note: the latest committed baseline closes the remaining trailing-`of <clock>` shorthand-edge grounding gap, so phrases like `third posedge of HCLK` and `2 negedges of HCLK` now preserve the local `clock_signal` instead of staying half grounded

## Recent commit chain (last 6)
- `328c30e` feat(temporal): ground trailing shorthand edge clocks
- `6477b08` docs(memory): sync unit-first diagram baseline
- `54e0519` feat(temporal): ground unit-first diagram positions
- `c63dd66` feat(temporal): recover named diagram-edge positions
- `d7557f8` feat(temporal): ground plural edge-of-clock phrases
- `8e6bcb4` docs(memory): sync named edge-window baseline
- `43c654a` feat(temporal): recover named bounded clock-edge windows
- `a84a60d` docs(memory): sync generic clock-edge baseline
- `46c3870` feat(temporal): recover generic clock-edge windows
- `78e51b0` docs(memory): sync named next-clock window baseline
- `71b321d` feat(temporal): recover named one-cycle clock windows
- `543c650` docs(memory): sync named cycle grounding baseline
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
- branch state before the next commit: `ahead 17` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `MEMORY.md`
- the feature slice is already committed; only the required continuity refresh commit remains
- do not push after this slice unless the user asks or the branch reaches the threshold again

## Latest landed slice
- outcome:
  - shorthand-edge timing such as `third posedge of HCLK`, `2 negedges of HCLK`, and `2 rising edges of HCLK` now preserves the local `clock_signal`
  - the parser already knew the explicit edge and bounded window for those phrases; this slice closes the remaining clock-grounding gap instead of widening timing semantics
  - the widening stays bounded to explicit trailing `of <known clock>` shorthand-edge phrasing; arbitrary shorthand edge text still does not become a clock guess
- verification passed:
  - `cargo fmt --all`
  - `cargo test -p specforge trailing_of_shorthand_edge`
  - `cargo test -p specforge cycle_window`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after the latest landed slice: `464` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `103` fixtures

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- commit the continuity refresh with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify the post-conditions
