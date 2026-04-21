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
- latest_commit_hash: `eb78219`
- latest_commit_brief_message: `docs(memory): sync symbolic temporal parsing baseline`
- note: the latest committed baseline captures the shorthand-edge and diagram-position temporal parsing slice, including `posedge` / `negedge` cycle-window recovery and unit-first diagram labels like `tick T3` and `posedge T4`

## Recent commit chain (last 6)
- `eb78219` docs(memory): sync symbolic temporal parsing baseline
- `111cda2` feat(temporal): parse symbolic edge and diagram positions
- `581cc6c` docs(memory): sync quantified temporal parsing baseline
- `912b2f0` feat(temporal): parse quantified tick and edge windows
- `32a9bc6` docs(memory): sync temporal phrase parsing baseline
- `5d30848` feat(temporal): parse later and ordinal timing phrases

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 22` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `README.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/validate.rs`
  - `crates/specforge/src/ir/semantic.rs`
  - `docs/book/src/domain/temporal-semantics.md`
- the current feature slice is implemented, documented, and validated locally; the next step is the feature commit, followed by the required continuity refresh commit
- do not push after this slice; even after the feature and docs commits the branch will remain below the user's `25`-commit auto-push threshold

## Latest landed slice
- outcome:
  - explicit shorthand edge timing now preserves the named edge itself in typed temporal rules instead of silently falling back to the default clock edge
  - `next posedge` now grounds `edge = rising`
  - `next negedge` now grounds `edge = falling`
  - signal constraints, conditional rules, and timing constraints now share the same explicit shorthand-edge detector before using default clock-edge fallback
  - validator coverage now proves `next negedge` no longer lands as a semantically mis-grounded temporal rule
- verification passed for the in-flight slice:
  - `cargo fmt --all`
  - `cargo test -p specforge negedge_cycle_window`
  - `cargo test -p specforge temporal_rules_missing_clock_grounding`
  - `cargo test -p specforge cycle_window`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline after the in-flight changes: `440` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `103` fixtures

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked files for the shorthand-edge grounding slice
- commit the feature slice with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- refresh `MEMORY.md` so it points at the newly created feature commit
- commit the continuity refresh with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify the worktree is clean except for the expected branch-ahead marker
