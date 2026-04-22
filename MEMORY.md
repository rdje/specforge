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
- latest_commit_hash: `ce35d8a`
- latest_commit_brief_message: `test(kg-bench): harden default zero-cycle lexical coverage`
- note: the latest committed baseline records the zero-cycle lexical hardening checkpoint that expanded the tracked default-clock zero-cycle benchmark family to prove `same`, `this`, and `current` spellings end to end

## Recent commit chain (last 6)
- `ce35d8a` test(kg-bench): harden default zero-cycle lexical coverage
- `755055a` docs(memory): sync msrv baseline
- `c739c95` build(msrv): raise rust floor to 1.95
- `4719d62` docs(memory): sync named zero-cycle lexical baseline
- `9e45c1f` test(kg-bench): harden named zero-cycle lexical coverage
- `7ad8e6d` docs(memory): sync named one-cycle lexical baseline

## Current repository state
- active workspace member: `crates/specforge`
- branch: `main`
- branch state before the next commit: `ahead 21` of `origin/main`
- modified tracked files:
- `MEMORY.md`
- the feature slice is committed and only the continuity refresh remains

## Current in-flight slice
- objective:
  - preserve the latest committed baseline after landing the default-clock zero-cycle lexical hardening slice
- tracker effect:
  - no live-status row change is expected in this continuity-only commit
- current tracked KG-quality suite size in the latest committed baseline:
  - `125` fixtures
- verification status:
  - `cargo fmt --all` passed
  - `cargo test -p specforge kg_bench_runs_tracked_fixtures` passed
  - `bash scripts/run_docs_ci.sh` passed
  - `bash scripts/run_ci.sh` passed
  - `git diff --check` passed
- current known local CI baseline:
  - `473` Rust tests plus warning-deny rustdoc and the mdBook build

## Next exact steps
- create the continuity commit
- truncate `git_message_brief.txt` back to `0` bytes and confirm it remains untracked after that commit
- do not push because the branch will remain below the `25`-commit threshold after the two-commit slice
