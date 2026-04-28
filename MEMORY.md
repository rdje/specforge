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
- latest_commit_hash: `b175d83`
- latest_commit_brief_message: `docs(memory): sync indexed timing-label baseline`
- note: the latest committed baseline records the post-commit continuity refresh for the indexed VLM timing-annotation hardening slice

## Recent commit chain (last 6)
- `b175d83` docs(memory): sync indexed timing-label baseline
- `428ecb0` fix(semantic): reject indexed VLM timing-value labels
- `2b4658a` docs(memory): sync generic next-cycle baseline
- `0e8468e` test(temporal): harden generic next-cycle coverage
- `cbb4036` docs(memory): sync next-tick baseline
- `9118ab4` test(temporal): harden next-tick direct coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 16` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `MEMORY.md`
- `ROADMAP.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/ir/semantic.rs`
- untracked tracked-fixture family:
  - `crates/specforge/test_data/kg_quality/vlm_timing_cycle_qualified_signal_value_annotation_negative/`

## Current in-flight slice
- objective:
  - keep cycle-qualified VLM signal-value labels like `XREQ HIGH at T1` and `XREQ asserted on T1` from becoming fake timing constraints while preserving only the concrete LOW/HIGH waveform samples
- tracker effect:
  - one live-status row is already updated in the working tree:
    - `KG benchmark harness now locks cycle-qualified signal-value VLM timing-annotation rejection`: `Done`
- current tracked KG-quality suite size after this in-flight slice:
  - `127` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge vlm_timing_diagram_observation_rejects_cycle_qualified_signal_value_labels` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `481` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the feature commit for the cycle-qualified VLM timing-value label hardening slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- refresh `MEMORY.md` again so it records the new feature-commit baseline
- create the continuity commit
- leave the branch unpushed because it remains below the `25`-commit threshold
