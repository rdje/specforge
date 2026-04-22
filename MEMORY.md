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
- latest_commit_hash: `07006f5`
- latest_commit_brief_message: `test(temporal): harden named diagram-edge coverage`
- note: the latest committed baseline records the named diagram-edge hardening checkpoint that expanded the tracked and direct proof lane to cover the signal-leading `HCLK edge T4` form alongside the canonical trailing `edge T3 of HCLK` spelling

## Recent commit chain (last 6)
- `07006f5` test(temporal): harden named diagram-edge coverage
- `1629861` docs(memory): sync unit-first diagram baseline
- `53ba2f6` test(temporal): harden unit-first diagram coverage
- `3185061` docs(memory): sync named one-cycle edge baseline
- `02233ae` test(temporal): harden named one-cycle edge coverage
- `1277f37` docs(memory): sync named zero-cycle edge baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 5` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed and only the continuity refresh remains

## Current in-flight slice
- objective:
  - preserve the latest committed baseline after landing the named diagram-edge hardening slice
- tracker effect:
  - no live-status row change is expected in this continuity-only commit
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge derives_named_diagram_edge_variants` passed
  - `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_grounding_for_named_diagram_edge_variants` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `476` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the continuity commit
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed after the slice because it will still be below the `25`-commit auto-push threshold
