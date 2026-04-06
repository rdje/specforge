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
- latest_commit_hash: `53da080`
- latest_commit_brief_message: `feat(learning): learn actor taxonomy priors`
- note: the current session lands the first bounded `R15f` consumer by teaching `EvidenceIR` / `converge` to consult the local prior store for actor-taxonomy guidance without weakening local grounding

## Recent commit chain (last 5)
- `53da080` feat(learning): learn actor taxonomy priors
- `8a9c3bd` ci(repo): unify local and hosted CI
- `eecf375` ci(repo): add GitHub Actions Rust checks
- `40a14ed` feat(learning): add local prior memory plane
- `48034e9` chore(git): untrack swap files

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `kg-bench`, `project-validation`, `learn-priors`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- GitHub Actions now mirrors the baseline Rust quality gate on every `push` / `pull_request` via `.github/workflows/ci.yml`
- `scripts/run_ci.sh` is now the canonical Rust CI entrypoint and is reused by GitHub Actions, so the same hosted path can be exercised locally before push
- the local `CorpusMemory` prior store now includes actor-taxonomy priors in addition to semantic and temporal phrase priors
- `specforge evidence` and `specforge converge` now consult `--prior-memory generated/prior_memory/corpus_memory.json` by default, and the first bounded consumer uses actor-taxonomy priors only to interpret explicit local actor terms in section headings and `Source` / `Destination` columns
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports
- tracked KG-quality fixtures now live under `crates/specforge/test_data/kg_quality/`

## Completed technical work in this session
- extended `crates/specforge/src/ir/prior_memory.rs` with reusable actor-taxonomy lookup helpers, normalized term matching, and protocol-family inference so the learning plane can be queried safely during extraction
- extended `crates/specforge/src/ir/evidence.rs` with `build_with_prior_memory(...)` and the first bounded prior-consumption path:
  - section-heading direction inference can use actor-taxonomy priors
  - `Source` / `Destination` column direction inference can use actor-taxonomy priors
  - prior guidance still requires explicit local actor terms in the current document
- extended `crates/specforge/src/cli.rs`, `crates/specforge/src/commands/evidence.rs`, and `crates/specforge/src/commands/converge.rs` so `specforge evidence` and `specforge converge` now consult the local prior store by default through `--prior-memory generated/prior_memory/corpus_memory.json`
- added regression coverage for prior-guided `Producer` / `Consumer` direction recovery from both source columns and section headings
- synced `README.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `DEVELOPMENT_NOTES.md`, `USER_GUIDE.md`, `CHANGES.md`, and `MEMORY.md` so the first bounded `R15f` consumer is continuity-safe
- validation ran for this task:
  - `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_section_heading_direction_inference -- --nocapture` passed
  - `cargo test --manifest-path Cargo.toml converge_defaults_to_ollama_for_vlm_and_nlp -- --nocapture` passed
  - `bash scripts/run_ci.sh` passed with `202/202` tests

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/cli.rs`
  - `crates/specforge/src/commands/evidence.rs`
  - `crates/specforge/src/commands/converge.rs`
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/src/ir/prior_memory.rs`
  - `README.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `DEVELOPMENT_NOTES.md`
  - `USER_GUIDE.md`
  - `CHANGES.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. commit the first bounded prior-consumption slice with the synced live docs
2. broaden `R15f` prior families beyond actor-taxonomy / semantic / temporal into table-shape, visual-motif, modality-reliability, and negative-knowledge
3. broaden prior consumption beyond actor-taxonomy direction guidance into semantic-role and temporal-language suggestion paths while preserving the local-grounding rule

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, multi-producer ambiguity, modality-aware semantic-role grounding, consensus summaries, semantic candidates, and semantic arbitration summaries (`R15d`)
- broader KG-quality evaluation and benchmark hardening beyond the new seed fixture pack (`R15e`)
- cross-document extractor learning is now started, and the first bounded `EvidenceIR` consumer is now live, but `R15f` still needs broader prior families plus benchmarked semantic/temporal/table/visual consumption paths
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
7. continue with the current semantic-truthfulness roadmap unless the user redirects, now treating the cross-document learning plane as an active workstream rather than just a documented future idea
