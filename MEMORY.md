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
- latest_commit_hash: `d7884cc`
- latest_commit_brief_message: `feat(validation): route source VLM gaps into rescans`
- note: the latest landed slice routes SourceIR timing/state diagram asset ids without VLM enrichment into a bounded local `enrich -> validate` replay lane and extends `kg-bench` so tracked fixtures can assert `validation.source`

## Recent commit chain (last 6)
- `d7884cc` feat(validation): route source VLM gaps into rescans
- `f18d02a` docs(memory): sync evidence VLM replay baseline
- `ffc075f` feat(validation): route evidence VLM gaps into rescans
- `fca8c73` docs(memory): sync evidence structural KG replay baseline
- `8a03e21` feat(validation): route evidence structural KG gaps into rescans
- `cb3c08b` docs(memory): sync evidence normative residual replay baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the continuity commit: `ahead 11` of `origin/main`
- push policy remains local-only for now; do not push in this slice
- feature slice is committed; only the required `MEMORY.md` continuity refresh remains before the repo returns to a clean state

## Latest landed slice
- outcome:
  - `validate` now emits `source_vlm_enrichment_missing_surface_rescan_guidance` alongside `source_vlm_enrichment_missing`
  - the new source-stage guidance carries exact timing/state diagram `asset_id` values already known in `SourceIR`
  - `project-validation` routes that guidance into a bounded SourceIR-local `enrich -> validate` replay lane
  - the replay planner now labels that lane explicitly as `source_ir_visual_enrichment_rescan`
  - `kg-bench` now accepts `validation.source` expectations so SourceIR validation surfaces can be locked directly in tracked fixtures
  - the new tracked fixture `source_vlm_enrichment_surface_negative` locks the source-stage replay surface end to end
- tracked files in the feature commit:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `README.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/kg_bench.rs`
  - `crates/specforge/src/commands/project_validation.rs`
  - `crates/specforge/src/commands/validate.rs`
  - `crates/specforge/test_data/kg_quality/source_vlm_enrichment_surface_negative/source.md`
  - `crates/specforge/test_data/kg_quality/source_vlm_enrichment_surface_negative/fixture.json`
  - `docs/book/src/commands/quality-and-learning.md`
  - `docs/book/src/quality/validation.md`
- verification passed for the landed slice:
  - `cargo fmt --all`
  - `cargo test -p specforge validate_source_ir_reports_missing_vlm_enrichment_related_ids`
  - `cargo test -p specforge project_validation_collects_source_vlm_enrichment_missing_rescan_guidance`
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures`
  - `bash scripts/run_docs_ci.sh`
  - `bash scripts/run_ci.sh`
  - `git diff --check`
- current full local CI baseline: `426` Rust tests plus warning-deny rustdoc and the mdBook build
- current tracked KG-quality suite size: `102` fixtures

## Next exact steps
- write the docs-only continuity commit message into `git_message_brief.txt`
- stage only `MEMORY.md`
- commit the continuity refresh with `git commit -F git_message_brief.txt`
- truncate `git_message_brief.txt` back to `0` bytes and verify it remains untracked
- verify post-conditions:
  - `git ls-files --error-unmatch git_message_brief.txt` fails
  - `wc -c git_message_brief.txt` reports `0`
  - `git status --short --branch` is clean except for the expected branch-ahead marker
- do not push after this slice; the branch will still be below the user's `25`-commit auto-push threshold
