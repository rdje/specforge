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
- latest_commit_hash: `d29b2ae`
- latest_commit_brief_message: `fix(semantic): reject cycle-qualified signal-value labels`
- note: the latest committed baseline records the cycle-qualified VLM timing-annotation hardening slice that keeps labels like `XREQ HIGH at T1` and `XREQ asserted on T1` from becoming fake timing constraints while locking the edge case through both a direct semantic regression and a tracked KG negative fixture

## Recent commit chain (last 6)
- `d29b2ae` fix(semantic): reject cycle-qualified signal-value labels
- `b175d83` docs(memory): sync indexed timing-label baseline
- `428ecb0` fix(semantic): reject indexed VLM timing-value labels
- `2b4658a` docs(memory): sync generic next-cycle baseline
- `0e8468e` test(temporal): harden generic next-cycle coverage
- `cbb4036` docs(memory): sync next-tick baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 17` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed; only the post-commit continuity refresh remains

## Current in-flight slice
- objective:
  - preserve the latest committed baseline after landing the cycle-qualified VLM timing-value label hardening slice
- tracker effect:
  - no live-status row change is expected in this continuity-only commit because the underlying feature commit already recorded the new cycle-qualified VLM timing-annotation negative-fixture row
- current tracked KG-quality suite size in the latest committed baseline:
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
- create the continuity commit
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed because it remains below the `25`-commit threshold
