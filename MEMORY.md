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
- latest_commit_hash: `858f4a8`
- latest_commit_brief_message: `feat(evidence): reject misclassified field tables`
- note: the current session hardens semantic timing lift so low-value VLM labels like `T0` and `Addr 1` do not become timing constraints, and adds a tracked negative fixture for that path

## Recent commit chain (last 5)
- `858f4a8` feat(evidence): reject misclassified field tables
- `cb9d7b2` feat(evidence): reject bogus source-column actors
- `7c1cfec` feat(quality): benchmark amba source-column gold path
- `1d198fe` feat(quality): benchmark visual semantic conflicts
- `94f7826` feat(quality): benchmark vlm timing-note noise rejection

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
- hardened semantic timing lift so low-value VLM labels like `T0`, `Addr 1`, and `Cycle 2` do not create `TimingConstraintRecord` or `TemporalRuleRecord` entries
- added a tracked `vlm_timing_spurious_annotation_negative` fixture that proves evidence-stage timing extraction survives while semantic and intent timing stay empty
- added a direct unit regression proving label-only VLM timing annotations are rejected as canonical timing semantics
- synced `README.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `USER_GUIDE.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` so the richer `R15e` harness shape is captured in the live docs
- validation ran for this task:
  - `cargo test --manifest-path Cargo.toml vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` passed
  - `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` passed
  - `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` passed
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `192/192`

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/test_data/kg_quality/vlm_timing_spurious_annotation_negative/...`
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
1. commit the semantic timing-noise rejection slice with the synced live docs
2. expand the fixture harness toward broader protocol-grade APB/AHB/AXI gold suites and the remaining multimodal arbitration negatives
3. keep the live docs aligned whenever the next truthfulness slice lands

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
