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
- latest_commit_hash: `5d30848`
- latest_commit_brief_message: `feat(temporal): parse later and ordinal timing phrases`
- note: the latest committed baseline expands built-in temporal cycle-window recovery to explicit later and ordinal edge phrasing while preserving prior-guided fallback for learned-only timing idioms

## Recent commit chain (last 6)
- `5d30848` feat(temporal): parse later and ordinal timing phrases
- `28b9df3` docs(memory): sync evidence caution planning baseline
- `9255110` feat(validation): plan evidence caution rescans
- `29d97ef` docs(memory): sync actor-port replay baseline
- `356c258` feat(validation): route actor-port gaps into rescans
- `5505790` docs(memory): sync source VLM replay baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 17` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `MEMORY.md`
- the feature slice is already committed; only the required continuity refresh commit remains
- do not push after this slice; even after the feature and docs commits the branch will remain below the user's `25`-commit auto-push threshold

## Latest landed slice
- outcome:
  - `extract_cycle_window_from_text()` now recognizes exact bounded later language like `two cycles later`
  - `extract_cycle_window_from_text()` now recognizes ordinal edge language like `on the third rising edge of HCLK`
  - ordinal numerals/words such as `3rd` / `third` are now accepted where local text also names a cycle-like unit
  - direct diagram-style `on T3` parsing stays conservative; learned-only phrases like `one beat later` still require prior memory fallback
  - direct parser and end-to-end `SemanticIR` regressions now lock the new timing phrase families
- verification passed for the landed slice:
  - `cargo fmt --all`
  - `cargo test -p specforge cycle_window`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline: `432` Rust tests plus warning-deny rustdoc and the mdBook build
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
