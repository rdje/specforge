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
- latest_commit_hash: `9907fe8`
- latest_commit_brief_message: `feat(semantic): tighten graph-backed interface coverage`
- note: the current session is now focused on clock/reset infrastructure semantics as a steering constraint after the AXI-Stream graph-coverage slice

## Recent commit chain (last 5)
- `9907fe8` feat(semantic): tighten graph-backed interface coverage
- `bd2b553` feat(evidence): recover complementary consumer edges
- `23eb7d1` feat(learning): reject bogus actor priors
- `f8932dc` feat(evidence): reject bogus prose actors
- `d490611` feat(evidence): fix multi-signal table role leakage

## Current repository state
- active workspace member: `crates/specforge`
- runnable CLI surface includes `inspect`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `kg-bench`, `project-validation`, `learn-priors`, and `nlp-enrich`
- `specforge converge` now defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; use `--vlm-provider skip` and/or `--nlp-provider skip` only when intentionally narrowing the loop
- GitHub Actions now mirrors the baseline Rust quality gate on every `push` / `pull_request` via `.github/workflows/ci.yml`
- `scripts/run_ci.sh` is now the canonical Rust CI entrypoint and is reused by GitHub Actions, so the same hosted path can be exercised locally before push
- the local `CorpusMemory` prior store now includes actor-taxonomy, semantic phrase, semantic modality-reliability, temporal, and table-shape prior families
- the thing that materially grows to capture learning is the typed prior store itself, usually `generated/prior_memory/corpus_memory.json`; code defines the learning rules, but the accumulated experience lives in that symbolic memory artifact
- `specforge evidence` and `specforge converge` now consult `--prior-memory generated/prior_memory/corpus_memory.json` by default, and the first bounded consumer uses actor-taxonomy priors to interpret explicit local actor terms in section headings and `Source` / `Destination` columns; width-only section-guided signal tables can now also recover structural `ActorSignalRelation::Drives` edges from that same prior-guided actor vocabulary
- `EvidenceIR` now also has a second bounded prior consumer for semantic phrase priors, and it now persists `prior_memory_path` so later semantic-hint refreshes keep the same advisory prior context
- `SemanticIR` now also has a third bounded prior consumer for temporal phrase priors, using them only as a fallback for local timing text when direct cycle-window parsing fails
- `EvidenceIR` now also has a fourth bounded prior consumer for table-shape priors, using them only when a current structured table is still `unknown`; explicit local `SourceIR.table_kind` values still win outright
- `SemanticIR` now also has a fifth bounded prior consumer overall and a second semantic-stage one for semantic modality-reliability priors, using them only to advisory-adjust arbitration between already-present locally grounded semantic candidates while preserving the underlying conflict surface
- the roadmap now also carries a new `R15g` workstream for a corpus knowledge base plane beside the KG and typed priors, so persistent cross-document synthesis has an explicit home instead of being forced into either canonical IR or `CorpusMemory`
- the latest live four-document prior-memory run over AXI/APB/AHB/AXI-Stream now yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors
- AXI-Stream is now the first unseen protocol run carried all the way through the full loopbacked path: it converged in `2` pipeline iterations with Ollama VLM + NLP Level 3, now validates at `90/100 EXCELLENT`, and its artifact is included in the tracked validation snapshot
- AXI-Stream prose KG extraction is now hardened against bogus payload/event actors: `control information` no longer survives as a producer for `TVALID`, AXI-Stream `signal_connectivity_conflicts` is now `0`, and the latest source-table fix also closes the missing consumer-side connectivity gap
- AXI-Stream interface coverage is now more honest too: bogus width-only `_WIDTH` symbols no longer count as declared interface signals, relation-grounded actors carry graph-backed `ACLK` / `ARESETN` input ports, and parity-check tables now recover both local structural ownership and carried width hints for the `*CHK` surface so declared graph-direction and width coverage are both `22/22`
- clock/reset handling now has an explicit steering rule in the live docs: those signals are infrastructure semantics, not ordinary protocol edges, so future canonical modeling should prefer dedicated infrastructure records and conservative ownership over false graph completeness
- the learning plane now applies the same bogus-actor hygiene rule at harvest and lookup time, and the stale `control information -> requester_like` actor-taxonomy prior has been removed from local `CorpusMemory`
- `specforge kg-bench` can now also stage a fixture-local `CorpusMemory`, and the first tracked gold/negative pair proves prior-guided temporal recovery on an unseen local phrase without leaking cross-document facts
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for semantic priors on an unseen local phrase
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for actor-taxonomy priors on unseen local section-heading vocabulary
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for semantic priors on unseen local visual-caption phrasing
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for table-shape priors on a locally `unknown` `Name | Direction | Width` table
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for table-shape priors on a locally `unknown` `Parameter | Min | Max | Unit` timing table
- `specforge kg-bench` now also locks the same before/after truthfulness pattern for semantic modality-reliability priors on locally conflicted semantic-role evidence, proving the conflict stays contested without the staged prior and becomes decisively resolved only when that prior is present
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
- logged the complementary architectural doctrine for "how to learn":
  - `R15f` should borrow the useful structural properties of human learning without trying to mimic humans completely
  - the learning plane must keep strict separation, typed memory, bounded influence, validation-gated feedback, negative learning, and provenance
  - the learning plane should be treated as a first-class epistemology layer, not as a side feature
- synced `README.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` so future sessions can quickly recover that distinction after a crash or handoff
- fixed the AXI-Stream table-row semantic leak where secondary signal mentions in one signal-description row could assign the wrong handshake role back to the row subject
- fixed the AXI-Stream prose relation-extraction leak where coordinated clauses like `Transmitter presents ... and asserts TVALID` could promote payload nouns like `control information` into canonical actors
- fixed the learning-plane follow-on bug where `learn-priors` could still harvest those same payload nouns into actor-taxonomy memory after the document-local extraction bug had been repaired
- ran full `converge` on `IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf` with Ollama VLM + NLP Level 3 enabled, validated the resulting `IntentIR`, and refreshed the tracked validation snapshot to include the new artifact
- re-ran full `converge` on AXI-Stream after the prose actor fix, confirmed convergence still takes `2` iterations, and refreshed the tracked validation snapshot so the carried producer conflict is now gone from the projected artifact state
- re-ran `specforge learn-priors` across AXI/APB/AHB/AXI-Stream after the learning-plane hygiene fix and confirmed the stale `control information` actor prior is now gone from local `CorpusMemory`
- refreshed `generated/prior_memory/corpus_memory.json` across AXI/APB/AHB/AXI-Stream, producing the first nonzero semantic phrase and semantic modality-reliability priors on a real corpus
- removed bogus width-only `_WIDTH` pseudo-signals from canonical interface coverage and added graph-backed clock/reset input ports for relation-grounded actors
- re-ran full `converge` on AXI-Stream after the semantic graph-coverage fix, confirmed convergence still takes `2` iterations, and refreshed the tracked validation snapshot so AXI-Stream now projects at `84/100 GOOD` with `12/16` graph-direction coverage
- added bounded parity-check relation recovery from `Check Signal / Signals Covered` tables and re-ran AXI-Stream through the full loopbacked path; the artifact still converges in `2` iterations and now projects at `88/100 GOOD` with `21/21` graph-direction coverage
- extended parity-check row recovery so the same local table semantics now also restore missing `*CHK` width hints and no longer let width-only statements block stronger graph-derived declarations; re-ran AXI-Stream through the full loopbacked path and the artifact now projects at `90/100 EXCELLENT` with declared graph-direction and width coverage both at `22/22`
- logged the next architectural layer after `CorpusMemory`: a tracked corpus knowledge base that can accumulate recurring protocol motifs, extraction failure archetypes, contradiction summaries, table/figure families, and infrastructure notes without directly authoring canonical IR truth
- logged the clock/reset infrastructure-semantics doctrine so future work does not confuse graph carry-through with ordinary producer/consumer semantics for those signals

## Current repo hygiene expectations
- completed tasks should end in a clean working tree after the commit workflow runs
- generated artifacts remain local-only and should stay untracked unless the user explicitly asks otherwise

## Exact next steps
1. decide the canonical producer-attribution policy for shared infrastructure signals like `ACLK` and `ARESETN`
2. recover explicit cycle-window bounds for the remaining typed temporal rules in AXI-Stream and similar timing-rich unseen protocols
3. broaden `R15f` beyond actor-taxonomy / semantic / semantic-modality-reliability / temporal / table-shape into visual-motif and negative-knowledge prior families

## Remaining engineering gaps after this commit
- remaining actor-relative direction modeling in `SemanticIR` / `IntentIR` (`R15`)
- richer explicit clock-tick temporal semantics (`R15b`)
- KG-guided multimodal rescans (`R15c`)
- broader evidence arbitration / conflict handling beyond polarity, interface-shape, multi-producer ambiguity, modality-aware semantic-role grounding, consensus summaries, semantic candidates, and semantic arbitration summaries (`R15d`)
- broader KG-quality evaluation and benchmark hardening beyond the new seed fixture pack (`R15e`)
- cross-document extractor learning is now started, and the first four bounded `EvidenceIR`/`SemanticIR` consumer families are now live, but `R15f` still needs broader prior families plus benchmarked visual-motif and negative-knowledge consumption paths
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
