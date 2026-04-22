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
- latest_commit_hash: `2fd726a`
- latest_commit_brief_message: `docs(memory): sync named diagram-edge fixture baseline`
- note: the latest committed baseline records the last completed continuity refresh after named diagram-edge timing entered the tracked KG-quality corpus

## Recent commit chain (last 6)
- `2fd726a` docs(memory): sync named diagram-edge fixture baseline
- `4bd9a16` test(kg-bench): add named diagram-edge timing fixture
- `b40ccbb` docs(memory): sync named quantified-edge fixture baseline
- `6f467e7` test(kg-bench): add named quantified-edge timing fixture
- `93ef907` docs(memory): sync named one-cycle fixture baseline
- `202c147` test(kg-bench): add named one-cycle timing fixture

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 2` of `origin/main`
- modified tracked files:
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/fixture.json`
- `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/source.md`
- the next feature slice is in progress; no new commit has been created yet in this slice
- `LIVE_ACHIEVEMENT_STATUS.md` should gain one new benchmark-coverage row in this slice

## Current in-flight slice
- objective:
-  - raise the unit-first diagram-position family into the tracked KG-quality corpus without widening the model
- planned code changes:
  - add `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/`
    - lock the already-proved `posedge T4 of HCLK` form through both `SemanticIR` and `IntentIR`
  - update the tracked live docs to reflect the new benchmark fixture and fixture-count growth
- tracker effect:
  - one row should change to `Done`: `KG benchmark harness now includes unit-first diagram-position timing coverage`
- current tracked KG-quality suite size after validation:
  - `111` fixtures
- verification status:
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- write the feature commit message into `git_message_brief.txt`
- stage only the intended tracked files for the feature commit
- create the feature commit, truncate `git_message_brief.txt`, and verify post-conditions
- refresh `MEMORY.md` to the new committed baseline, create the required continuity commit, and do not push because the branch will remain below the `25`-commit threshold
