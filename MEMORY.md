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
- latest_commit_hash: `9255110`
- latest_commit_brief_message: `feat(validation): plan evidence caution rescans`
- note: the latest committed baseline makes evidence-stage negative-knowledge caution a real replay-planning consumer instead of a planner dead end

## Recent commit chain (last 6)
- `9255110` feat(validation): plan evidence caution rescans
- `29d97ef` docs(memory): sync actor-port replay baseline
- `356c258` feat(validation): route actor-port gaps into rescans
- `5505790` docs(memory): sync source VLM replay baseline
- `d7884cc` feat(validation): route source VLM gaps into rescans
- `f18d02a` docs(memory): sync evidence VLM replay baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 15` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- modified tracked files:
  - `MEMORY.md`
- the feature slice is already committed; only the required continuity refresh commit remains
- do not push after this slice; even after the feature and docs commits the branch will remain below the user's `25`-commit auto-push threshold

## Latest landed slice
- outcome:
  - `project-validation` now consumes `evidence_negative_knowledge_rescan_guidance` instead of dropping it
  - evidence-stage learned caution now replays through the bounded `SourceIR -> EvidenceIR -> validate` lane
  - the planner action text is now explicit that the target ids are evidence-stage conflict/residual ids, not already-canonical surfaces
  - `evidence_input_for_snapshot_stage()` now accepts current `EvidenceIR` artifacts directly so replay inputs can be derived from persisted evidence artifacts
  - the new regression `project_validation_collects_evidence_negative_knowledge_rescan_guidance` locks the replay inputs, action text, and command hints for this narrower caution lane
- verification passed for the landed slice:
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
- write the docs/continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- commit the continuity refresh with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- verify post-conditions:
  - `git ls-files --error-unmatch git_message_brief.txt` fails
  - `wc -c git_message_brief.txt` reports `0`
  - `git status --short --branch` is clean except for the expected branch-ahead marker
