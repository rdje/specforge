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
- latest_commit_hash: `dbae9b7`
- latest_commit_brief_message: `feat(semantic): infer handshake roles from signal meaning`
- note: current uncommitted work broadens that role inference into prose and alias-grounded prose, refreshes the role hints during `nlp-enrich`, updates validation source-kind reporting, and syncs the live docs

## Recent commit chain (last 5)
- `dbae9b7` feat(semantic): infer handshake roles from signal meaning
- `d323658` docs(roadmap): log semantic programming doctrine
- `bd6916f` feat(semantic): derive handshake temporal predicates
- `41d6bfd` feat(semantic): recover idiomatic one-cycle phrases
- `a57a564` feat(semantic): surface interface signal conflicts

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports

## Completed technical work in this session
- `crates/specforge/src/ir/evidence.rs` now refreshes `signal_semantic_hints` from:
  - `SignalDescription` table descriptions
  - direct `SourceFact` prose descriptions
  - alias-grounded prose descriptions
- `crates/specforge/src/commands/nlp_enrich.rs` now refreshes `signal_semantic_hints` before persisting updated `EvidenceIR`, so alias learning can feed role grounding immediately
- `crates/specforge/src/ir/semantic.rs` continues to carry those roles as per-signal `semantic_tags` and uses them before literal `VALID` / `READY` fallback for typed handshake derivation
- `crates/specforge/src/commands/validate.rs` now reports:
  - `signal_semantic_hints` for `EvidenceIR`
  - source-kind breakout for `signal_semantic_hints` in `EvidenceIR`
  - `with_semantic_tags` for `SemanticIR`
  - `with_semantic_tags` for `IntentIR`
- synced `README.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` so the new meaning-grounded role surface is visible in continuity docs
- validation ran for this task:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `149/149`

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/commands/nlp_enrich.rs`
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/src/commands/validate.rs`
  - `README.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. commit the prose / alias-grounded semantic-role slice
2. continue `R15c` by broadening semantic-role inference beyond tables and prose into richer multimodal grounding
3. keep adapter work de-prioritized until the graph, temporal, arbitration, and evaluation surfaces are materially stronger

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, and multi-producer ambiguity (`R15d`)
- KG-quality evaluation and benchmark hardening (`R15e`)
- Tier 3 relation extraction after the graph/temporal/eval surfaces are ready (`R14`)
- some downstream compatibility and consumer paths still rely on flat `direction_hint` instead of the graph-native surface
- meaning-based role inference now covers signal-description tables plus conservative prose and alias-grounded prose, but it still needs richer multimodal grounding so protocol semantics do not depend only on text surfaces
- adapter expansion and adapter validation are now intentionally horizon work
- the workspace still emits compile warnings in `ir/adapters.rs` and `ir/semantic.rs`

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `LIVE_ACHIEVEMENT_STATUS.md`
4. read `ROADMAP.md`
5. read `RUST_CODEBASE_ANALYSIS.md`
6. inspect `git --no-pager status --short`
7. continue with the next evidence-loop hardening slice unless the user redirects
