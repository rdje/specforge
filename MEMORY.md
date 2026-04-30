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
- latest_commit_hash: `4f2bf492208a0bf83dfaa0d04a0500d8c8d2426f`
- latest_commit_brief_message: `test(adapter): lock top link root confidence`
- note: the latest committed baseline locks top root-kind decision confidence when high-confidence evidence comes from explicit top-link topology

## Recent commit chain (last 6)
- `4f2bf49` test(adapter): lock top link root confidence
- `9795e3e` test(adapter): lock top child root confidence
- `4053491` test(adapter): lock top child provenance
- `db40fdf` test(adapter): lock child renderable system provenance
- `240b5cc` test(adapter): lock renderable system provenance
- `5c715d6` test(adapter): lock standalone system provenance

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 11` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - lock renderable top-root link support IDs and automation confidence for explicit topology links
- tracker effect:
  - added a renderable top-root link provenance `Done` row for `?toplink:wiring` emission
- verification status:
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_root_kind_confidence_follows_top_link_evidence -- --exact --nocapture` passed
  - `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` passed with `69` adapter tests
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `git diff --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `513` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `513` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- rerun final lightweight formatting, docs, and whitespace checks after validation-doc updates
- commit the renderable top-link provenance regression slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 12`, below the `25`-commit push threshold
