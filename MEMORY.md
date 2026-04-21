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
- latest_commit_hash: `912b2f0`
- latest_commit_brief_message: `feat(temporal): parse quantified tick and edge windows`
- note: the latest committed baseline expands built-in cycle-window recovery to counted tick and edge phrasing, including structured timing-constraint units like `ticks`

## Recent commit chain (last 6)
- `912b2f0` feat(temporal): parse quantified tick and edge windows
- `32a9bc6` docs(memory): sync temporal phrase parsing baseline
- `5d30848` feat(temporal): parse later and ordinal timing phrases
- `28b9df3` docs(memory): sync evidence caution planning baseline
- `9255110` feat(validation): plan evidence caution rescans
- `29d97ef` docs(memory): sync actor-port replay baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 19` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `MEMORY.md`
- the feature slice is already committed; only the required continuity refresh commit remains
- do not push after this slice; even after the feature and docs commits the branch will remain below the user's `25`-commit auto-push threshold

## Latest landed slice
- outcome:
  - quantitative prose timing now recognizes `within 2 ticks` and `after 3 falling edges`
  - single-cycle edge phrasing now also recognizes `next falling edge`
  - numeric timing-constraint units like `ticks` now recover bounded `cycle_window` values through the same cycle-like-unit detector
  - prior-guided-only phrases like `one beat later` remain outside the built-in parser
- verification passed for the landed slice:
  - `cargo fmt --all`
  - `cargo test -p specforge cycle_window`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after the in-flight changes: `435` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `103` fixtures

## Next exact steps
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- commit the continuity refresh with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- verify post-conditions:
  - `git ls-files --error-unmatch git_message_brief.txt` fails
  - `wc -c git_message_brief.txt` reports `0`
  - `git status --short --branch` is clean except for the expected branch-ahead marker
