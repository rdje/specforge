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
- latest_commit_hash: `29d97ef`
- latest_commit_brief_message: `docs(memory): sync actor-port replay baseline`
- note: the latest committed baseline closes the actor-port-gap replay slice and its continuity refresh

## Recent commit chain (last 6)
- `29d97ef` docs(memory): sync actor-port replay baseline
- `356c258` feat(validation): route actor-port gaps into rescans
- `5505790` docs(memory): sync source VLM replay baseline
- `d7884cc` feat(validation): route source VLM gaps into rescans
- `f18d02a` docs(memory): sync evidence VLM replay baseline
- `ffc075f` feat(validation): route evidence VLM gaps into rescans

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 14` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `README.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/project_validation.rs`
  - `docs/book/src/commands/quality-and-learning.md`
  - `docs/book/src/quality/validation.md`
- `MEMORY.md` is being refreshed now as the required pre-commit continuity checkpoint for the current slice
- do not push after this slice; even after the feature and docs commits the branch will remain below the user's `25`-commit auto-push threshold

## Current completed slice awaiting commit
- outcome:
  - `project-validation` now consumes `evidence_negative_knowledge_rescan_guidance` instead of dropping it
  - evidence-stage learned caution now replays through the bounded `SourceIR -> EvidenceIR -> validate` lane
  - the planner action text is now explicit that the target ids are evidence-stage conflict/residual ids, not already-canonical surfaces
  - `evidence_input_for_snapshot_stage()` now accepts current `EvidenceIR` artifacts directly so replay inputs can be derived from persisted evidence artifacts
  - the new regression `project_validation_collects_evidence_negative_knowledge_rescan_guidance` locks the replay inputs, action text, and command hints for this narrower caution lane
- verification passed for the current slice:
  - `cargo fmt --all`
  - `cargo test -p specforge project_validation_collects_negative_knowledge_rescan_guidance`
  - `cargo test -p specforge project_validation_collects_evidence_negative_knowledge_rescan_guidance`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline: `429` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `103` fixtures

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked feature files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `README.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`
  - `crates/specforge/src/commands/project_validation.rs`
  - `docs/book/src/commands/quality-and-learning.md`
  - `docs/book/src/quality/validation.md`
- commit the feature slice with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- refresh `MEMORY.md` again so it reflects the newly created feature commit hash/message
- commit that continuity refresh as the required docs/memory follow-up commit
- verify post-conditions:
  - `git ls-files --error-unmatch git_message_brief.txt` fails
  - `wc -c git_message_brief.txt` reports `0`
  - `git status --short --branch` is clean except for the expected branch-ahead marker
