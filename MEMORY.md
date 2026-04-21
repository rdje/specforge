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
- `581cc6c` docs(memory): sync quantified temporal parsing baseline
- `912b2f0` feat(temporal): parse quantified tick and edge windows
- `32a9bc6` docs(memory): sync temporal phrase parsing baseline
- `5d30848` feat(temporal): parse later and ordinal timing phrases
- `28b9df3` docs(memory): sync evidence caution planning baseline
- `9255110` feat(validation): plan evidence caution rescans

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 20` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `README.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/semantic.rs`
  - `docs/book/src/domain/temporal-semantics.md`
- the current in-flight slice extends the temporal parser again:
  - `next posedge` / `next negedge` now use the same single-cycle path as `next rising edge`
  - unit-first diagram labels such as `tick T3` and `posedge T4` now use the same direct positional path as `cycle T3`
  - learned-only timing idioms still remain prior-guided fallback
- do not push after this slice; even after the feature and docs commits the branch will remain below the user's `25`-commit auto-push threshold

## Current in-flight slice
- outcome:
  - shorthand edge timing now recognizes `next posedge` and `next negedge`
  - diagram-style unit-first position language now recognizes `tick T3` and `posedge T4`
  - end-to-end `SemanticIR` temporal derivation now covers both a shorthand edge constraint and a diagram-style tick-position constraint
  - prior-guided-only phrases like `one beat later` remain outside the built-in parser
- verification completed so far:
  - `cargo fmt --all`
  - `cargo test -p specforge cycle_window`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after the in-flight changes: `438` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `103` fixtures

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked feature files
- commit the feature slice with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- refresh `MEMORY.md` so it points at the new feature commit baseline
- write the continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- commit the continuity refresh with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- verify post-conditions:
  - `git ls-files --error-unmatch git_message_brief.txt` fails
  - `wc -c git_message_brief.txt` reports `0`
  - `git status --short --branch` is clean except for the expected branch-ahead marker
