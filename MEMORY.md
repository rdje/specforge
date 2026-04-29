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
- latest_commit_hash: `b6255cf`
- latest_commit_brief_message: `fix(adapter): surface graph direction conflicts`
- note: the latest committed baseline makes `.fsm` renderability diagnostics distinguish graph-direction conflicts from genuinely missing direction hints

## Recent commit chain (last 6)
- `b6255cf` fix(adapter): surface graph direction conflicts
- `72d47f0` fix(adapter): keep top graph conflicts sticky
- `f2618b4` fix(adapter): keep top graph directions in inventory
- `65fcf4b` fix(validate): split semantic compat direction lag
- `ce6d875` Docs: refresh bootstrap corpus projections
- `d4f53bb` fix(adapter): recover transitive topology widths

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 8` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - preserve `.fsm` width-conflict provenance in signal inventory and report conflicted widths separately from missing widths
- tracker effect:
  - added a `Done` live-status row for adapter signal inventory preserving width-conflict provenance and reporting conflicted widths distinctly
- verification status:
  - `cargo fmt --manifest-path Cargo.toml` passed
  - `cargo test --manifest-path Cargo.toml standalone_dt_blocks_conflicting_control_input_actor_port_widths -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml standalone_dt_keeps_conflicting_actor_port_width_unresolved -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml standalone_explicit_module_blocks_conflicting_control_input_actor_port_widths -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml standalone_sequential_dt_blocks_conflicting_system_contract_signal_width -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_top_port_widths_from_child_links -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_child_topology_widths -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` passed with `57` adapter tests
  - `cargo fmt --manifest-path Cargo.toml -- --check` passed
  - `git diff --check` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed with `501` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
  - `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` passed with `127` fixtures and `0` failures
- current known local CI baseline:
  - `501` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build

## Next exact steps
- rerun final lightweight formatting, docs, and whitespace checks after validation-doc updates
- commit the width-conflict provenance/diagnostics slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed; after this commit it should be `ahead 9`, below the `25`-commit push threshold
