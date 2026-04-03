# MEMORY
## Purpose
- maintain a compact but actionable continuity record for interrupted sessions
- preserve enough context to resume work quickly after session loss, tool restart, machine crash, or model handoff

## Current project identity
- repository name: `specforge`
- CLI/binary name: `specforge`
- implementation language: Rust
- canonical deliverable: `IntentIR`
- stage model: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## User-required continuity rules
- live documents are critical infrastructure, not optional afterthoughts
- update `MEMORY.md` after completed tasks and at meaningful checkpoints during long-running work
- `MEMORY.md` must record the latest already-committed Git hash/message known at the time of update
- `CHANGES.md` must capture the detailed changes about to be committed
- `DEVELOPMENT_NOTES.md` must capture engineering decisions, rationale, and validation context
- `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, and `RUST_CODEBASE_ANALYSIS.md` must be reviewed before each commit when the task changes them
- `README.md` remains the single entry point and its last line must be `Read SESSION_BOOTSTRAP.md and start from there.`
- `subs/fsmgen` is contextual-only and must stay read-only from `specforge`
- if `fsmgen` behavior looks wrong, file a local tracked bug report using `FSMGEN-BUG-####` instead of patching the submodule

## Latest committed baseline
- latest_commit_hash: `1a5f7bc`
- latest_commit_brief_message: `fix(ingest): broader timing_diagram classification for figure captions`
- note: this is the pre-commit baseline; after the next commit, refresh this section again so it points at the newly created commit

## Recent commit chain (last 5)
- `1a5f7bc` fix(ingest): broader timing_diagram classification for figure captions
- `a264144` fix(ingest): caption-gated signal_description classification
- `dc3b34d` fix(evidence): row-scan contract detection, immune to Docling column-ordering bugs
- `3e6ab6c` fix(evidence): header-clue + positional column detection; AMBA 5 direction mapping
- `c8915fa` feat(ir): WidthHint enum — parametric widths + table width map for KG synthesis

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, and `nlp-enrich`
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior

## Completed technical work in this session
- followed the repo bootstrap contract from `README.md` into `SESSION_BOOTSTRAP.md`, reviewed the live docs, and inspected the current Rust codebase/CLI surface against the documented architecture
- `crates/specforge/src/commands/nlp_enrich.rs` now rejects alias subjects beginning with markdown/table markers `-`, `|`, or `#`, closing the remaining Form 2 alias-garbage cleanup in `extract_alias_phrase()`
- regression test added:
  - `extract_alias_phrase_rejects_markdown_marker_prefixes`
- validation completed:
  - `cargo test --manifest-path Cargo.toml` → 99 passed
  - `cargo run -p specforge -- --help` → passed
  - `cargo run -p specforge -- inspect README.md` → passed
  - `cargo run -p specforge -- ingest README.md --dry-run` → passed
  - `cargo run -p specforge -- ingest README.md` → passed
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run` → passed
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json` → passed
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run` → passed
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json` → passed
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run` → passed
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json` → passed
  - `cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run` → passed
- generated README artifacts now exist under:
  - `generated/source_ir/readme/`
  - `generated/evidence_ir/readme/`
  - `generated/semantic_ir/readme/`
  - `generated/intent_ir/readme/`

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/commands/nlp_enrich.rs`
  - `CHANGES.md`
  - `DEVELOPMENT_NOTES.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `ROADMAP.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `MEMORY.md`
- untracked generated artifacts currently include:
  - `generated/source_ir/readme/`
  - `generated/evidence_ir/readme/`
  - `generated/semantic_ir/readme/`
  - `generated/intent_ir/readme/`

## Exact next steps
1. decide whether the untracked README-derived `generated/.../readme/` artifacts should be kept for continuity or removed before any commit
2. if committing this slice, stage `crates/specforge/src/commands/nlp_enrich.rs` and the refreshed live docs, and only stage the README-derived generated artifacts if intentional versioning is desired
3. begin the next roadmap slice at `R7` (validation back-annotation), keeping `R15` actor-relative direction modeling as the next larger architectural step

## Remaining engineering gaps after this commit
- validation back-annotation and artifact-linked reports (`R7`)
- actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- downstream interfaces still flatten actor-aware relations too early
- the workspace still emits compile warnings in `commands/enrich.rs`, `ir/adapters.rs`, and `ir/semantic.rs`

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `LIVE_ACHIEVEMENT_STATUS.md`
4. read `ROADMAP.md`
5. read `RUST_CODEBASE_ANALYSIS.md`
6. inspect `git --no-pager status --short`
7. continue with `R7` unless the user redirects
