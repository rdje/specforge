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
- latest_commit_hash: `2b4658a`
- latest_commit_brief_message: `docs(memory): sync generic next-cycle baseline`
- note: the latest committed baseline records the post-slice continuity refresh after the generic next-cycle direct hardening checkpoint; the current task starts from that committed continuity state and is landing a new VLM timing-evaluation hardening slice

## Recent commit chain (last 6)
- `2b4658a` docs(memory): sync generic next-cycle baseline
- `0e8468e` test(temporal): harden generic next-cycle coverage
- `cbb4036` docs(memory): sync next-tick baseline
- `9118ab4` test(temporal): harden next-tick direct coverage
- `ec0c155` docs(memory): sync clock-edge-of-clock baseline
- `37cf085` test(temporal): harden clock-edge-of-clock coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 14` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `MEMORY.md`
- `ROADMAP.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/src/ir/semantic.rs`
- added tracked fixture family:
  - `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/source.md`
  - `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/fixture.json`
- the feature slice is implemented and verified; the remaining work is the commit workflow plus the post-commit continuity refresh

## Current in-flight slice
- objective:
  - harden VLM timing evaluation so indexed signal-value labels like `XREQ[0] HIGH` and `XREQ[3:0] asserted` stay low-value timing-diagram noise instead of becoming fake `TimingConstraintRecord`s
- tracker effect:
  - one live-status row is expected to change because the tracked KG benchmark matrix will now explicitly lock indexed signal-value VLM timing-annotation rejection as its own hardened negative edge case
- current tracked KG-quality suite size in the latest committed baseline:
  - `126` fixtures after adding the new tracked negative fixture family
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge vlm_timing_diagram_observation_rejects_indexed_signal_value_labels` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `480` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the feature commit for the indexed VLM timing-annotation hardening slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- refresh `MEMORY.md` again so it records the newly-created feature commit hash/message
- create the continuity commit for that memory refresh
- leave the branch unpushed because it remains below the `25`-commit threshold
