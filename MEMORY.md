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
- latest_commit_hash: `307f967`
- latest_commit_brief_message: `fix(doctor): tolerate cold local model load`
- note: the latest committed baseline records the runtime doctor cold-load tolerance slice for local Ollama and LM Studio probes

## Recent commit chain (last 6)
- `307f967` fix(doctor): tolerate cold local model load
- `e24a371` docs(memory): sync cycle-qualified timing-label baseline
- `d29b2ae` fix(semantic): reject cycle-qualified signal-value labels
- `b175d83` docs(memory): sync indexed timing-label baseline
- `428ecb0` fix(semantic): reject indexed VLM timing-value labels
- `2b4658a` docs(memory): sync generic next-cycle baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 19` of `origin/main`
- modified tracked files:
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `MEMORY.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `crates/specforge/src/ir/adapters.rs`

## Current in-flight slice
- objective:
  - move the explicit `.fsm` top-root boundary direction consumer one step further away from flat `direction_hint` by letting matching top actor-port graph evidence recover width-only top port direction
- tracker effect:
  - add a Done row for top actor-port boundary direction recovery; broader `Actor-relative direction model` remains In Progress
- current tracked KG-quality suite size in the latest committed baseline:
  - `127` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge top_composition_recovers_top_port_direction_from_actor_ports` passed
  - `cargo test -p specforge top_composition_blocks_conflicting_top_actor_port_direction` passed
  - `cargo test -p specforge ir::adapters::tests` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `485` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- commit the top actor-port boundary direction recovery slice
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- leave the branch unpushed unless it reaches the `25`-commit threshold
