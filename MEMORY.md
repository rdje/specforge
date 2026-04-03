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

## What the user has explicitly required
- live documents are critical infrastructure, not optional afterthoughts
- all live documents together must preserve full operational continuity after session loss, crashes, or tool restarts
- `RUST_CODEBASE_ANALYSIS.md` must remain a live deep-dive analysis of the Rust codebase
- `MEMORY.md` must be updated after completed tasks and also at meaningful checkpoints while long tasks are in progress
- `MEMORY.md` must contain the latest committed Git hash and the corresponding brief commit message, or explicitly state that no commit exists yet
- `MEMORY.md` must stay compact, precise, and operational rather than becoming a verbatim transcript
- `CHANGES.md` must contain the full detailed set of changes about to be committed
- `DEVELOPMENT_NOTES.md` must capture engineering decisions, rationale, and context
- `USER_GUIDE.md` must explain the tool from the user perspective
- `ROADMAP.md` must track the goals, remaining work, and advancement
- `README.md` must be the single entry point and its last line must be:
  - `Read SESSION_BOOTSTRAP.md and start from there.`
- `subs/fsmgen` is contextual only and must be treated as read-only from `specforge`
- if `fsmgen` misbehavior is identified, file a thorough local tracked bug report instead of patching the submodule in place
- use the local bug-report ID format `FSMGEN-BUG-####` for such upstream reports

## Repository state at this checkpoint
- Git repository exists
- the live documentation surface is established
- the active workspace member is `crates/specforge`
- the runnable CLI command surface now includes:
  - `inspect <path>`
  - `ingest <source> --dry-run`
  - `ingest <source>`
  - `evidence <source-ir> --dry-run`
  - `evidence <source-ir>`
  - `semantic <evidence-ir> --dry-run`
  - `semantic <evidence-ir>`
  - `intent <semantic-ir> --dry-run`
  - `intent <semantic-ir>`
  - `adapt <intent-ir> --target fsm --dry-run`
  - `adapt <intent-ir> --target fsm`
- the currently implemented real stage artifacts are `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, and the first renderable `.fsm` adapter slices for explicit standalone DT, widened canonical symbol/control lowering, selector/test-node branches, compound-update shorthand, explicit structured FSM cases, and explicit top-root composition cases, with the adapter root-kind surface now intentionally limited to `dt` / `fsm` / `top`
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters

## Latest committed baseline
- latest_commit_hash: `fdee7c2`
- latest_commit_brief_message: `chore: AHB+APB end-to-end pipeline run; add APB artifacts`
- continuity_rule:
  - refresh this section whenever a new latest committed baseline exists at the time `MEMORY.md` is updated

## Recent commit chain (last 5)
- `fdee7c2` chore: AHB+APB end-to-end pipeline run; add APB artifacts (AHB=86/100, APB=35/100)
- `115cfc5` refactor(nlp): remove --max-passes, use residual-stable convergence criterion
- `c015f61` feat(nlp): Form 2 signal alias learning feedback loop
- `8820698` feat(nlp): Form 1 backannotation feedback loop
- `d1ad9c5` feat(nlp): implement Layers A/B/C/D/E — boilerplate suppression, grounded multi-pass NLP, declared-signal gating, spec-type-aware scoring

## Important session history
- the repository bootstrap and first baseline commit were completed earlier in the session
- the user then requested a stronger architecture:
  - stop treating `.fsm` as the endpoint
  - make `IntentIR` the canonical deliverable
  - keep backends as adapters
  - rename the CLI direction from `spec2fsm` to `specforge`
  - redesign the pipeline explicitly as `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- the user then clarified that PDF images, figures, and diagrams must be treated as semantically meaningful inputs when they carry information content
- the user explicitly required a strict SOTA quality bar for every stage of the tool
- the repo docs were rewritten around that architecture
- `INTENTIR_SPEC.md` was added as the canonical long-form architecture/spec document
- the Rust crate and CLI identity were renamed to `specforge`
- the Rust types were refactored so the current ingest slice is now explicitly `SourceIR`
- typed scaffolding was added for `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter planning
- `SourceIR` was further extended to reserve parser-backend, page-artifact, and visual-asset schema surface
- `EvidenceIR` was further extended to reserve multimodal visual-evidence records and text-to-figure linkage
- the next slice has now implemented a real Docling-backed PDF normalization path inside `SourceIR`
- the current slice has now implemented the first real `EvidenceIR` extractor and CLI on top of ready `SourceIR` artifacts
- the current slice has now implemented the first real `SemanticIR` extractor and CLI on top of ready `EvidenceIR` artifacts
- the current slice has now implemented the first real `IntentIR` constructor and CLI on top of ready `SemanticIR` artifacts
- the current slice has now added `subs/fsmgen` as a pinned local git submodule for `.fsm` adapter reference work
- the user has now clarified that `subs/fsmgen` is contextual-only, read-only, and any observed upstream misbehavior must be tracked locally under `FSMGEN-BUG-####`
- the current slice has now implemented the first DT-centric `.fsm` adapter artifact and `specforge adapt` command on top of persisted `IntentIR`
- the current slice has now enriched `SemanticIR` and `IntentIR` with backend-neutral interface/control records just far enough to support honest standalone renderable `.fsm` emission
- the current slice has now widened the `.fsm` adapter so it can emit a real standalone `?dt:name` file when the canonical facts are explicit and keep broader cases blocked otherwise
- the current slice has now enriched `SemanticIR` and `IntentIR` with backend-neutral system contract and init-assignment records just far enough to support honest standalone sequential `?dt:name` emission
- the current slice has now widened the `.fsm` adapter so explicit standalone sequential DT cases can emit `(+system ...)` and `(:= ...)` while broader `?fsm:name` and composition roots remain blocked
- the current slice has now enriched `SemanticIR` and `IntentIR` with backend-neutral regular-state and transition records from explicit `State ...` and `Transition ...` statements
- the current slice has now widened the `.fsm` adapter so explicit state-graph cases can emit honest structured `?fsm:name` text while composition roots remain blocked
- the current slice has now enriched `SemanticIR` and `IntentIR` with explicit backend-neutral module/top composition facts from `Module ...` and `Top ...` statements
- the current slice has now widened the `.fsm` adapter so explicit composition cases can emit honest `?top:name` source documents with embedded DT/FSM child roots while direct `?mod:name` / `?module:name` alias roots remain deferred
- the current slice has now widened `SemanticIR` and `IntentIR` further so canonical `.fsm`-relevant symbol definitions and structured control blocks, including dedicated synchronous-reset and asynchronous-reset roles, are preserved explicitly
- the current slice has now widened the `.fsm` adapter so renderable DT/FSM lowering consumes canonical symbol-definition/control-block surface first and now lowers honest selector/test-node branches plus compound-update shorthand while still blocking unsupported selector/predicate shapes instead of inventing target syntax
- the current slice has now reviewed the `fsmgen` direct-root contract and confirmed that `?mod:` / `?module:` remain compatibility-level accepted spellings rather than a settled backend-neutral direct-module distinction for SpecForge, so the adapter root-kind surface now stays limited to `dt` / `fsm` / `top`
- the user has now clarified the real-hardware reset model in more detail, and the canonical reset contract now preserves reset kind, polarity, assertion timing, release timing, and reset-target semantics explicitly rather than leaving them implicit

## In-flight work in this session (2026-04-03)

### Completed this session
- Layer A (boilerplate suppression), Layer D (declared-signal gating), Layer E (spec-type-aware quality scoring) implemented
- Form 1 (backannotation), Form 2 (signal alias learning), residual-stable convergence loop implemented
- 90 tests, all passing
- AHB pipeline run: 86/100 GOOD
- APB pipeline run: 35/100 (Requester/Completer direction bug)
- AXI pipeline run: 85/100 (misleading: 1 of ~100+ signals declared)
- Docling Python at `/opt/homebrew/opt/python@3.11/bin/python3.11` (set SPECFORGE_DOCLING_PYTHON)

### Key architectural decision made this session
A deep discussion with the user established the knowledge graph extraction vision. See `KNOWLEDGE_GRAPH_ARCHITECTURE.md` (new) and `DEVELOPMENT_NOTES.md` §Knowledge graph extraction for the full design record. Summary:
- Tables give signal NAMES; prose gives RELATIONS between actors and signals
- Direction is ALWAYS relative to a specific actor, never absolute
- Actor identity is behavioral (what it DOES), not lexical (what it is called)
- Verb phrases encode typed triples: (actor, drives|reads, signal)
- The structural knowledge graph = the block diagram
- The behavioral layer (signals over time, clock-driven) = the waveforms/FSM
- New IR type needed: `ActorSignalRelation { actor_name, signal_name, relation: Drives|Reads }`
- New command planned: `specforge signal-resolve`

### Immediately pending bugs to fix
1. **Bug 1 — alias garbage filter** (2 lines): `extract_alias_phrase()` in `nlp_enrich.rs` produces "- the address" from markdown list items. Reject phrases starting with "-", "|", "#".
2. **Bug 2 — APB direction from Source column** (5 lines): In `synthesize_signal_declarations()` in `evidence.rs`, extend direction cell value parser: requester/initiator/manager/master→output; completer/responder/subordinate/slave/target→input; clock/reset/system→input.

### Next actions in order
1. Fix Bug 1 and Bug 2 (quick, ~30 min)
2. Re-run APB pipeline (expected ~75/100)
3. Implement ActorSignalRelation record type + Tier 2 prose extraction (R13)
4. Implement `specforge signal-resolve` command (R14)
5. Re-run APB and AXI (expected 80-90+/100)

## Old in-flight work (pre-2026-04-03 session)
- real-world PDF testing against AMBA AHB spec is active (user instruction: "test using AHB pdf until further notice")
- SOTA SourceIR/EvidenceIR extraction architecture fully implemented and validated on AHB PDF
- EXTRACTION_ARCHITECTURE.md created as the permanent reference document for the SOTA extraction vision
- SemanticIR parse_signal_table_row band-aid removed; signal declarations now flow architecturally from EvidenceIR
- Level 1+2 NLP: full vocabulary expansion (cannot/is not permitted/unless/is tied high/multi-signal/etc.) + 15 regression tests
- Level 3 NLP: specforge nlp-enrich command implemented with qwen2.5vl:7b via Ollama
- VLM truncation bug fixed; qwen2.5vl:7b set as default Ollama model; model pulled and ready
- Test suite: 75 tests, all passing
- NEXT: AHB end-to-end pipeline run with specforge validate to measure actual coverage improvements
- VLM observations wired into EvidenceIR and SemanticIR: timing annotations → TimingConstraintRecord; state/transition JSON → RegularStateRecord/StateTransitionRecord
- specforge validate command implemented for all four IR stages with quality score
- test suite expanded to 55 tests; all passing
- EXTRACTION_ARCHITECTURE.md statuses fully corrected (all Tier 1–3 steps updated)
- NLP Level 3 plan defined as Step 3.4 in EXTRACTION_ARCHITECTURE.md and new R11 workstream in ROADMAP.md

## Current execution checkpoint
- `IntentIR` now has a real build/materialization path
- the repository now includes:
  - `.gitmodules`
  - `subs/fsmgen` pinned at `57f00e581b4fc9a2aa02318846d1eb8a726c8960`
- `subs/fsmgen` is now explicitly treated as read-only contextual input, with future upstream bug reports to be tracked locally as `FSMGEN-BUG-####`
- execute-mode `specforge intent` now writes:
  - `generated/intent_ir/<document_key>/intent_ir.json`
  - intent identity, actor responsibilities, behaviors, constraints, assumptions, and residual decisions
- execute-mode `specforge adapt --target fsm` now writes:
  - `generated/adapters/fsm/<document_key>/adapter.json`
  - root-kind choice (`?dt:name`, `?fsm:name`, or `?top:name`), canonical signal inventory, canonical system/init surface, canonical control/state/transition candidates, explicit module/top candidate counts, renderability status, and adapter residual decisions
  - a real emitted standalone, structured, or explicit top-root `.fsm` file when every referenced signal has explicit width/direction, every rendered control fragment is fully typed, any sequential/stateful case has explicit system/init facts, any explicit top composition has renderable child modules plus width-compatible links, and any selector/test-node or compound-update control maps directly into the current honest `.fsm` surface
- `SemanticIR` now preserves:
  - typed signal records when explicit declarations are present
  - backend-neutral system contract and init-assignment records from explicit `Clock ...`, `Reset ...`, and `Init ...` statements, including first-class reset polarity/assertion/release/target semantics
  - backend-neutral regular-state and transition records from explicit `State ...` and `Transition ...` statements
  - explicit backend-neutral module records from `Module ...` statements
  - explicit backend-neutral top-composition records from `Top ...` statements
  - canonical symbol-definition sections for explicit `Constant`, `Define`, `Param`, and `Enum` statements
  - structured control blocks with branch-local actions and dedicated synchronous-reset/asynchronous-reset roles
  - backend-neutral guarded/action control fragments from explicit `Block ...` statements
- `IntentIR` now carries:
  - canonical interface inventory
  - canonical backend-neutral system contract and init assignments, including first-class reset polarity/assertion/release/target semantics
  - canonical regular states and state transitions
  - explicit canonical module and top-composition records
  - canonical symbol-definition sections
  - canonical structured control blocks, including dedicated reset-role blocks
  - backend-neutral guarded/action control fragments
- the current validation set is:
  - `cargo fmt --all --manifest-path Cargo.toml --check`
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/inferred_reset_cli.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/inferred_reset_cli/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/inferred_reset_cli/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/inferred_reset_cli/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/inferred_reset_cli/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/handshake.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/handshake/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/handshake/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/handshake/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/handshake/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/comb_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/comb_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/comb_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/comb_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/comb_dt/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/seq_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/seq_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/seq_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/seq_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/seq_dt/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_fsm.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_fsm/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_fsm/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_fsm/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_fsm/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_top.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_top/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_top/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_top/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_top/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/sync_control.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/sync_control/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/sync_control/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/sync_control/intent_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/sync_control/intent_ir.json --target fsm`
  - `cargo test --manifest-path Cargo.toml adapters`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/selector_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/selector_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/selector_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/selector_dt/intent_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/selector_dt/intent_ir.json --target fsm`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/compound_update_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/compound_update_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/compound_update_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/compound_update_dt/intent_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/compound_update_dt/intent_ir.json --target fsm`
- the next implementation action is R12: run AHB end-to-end pipeline with specforge validate to quantify NLP coverage improvements
- the runnable CLI command surface now also includes:
  - `enrich <source-ir> --vlm-provider <ollama|openai|lmstudio|skip>`
  - `nlp-enrich <evidence-ir> --vlm-provider <ollama|openai|lmstudio|skip>`
  - `validate <artifact>` (autodetects IR stage)

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. read `INTENTIR_SPEC.md`
4. inspect `LIVE_ACHIEVEMENT_STATUS.md`
5. inspect `ROADMAP.md`
6. inspect `RUST_CODEBASE_ANALYSIS.md`
7. continue with the next implementation slice unless the user redirects

## Recommended next implementation slice
- R12: run AHB end-to-end pipeline run with specforge validate at each stage; record quality scores
  - measure actual NormativeStatement residual after Level 1+2 expansion (was 91 before; expected ~40–50 after)
  - run specforge nlp-enrich --vlm-provider ollama --vlm-model qwen2.5vl:7b on AHB EvidenceIR
  - compare quality score before/after both enrichment steps
  - identify remaining gaps before starting RTL adapter work (SystemVerilog)
- keep `IntentIR` as the canonical endpoint and keep adapters downstream of it

## Commit status
- the latest committed baseline is `464a1a1d73d352f08396cb24e544c3675e4d74d5`
- the current working tree contains the uncommitted direct-module defer decision slice, the earlier selector/test-node and compound-update `.fsm` lowering slice, the earlier first-class reset-contract widening, and the matching live-doc refreshes
- before the next commit, follow `COMMIT.md`
