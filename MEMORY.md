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
- latest_commit_hash: `48034e9`
- latest_commit_brief_message: `chore(git): untrack swap files`
- note: the current session adds the first real `R15f` implementation slice, landing a typed local `CorpusMemory` prior store plus the new `specforge learn-priors <intent_ir>...` command

## Recent commit chain (last 5)
- `48034e9` chore(git): untrack swap files
- `20cd24a` feat(quality): benchmark ahb timing semantics
- `001a6dc` feat(quality): benchmark apb timing semantics
- `234a39f` feat(quality): benchmark axi timing semantics
- `a657305` feat(quality): benchmark axi width-only recovery

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `kg-bench`, `project-validation`, `learn-priors`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports
- tracked KG-quality fixtures now live under `crates/specforge/test_data/kg_quality/`

## Completed technical work in this session
- added `crates/specforge/src/ir/prior_memory.rs`, the first typed `CorpusMemory` schema for the cross-document learning plane
- added `crates/specforge/src/commands/learn_priors.rs` and the new `specforge learn-priors <intent_ir>...` CLI command
- the first `R15f` trust/update policy is now explicit:
  - only validated `IntentIR` artifacts are eligible
  - artifacts with validation error findings are skipped
  - the learning plane stays advisory-only and cannot directly author canonical document facts
- the first harvested prior families are now live:
  - semantic-role phrase priors from decisive, non-alias-dependent canonical semantic consensus plus preserved observation text
  - temporal-language phrase priors from canonical `temporal_rules` plus validated canonical `signal_constraints` / `conditional_rules`
- the first AMBA prior-memory run now materializes a local `generated/prior_memory/corpus_memory.json` with:
  - `3` source artifacts
  - `222` temporal phrase priors
  - `0` semantic phrase priors
- synced `README.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `USER_GUIDE.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` so the new `R15f` slice is continuity-safe
- validation ran for this task:
  - `cargo fmt --all --check` passed
  - `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` passed
  - `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json --output generated/prior_memory/corpus_memory.json` passed
  - `cargo test --manifest-path Cargo.toml` passed

## Current working tree before commit
- modified tracked files currently include:
  - `crates/specforge/src/cli.rs`
  - `crates/specforge/src/commands/learn_priors.rs`
  - `crates/specforge/src/commands/mod.rs`
  - `crates/specforge/src/ir/mod.rs`
  - `crates/specforge/src/ir/prior_memory.rs`
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
1. commit the first `R15f` prior-memory slice with the synced live docs
2. broaden the prior store beyond temporal-language priors into actor-taxonomy, table-shape, visual-motif, modality-reliability, and negative-knowledge priors
3. begin teaching `EvidenceIR` / `SemanticIR` to consume retrieved priors as bounded suggestions without weakening the local-grounding rule

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, multi-producer ambiguity, modality-aware semantic-role grounding, consensus summaries, semantic candidates, and semantic arbitration summaries (`R15d`)
- broader KG-quality evaluation and benchmark hardening beyond the new seed fixture pack (`R15e`)
- cross-document extractor learning is now started, but the typed prior store still needs actor-taxonomy, table-shape, visual-motif, modality-reliability, negative-knowledge, and benchmarked retrieval/consumption paths (`R15f`)
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
