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
- latest_commit_hash: `60e76b5`
- latest_commit_brief_message: `feat(semantic): consume temporal phrase priors safely`
- note: the current session lands the first tracked semantic gold/negative pair that proves prior-guided role recovery improves an unseen local phrase honestly

## Recent commit chain (last 5)
- `60e76b5` feat(semantic): consume temporal phrase priors safely
- `a756010` feat(evidence): consume semantic phrase priors safely
- `7ebd697` feat(evidence): consume actor taxonomy priors safely
- `53da080` feat(learning): learn actor taxonomy priors
- `8a9c3bd` ci(repo): unify local and hosted CI

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `kg-bench`, `project-validation`, `learn-priors`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- GitHub Actions now mirrors the baseline Rust quality gate on every `push` / `pull_request` via `.github/workflows/ci.yml`
- `scripts/run_ci.sh` is now the canonical Rust CI entrypoint and is reused by GitHub Actions, so the same hosted path can be exercised locally before push
- the local `CorpusMemory` prior store now includes actor-taxonomy, semantic, temporal, and table-shape priors
- the thing that materially grows to capture learning is the typed prior store itself, usually `generated/prior_memory/corpus_memory.json`; code defines the learning rules, but the accumulated experience lives in that symbolic memory artifact
- `specforge evidence` and `specforge converge` now consult `--prior-memory generated/prior_memory/corpus_memory.json` by default, and the first bounded consumer uses actor-taxonomy priors to interpret explicit local actor terms in section headings and `Source` / `Destination` columns; width-only section-guided signal tables can now also recover structural `ActorSignalRelation::Drives` edges from that same prior-guided actor vocabulary
- `EvidenceIR` now also has a second bounded prior consumer for semantic phrase priors, and it now persists `prior_memory_path` so later semantic-hint refreshes keep the same advisory prior context
- `SemanticIR` now also has a third bounded prior consumer for temporal phrase priors, using them only as a fallback for local timing text when direct cycle-window parsing fails
- `EvidenceIR` now also has a fourth bounded prior consumer for table-shape priors, using them only when a current structured table is still `unknown`; explicit local `SourceIR.table_kind` values still win outright
- `specforge kg-bench` can now also stage a fixture-local `CorpusMemory`, and the first tracked gold/negative pair proves prior-guided temporal recovery on an unseen local phrase without leaking cross-document facts
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for semantic priors on an unseen local phrase
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for actor-taxonomy priors on unseen local section-heading vocabulary
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for semantic priors on unseen local visual-caption phrasing
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for table-shape priors on a locally `unknown` `Name | Direction | Width` table
- canonical generated artifact roots are under `generated/source_ir/`, `generated/evidence_ir/`, `generated/semantic_ir/`, and `generated/intent_ir/`
- `generated/` is git-ignored and intentionally untracked; continuity must live in docs, not versioned artifacts
- `subs/fsmgen/` is a local read-only reference checkout for `.fsm` behavior
- `VALIDATION_SNAPSHOT.md` is now a tracked continuity doc refreshed from persisted validation reports
- tracked KG-quality fixtures now live under `crates/specforge/test_data/kg_quality/`

## Completed technical work in this session
- logged the learning-plane growth model explicitly:
  - code defines how `R15f` learns
  - the typed prior store captures what has been learned so far
  - the main growing artifact is `generated/prior_memory/corpus_memory.json`
- synced `README.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` so future sessions can quickly recover that distinction after a crash or handoff

## Current working tree before commit
- modified tracked files currently include:
  - `README.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. commit the table-shape prior family plus bounded `EvidenceIR` consumer
2. continue replacing remaining direct `direction_hint` consumers with graph-first semantics
3. broaden `R15f` beyond actor-taxonomy / semantic / temporal / table-shape into visual-motif, modality-reliability, and negative-knowledge prior families

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, multi-producer ambiguity, modality-aware semantic-role grounding, consensus summaries, semantic candidates, and semantic arbitration summaries (`R15d`)
- broader KG-quality evaluation and benchmark hardening beyond the new seed fixture pack (`R15e`)
- cross-document extractor learning is now started, and the first three bounded `EvidenceIR` consumers plus the first `SemanticIR` consumer are now live, but `R15f` still needs broader prior families plus benchmarked visual-motif/modality-reliability/negative-knowledge consumption paths
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
