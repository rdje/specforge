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
- latest_commit_hash: `55df3a3`
- latest_commit_brief_message: `docs(memory): sync shorthand next-edge fixture baseline`
- note: the latest committed baseline records the latest continuity checkpoint after the shorthand next-edge benchmark slice

## Recent commit chain (last 6)
- `55df3a3` docs(memory): sync shorthand next-edge fixture baseline
- `d60d1e3` test(kg-bench): add shorthand next-edge timing fixture
- `2804882` docs(memory): sync tick-unit fixture baseline
- `08f3858` test(kg-bench): add tick-unit timing fixture
- `7efb8ce` docs(memory): sync plural edge-of-clock fixture baseline
- `9dbcaef` test(kg-bench): add plural edge-of-clock timing fixture

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 10` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/test_data/kg_quality/later_phrase_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/later_phrase_timing_gold/source.md`
- the new slice is now in flight locally and has not been validated or committed yet

## Current in-flight slice
- objective:
  - raise the already-landed explicit later-phrase temporal family into the tracked KG-quality benchmark corpus
- tracker effect:
  - move `KG benchmark harness now includes explicit later-phrase timing coverage` to `Done`
- current tracked KG-quality suite size after validation:
  - `115` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the feature commit for the later-phrase benchmark fixture slice
- refresh `MEMORY.md` again for the required continuity commit
- do not push because the branch remains below the `25`-commit threshold
