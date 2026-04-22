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
- latest_commit_hash: `4719d62`
- latest_commit_brief_message: `docs(memory): sync named zero-cycle lexical baseline`
- note: the latest committed baseline records the continuity refresh immediately after the named local zero-cycle lexical benchmark hardening checkpoint

## Recent commit chain (last 6)
- `4719d62` docs(memory): sync named zero-cycle lexical baseline
- `9e45c1f` test(kg-bench): harden named zero-cycle lexical coverage
- `7ad8e6d` docs(memory): sync named one-cycle lexical baseline
- `bce577b` test(kg-bench): harden named one-cycle lexical coverage
- `0175a30` docs(memory): sync default explicit-edge baseline
- `1a8ac65` test(kg-bench): harden default explicit-edge lexical coverage

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 18` of `origin/main`
- modified tracked files:
- `Cargo.toml`
- `.github/workflows/ci.yml`
- `docs/book/src/getting-started.md`
- `CHANGES.md`
- `DEVELOPMENT_NOTES.md`
- `MEMORY.md`
- `RUST_CODEBASE_ANALYSIS.md`
- the MSRV alignment slice is in progress and ready for verification

## Current in-flight slice
- objective:
  - align the declared workspace MSRV, hosted CI toolchain, and public getting-started docs to Rust `1.95` after the toolchain bump
- tracker effect:
  - no live-status row change is expected in this maintenance slice
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - metadata and tracked-doc updates are staged in the worktree
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- run `bash scripts/run_ci.sh`
- run `git diff --check`
- commit the feature slice
- refresh `MEMORY.md` again so the latest committed baseline points at that new feature commit
- create the continuity commit
- do not push because the branch will remain below the `25`-commit threshold
