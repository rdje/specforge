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
- latest_commit_hash: `f8626f1`
- latest_commit_brief_message: `docs(memory): sync later-edge fixture baseline`
- note: the latest committed baseline records the continuity refresh after the default-clock later-edge benchmark checkpoint

## Recent commit chain (last 6)
- `f8626f1` docs(memory): sync later-edge fixture baseline
- `67bca73` test(kg-bench): add later-edge timing fixture
- `b640021` docs(memory): sync quantified-edge fixture baseline
- `283148e` test(kg-bench): add quantified-edge timing fixture
- `081299b` docs(memory): sync default next-edge fixture baseline
- `ab08b25` test(kg-bench): add default next-edge timing fixture

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 6` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/test_data/kg_quality/next_tick_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/next_tick_timing_gold/source.md`
- the next-tick lexical hardening slice is ready for the feature commit after this continuity refresh

## Current in-flight slice
- objective:
  - harden the existing `next_tick_timing_gold` benchmark family so it proves `next tick`, `following tick`, and `subsequent tick` together
- tracker effect:
  - no live-status row change is expected because this is reliability hardening inside an already-done benchmark family
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - feature commit is not created yet
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- stage the hardened next-tick fixture plus synced docs and create the feature commit
- refresh `MEMORY.md` again so it records that new feature baseline
- create the continuity commit after the feature commit lands
- do not push because the branch will remain below the `25`-commit threshold
