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
- latest_commit_hash: `d4f53bb`
- latest_commit_brief_message: `fix(adapter): recover transitive topology widths`
- note: the latest committed baseline locks fixed-point explicit top-link endpoint-width propagation across transitive child/top topology paths

## Recent commit chain (last 6)
- `d4f53bb` fix(adapter): recover transitive topology widths
- `3598999` test: harden sibling width and source env isolation
- `b7274bb` fix(adapter): recover child widths from sibling links
- `b462600` fix(adapter): recover top widths from child links
- `e9a91dd` fix(adapter): recover module input widths from actor graph
- `7b266d2` fix(adapter): recover direct input widths from actor graph

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 3` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `corpus_kb/benchmarks/kg-fixtures.md`
  - `corpus_kb/infra/kg-fixtures.md`
  - `corpus_kb/patterns/kg-fixtures.md`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.json`
  - `corpus_kb/prior_candidates/kg-fixture-candidates.md`
  - `corpus_kb/prior_memory/kg-fixtures.md`
  - `corpus_kb/timing/kg-fixtures.md`
  - `corpus_kb/visuals/kg-fixtures.md`

## Current in-flight slice
- objective:
  - restart and execute the README/SESSION_BOOTSTRAP context path, analyze the Rust codebase, and sync stale continuity plus corpus-KB managed projections discovered during the pass
- tracker effect:
  - no product-status row change; this is a continuity/projection refresh, not a new Rust behavior slice
- current tracked KG-quality suite size verified in this slice:
  - `127` fixtures
- verification status:
  - `rustc --version` passed with `rustc 1.95.0`
  - `cargo --version` passed with `cargo 1.95.0`
  - `cargo run --manifest-path Cargo.toml -p specforge -- --help` passed
  - `cargo run --manifest-path Cargo.toml -p specforge -- inspect README.md` passed
  - staged README pipeline passed through `ingest`, `evidence`, `semantic`, `intent`, `validate`, and `adapt --target fsm`
  - README-derived `IntentIR` validation scored `30/100 NEEDS IMPROVEMENT`, which is expected for project documentation rather than a hardware protocol spec
  - README-derived `.fsm` adapter lowering blocked honestly with residuals
  - `bash scripts/run_docs_ci.sh` passed
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
  - `cargo run --manifest-path Cargo.toml -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed managed corpus-KB fixture projections to `127/127`
  - `bash scripts/run_ci.sh` passed with `499` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- current known local CI baseline:
  - `499` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- review the continuity/corpus-KB projection diff
- run final formatting/diff/stale-doc checks after these doc status updates
- commit the README/bootstrap continuity and corpus-KB projection refresh
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 4`, below the `25`-commit push threshold
