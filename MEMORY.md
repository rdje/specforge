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
- latest_commit_hash: `5505790`
- latest_commit_brief_message: `docs(memory): sync source VLM replay baseline`
- note: the latest landed slice routes SourceIR timing/state diagram asset ids without VLM enrichment into a bounded local `enrich -> validate` replay lane

## Recent commit chain (last 6)
- `5505790` docs(memory): sync source VLM replay baseline
- `d7884cc` feat(validation): route source VLM gaps into rescans
- `f18d02a` docs(memory): sync evidence VLM replay baseline
- `ffc075f` feat(validation): route evidence VLM gaps into rescans
- `fca8c73` docs(memory): sync evidence structural KG replay baseline
- `8a03e21` feat(validation): route evidence structural KG gaps into rescans

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 12` of `origin/main`
- push policy remains local-only for this slice; do not push
- working tree currently contains one completed-but-uncommitted actor-port replay-guidance slice

## Current in-flight slice (pre-commit)
- objective: make relation-only actor graph remnants a first-class replay target instead of leaving them as passive canonical graph debt
- implementation now in flight:
  - `validate` emits `semantic_actor_port_gap_surface_rescan_guidance` / `intent_actor_port_gap_surface_rescan_guidance`
  - the new guidance carries exact `asr_*` actor-signal relation ids already reported by `semantic_actor_ports_missing` / `intent_actor_ports_missing`
  - `project-validation` routes that guidance through the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - `kg-bench` now accepts `semantic_ir_patch.clear_actor_ports`
  - the new tracked fixture `actor_port_gap_surface_negative` locks the relation-present, actor-port-missing canonical shape end to end
- touched tracked files for the feature slice:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `README.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`
  - `crates/specforge/src/commands/kg_bench.rs`
  - `crates/specforge/src/commands/project_validation.rs`
  - `crates/specforge/src/commands/validate.rs`
  - `crates/specforge/test_data/kg_quality/actor_port_gap_surface_negative/source.md`
  - `crates/specforge/test_data/kg_quality/actor_port_gap_surface_negative/fixture.json`
  - `docs/book/src/commands/quality-and-learning.md`
  - `docs/book/src/quality/validation.md`

## Validation status for the in-flight slice
- passed:
  - `cargo fmt --all`
  - `cargo test -p specforge validate_semantic_and_intent_ir_report_actor_port_gap_related_ids`
  - `cargo test -p specforge project_validation_collects_actor_port_gap_rescan_guidance`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline: `428` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `103` fixtures

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked feature-slice files
- commit the feature slice with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- update `MEMORY.md` so it records the newly created feature commit hash/message as the latest committed baseline
- commit that continuity refresh as the required docs-only follow-up
- verify post-conditions:
  - `git ls-files --error-unmatch git_message_brief.txt` fails
  - `wc -c git_message_brief.txt` reports `0`
  - `git status --short --branch` is clean except for the expected branch-ahead marker
- do not push after this slice; the branch will still be below the user's `25`-commit auto-push threshold
