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
- latest_commit_hash: `7c1cfec`
- latest_commit_brief_message: `feat(quality): benchmark amba source-column gold path`
- note: the current session hardens table-driven top-level signal synthesis so misclassified `Bits | Name | Description` field tables do not create fake signals or semantic roles, and adds a tracked negative fixture for that path

## Recent commit chain (last 5)
- `cb9d7b2` feat(evidence): reject bogus source-column actors
- `7c1cfec` feat(quality): benchmark amba source-column gold path
- `1d198fe` feat(quality): benchmark visual semantic conflicts
- `94f7826` feat(quality): benchmark vlm timing-note noise rejection
- `9e39971` feat(quality): benchmark vlm timing-note grounding

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `kg-bench`, `project-validation`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports
- tracked KG-quality fixtures now live under `crates/specforge/test_data/kg_quality/`

## Completed technical work in this session
- hardened table-driven top-level signal synthesis so field-like `Bits | Name | Description` layouts and `... signal fields` captions do not create fake top-level signals or semantic roles when a table is misclassified upstream as `signal_description`
- added a tracked `table_misclassification_field_table_negative` fixture that proves field names like `REQ` / `ACK` stay out of the canonical signal model and semantic-role surface
- added a direct unit regression proving a misclassified field table does not synthesize fake declarations, semantic hints, or actor relations
- synced `README.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `USER_GUIDE.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` so the richer `R15e` harness shape is captured in the live docs
- validation ran for this task:
  - `cargo test --manifest-path Cargo.toml misclassified_field_table_does_not_synthesize_fake_signal_semantics -- --nocapture` passed
  - `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality table_misclassification_field_table_negative` passed
  - `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` passed
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `191/191`

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/test_data/kg_quality/table_misclassification_field_table_negative/...`
  - `README.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
  - `USER_GUIDE.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. expand the fixture harness toward broader protocol-grade APB/AHB/AXI gold suites and the remaining spurious-timing negatives
2. keep finishing the current graph / temporal / arbitration / evaluation workstreams without weakening local truthfulness
3. keep the live docs aligned whenever a new truthfulness slice lands

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, multi-producer ambiguity, modality-aware semantic-role grounding, consensus summaries, semantic candidates, and semantic arbitration summaries (`R15d`)
- broader KG-quality evaluation and benchmark hardening beyond the new seed fixture pack (`R15e`)
- cross-document extractor learning and typed prior memory remain not started (`R15f`)
- Tier 3 relation extraction after the graph/temporal/eval surfaces are ready (`R14`)
- some downstream compatibility and consumer paths still rely on flat `direction_hint` instead of the graph-native surface
- meaning-based role inference now covers tables, prose, alias-grounded prose, visual captions, and VLM timing annotations, and canonical layers now preserve that provenance, but broader downstream use of preserved arbitration state is still early
- semantic-role disagreement is now surfaced explicitly across `EvidenceIR`, `SemanticIR`, and `IntentIR`, but richer multimodal role grounding and broader arbitration still need to grow
- `SourceIR` / ingest are strong enough to remain the foundation, but not strong enough to be assumed universal; future Tier 1 work should stay focused on robustness and honest failure handling
- adapter expansion and adapter validation are now intentionally horizon work
- the workspace still emits compile warnings in `ir/adapters.rs` and `ir/semantic.rs`

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `LIVE_ACHIEVEMENT_STATUS.md`
4. read `ROADMAP.md`
5. read `RUST_CODEBASE_ANALYSIS.md`
6. inspect `git --no-pager status --short`
7. continue with the current semantic-truthfulness roadmap unless the user redirects, while treating the new cross-document learning plane as a documented future workstream
