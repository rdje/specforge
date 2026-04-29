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
- latest_commit_hash: `e24a371`
- latest_commit_brief_message: `docs(memory): sync cycle-qualified timing-label baseline`
- note: the latest committed baseline records the continuity refresh after landing the cycle-qualified VLM timing-annotation hardening slice

## Recent commit chain (last 6)
- `e24a371` docs(memory): sync cycle-qualified timing-label baseline
- `d29b2ae` fix(semantic): reject cycle-qualified signal-value labels
- `b175d83` docs(memory): sync indexed timing-label baseline
- `428ecb0` fix(semantic): reject indexed VLM timing-value labels
- `2b4658a` docs(memory): sync generic next-cycle baseline
- `0e8468e` test(temporal): harden generic next-cycle coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 18` of `origin/main`
- modified tracked files:
  - `README.md`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/commands/doctor.rs`
  - `docs/book/src/runtime-and-doctor.md`
  - `docs/book/src/reference/troubleshooting.md`

## Current in-flight slice
- objective:
  - harden `specforge doctor --strict` so a healthy local Ollama or LM Studio provider no longer false-negatives only because the default model is cold and needs more than the old 5-second chat probe to load
- tracker effect:
  - no status-state change is expected; the existing Done doctor rows now note cold-load-tolerant local chat probes
- current tracked KG-quality suite size in the latest committed baseline:
  - `127` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge commands::doctor` passed
  - `cargo run --manifest-path Cargo.toml -p specforge -- doctor` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `483` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- commit the doctor cold-load tolerance slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed unless it reaches the `25`-commit threshold
