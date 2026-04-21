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
- latest_commit_hash: `28b9df3`
- latest_commit_brief_message: `docs(memory): sync evidence caution planning baseline`
- note: the latest committed baseline captures the last completed replay-planning slice and its continuity refresh

## Recent commit chain (last 6)
- `28b9df3` docs(memory): sync evidence caution planning baseline
- `9255110` feat(validation): plan evidence caution rescans
- `29d97ef` docs(memory): sync actor-port replay baseline
- `356c258` feat(validation): route actor-port gaps into rescans
- `5505790` docs(memory): sync source VLM replay baseline
- `d7884cc` feat(validation): route source VLM gaps into rescans

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 16` of `origin/main`
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
- the current slice is a temporal-model expansion, not a replay-planning change:
  - built-in cycle-window parsing now recognizes explicit later phrases like `two cycles later`
  - built-in cycle-window parsing now recognizes ordinal edge phrases like `on the third rising edge`
  - prior-guided fallback remains necessary for learned-only phrasing like `one beat later`
- do not push after this slice; even after the feature and docs commits the branch will remain below the user's `25`-commit auto-push threshold

## Current in-flight slice
- outcome:
  - `extract_cycle_window_from_text()` now recognizes exact bounded later language like `two cycles later`
  - `extract_cycle_window_from_text()` now recognizes ordinal edge language like `on the third rising edge of HCLK`
  - ordinal numerals/words such as `3rd` / `third` are now accepted where local text also names a cycle-like unit
  - direct diagram-style `on T3` parsing stays conservative; learned-only phrases like `one beat later` still require prior memory fallback
  - new regressions lock both parser recovery and end-to-end `SemanticIR` temporal-rule derivation for the added phrase families
- verification completed so far:
  - `cargo fmt --all`
  - `cargo test -p specforge cycle_window`
- pending verification:
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`

## Next exact steps
- run the remaining verification commands for the temporal slice
- if green, refresh any final live-doc wording that needs the verified local CI counts
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
