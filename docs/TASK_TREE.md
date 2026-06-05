# Repo-Local Task Tree Workflow

This document defines the repo-local task-tree workflow used by SpecForge.
Adapted from the FSMGen task-tree system.

See [`docs/TASK_TREE_README.md`](TASK_TREE_README.md) for the system overview
and the governing doctrine (no code change without an owning tree; every
activity tracked; past work audited into trees; ROADMAP ↔ codebase ↔ mdBook
locked with zero drift). Whole-roadmap task-tree coverage + the past-work audit
is driven by [`docs/tasks/ROADMAP-TASKTREE-COVERAGE.md`](tasks/ROADMAP-TASKTREE-COVERAGE.md).

## Purpose

Use a task tree when a top-level task is too broad to finish safely as one
signoff-level slice, or when a task is expected to discover subtasks and
sub-subtasks over time.

The goal is not to create a second roadmap. The roadmap states the high-level
workstream direction. A task tree owns the recursive breakdown, current
frontier, acceptance criteria, blockers, decisions, validation, and completion
evidence for one top-level task.

## Active Task Trees

| Tree | Status | Roadmap lane | Current frontier | File |
| --- | --- | --- | --- | --- |
| `PROVENANCE-HARDENING` | `done` | `R6` | — | [docs/tasks/PROVENANCE-HARDENING.md](docs/tasks/PROVENANCE-HARDENING.md) |
| `R6-FSM-ADAPTER` | `superseded` | `R6` | — (superseded by `ISF-ONLY-CONSOLIDATION`) | [docs/tasks/R6-FSM-ADAPTER.md](docs/tasks/R6-FSM-ADAPTER.md) |
| `R6-SOURCE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-SOURCE-HARDENING.md](docs/tasks/R6-SOURCE-HARDENING.md) |
| `R6-CONVERGE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-CONVERGE-HARDENING.md](docs/tasks/R6-CONVERGE-HARDENING.md) |
| `R6-EVIDENCE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-EVIDENCE-HARDENING.md](docs/tasks/R6-EVIDENCE-HARDENING.md) |
| `R6-INTENT-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-INTENT-HARDENING.md](docs/tasks/R6-INTENT-HARDENING.md) |
| `R6-SEMANTIC-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-SEMANTIC-HARDENING.md](docs/tasks/R6-SEMANTIC-HARDENING.md) |
| `R6-PRIOR-MEMORY-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-PRIOR-MEMORY-HARDENING.md](docs/tasks/R6-PRIOR-MEMORY-HARDENING.md) |
| `R15-GRAPH-DIRECTION-MIGRATION` | `done` | `R15` | — | [docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md](docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md) |
| `R7-VALIDATION` | `done` | `R7` | — (closed `2026-05-20`; `.1`–`.4` implemented `2026-05-16`; `.5` design-deliverable landed `2026-05-20` — tracked-approval-evidence design for canonical IR mutation; implementation pathway stays gated on user-owned decision) | [docs/tasks/R7-VALIDATION.md](docs/tasks/R7-VALIDATION.md) |
| `SIGNOFF-REMEDIATION` | `done` | `R0` | — | [docs/tasks/SIGNOFF-REMEDIATION.md](docs/tasks/SIGNOFF-REMEDIATION.md) |
| `R6-ISF-ADAPTER` | `done` | `R6` | — | [docs/tasks/R6-ISF-ADAPTER.md](docs/tasks/R6-ISF-ADAPTER.md) |
| `ISF-ONLY-CONSOLIDATION` | `done` | `R6` | — | [docs/tasks/ISF-ONLY-CONSOLIDATION.md](docs/tasks/ISF-ONLY-CONSOLIDATION.md) |
| `ISF-TEMPORAL-LOWERING` | `done` | `R15b` | — | [docs/tasks/ISF-TEMPORAL-LOWERING.md](docs/tasks/ISF-TEMPORAL-LOWERING.md) |
| `ISF-ONLY-IR-PRUNE` | `done` | `R6` | — | [docs/tasks/ISF-ONLY-IR-PRUNE.md](docs/tasks/ISF-ONLY-IR-PRUNE.md) |
| `AUDIT-DOC-RECONCILE` | `done` | `R0` | — | [docs/tasks/AUDIT-DOC-RECONCILE.md](docs/tasks/AUDIT-DOC-RECONCILE.md) |
| `FSMGEN-ISSUE-REPORTING` | `done` | `R0` | — | [docs/tasks/FSMGEN-ISSUE-REPORTING.md](docs/tasks/FSMGEN-ISSUE-REPORTING.md) |
| `FSMGEN-SUBMODULE-BUMP` | `done` | `R0` | — | [docs/tasks/FSMGEN-SUBMODULE-BUMP.md](docs/tasks/FSMGEN-SUBMODULE-BUMP.md) |
| `ISF-HANDSHAKE-STAGE-LOWERING` | `superseded` | `R15b` | — (delivered via `R16-CONTRACT-IR.4`) | [docs/tasks/ISF-HANDSHAKE-STAGE-LOWERING.md](docs/tasks/ISF-HANDSHAKE-STAGE-LOWERING.md) |
| `R16-INTENT-CAPTURE` | `done` | `R16` | — (umbrella CLOSED `2026-05-29`; **R16 program complete** — all 6 sub-trees done at their honest scope boundaries #1/#2/#3/#4/#5/#6; 2 remaining crux items as honestly-deferred FUTURE trees: CVE producer wiring (**NOT** upstream-blocked — the Ollama+Qwen2.5VL prose provider is production-default; needs wiring into `parse_constrained_contract`) + waveform raster/vector `FigureRegion` extraction (the genuinely upstream-absent piece)) | [docs/tasks/R16-INTENT-CAPTURE.md](docs/tasks/R16-INTENT-CAPTURE.md) |
| `R16-MODULE-HARDENING` | `done` | `R16` | — (CLOSED `2026-05-29`; all 7 R16 IR modules at the unit-test signoff bar via `.1`–`.4`; R16 module tests `67 → 86`) | [docs/tasks/R16-MODULE-HARDENING.md](docs/tasks/R16-MODULE-HARDENING.md) |
| `FSMGEN-REFRESH-INTEGRATE` | `done` | `R6` | — (CLOSED `2026-05-29`: bumped `subs/fsmgen` `9bfb9a20` → `88a7af9c` + verified, ISF feature-adoption assessment recorded; adoptable work → proposed trees `ISF-SYMBOL-SURFACE-EMIT` + `ISF-TXN-GRAMMAR-FIX`) | [docs/tasks/FSMGEN-REFRESH-INTEGRATE.md](docs/tasks/FSMGEN-REFRESH-INTEGRATE.md) |
| `ISF-TXN-GRAMMAR-FIX` | `done` | `R6` | — (CLOSED `2026-05-29`: 6 emitter grammar sites corrected to FSMGen-contract-exact forms `shift_left`/`shift_right`/`await_all`/`await_any` + `spawn … as`; book-verified; render-lock test) | [docs/tasks/ISF-TXN-GRAMMAR-FIX.md](docs/tasks/ISF-TXN-GRAMMAR-FIX.md) |
| `ISF-SYMBOL-SURFACE-EMIT` | `done` | `R6` | — (CLOSED `2026-05-29`: ingested FSMGen's enum-type answer (pin `c0b7eaa7`); `render()` now emits `(types)`/`(enums)`/`(constants)`, fsmgen-`--strict`-verified e2e; count-vs-emission honesty gap closed) | [docs/tasks/ISF-SYMBOL-SURFACE-EMIT.md](docs/tasks/ISF-SYMBOL-SURFACE-EMIT.md) |
| `ISF-SYMBOL-COUNT-EMITTED` | `done` | `R6` | — (CLOSED `2026-05-29`: adapter `constant_count`/`enum_count` now count emitted content via DRY-shared `emitted_*` accessors — metric == emitted, mirroring the transaction/rule counts) | [docs/tasks/ISF-SYMBOL-COUNT-EMITTED.md](docs/tasks/ISF-SYMBOL-COUNT-EMITTED.md) |
| `R16-CONTRACT-IR` | `done` | `R16` | — (program point #1 delivered) | [docs/tasks/R16-CONTRACT-IR.md](docs/tasks/R16-CONTRACT-IR.md) |
| `R16-KG-PROTOCOL-ONTOLOGY` | `done` | `R16` | — (program point #2 delivered `2026-05-20`; typed `protocol_graph` + projection + `validate` count surface; kg-bench fixtures deferred to extraction trees) | [docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md](docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md) |
| `R16-CAPTURE-FIDELITY-GATES` | `done` | `R16` | — (program point #5/order-3 delivered `2026-05-20`; typed `fidelity` + producer with honesty-doctrine routing + `validate fidelity:` block; corpus baseline `fail=0 score=1.000`) | [docs/tasks/R16-CAPTURE-FIDELITY-GATES.md](docs/tasks/R16-CAPTURE-FIDELITY-GATES.md) |
| `R16-MULTIMODAL-CONTRACT-FUSION` | `done` | `R16` | — (program point #3 delivered `2026-05-20`; typed `fusion` + producer with disagreement→Residual + `validate fusion:` block; corpus baseline `groups_merged=0 disagreements=0`; load-bearing for extraction) | [docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md](docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md) |
| `R16-WAVEFORM-CONTRACT-MINING` | `done` | `R16` | — (program point #4 fully delivered `2026-05-20`: `.1`+`.2`+`.3` (`.3.1`+`.3.2`)+`.4`; `validate waveform: figure_contracts=0 verifier_fail_residuals=0` corpus baseline; raster/vector handling deferred to a future tree when upstream PDF pipeline produces those bytes) | [docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md](docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md) |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION` | `done` | `R16` | — (program point #6 delivered `2026-05-20`; fails-closed adapter + entailment verifier + template library + uncertainty VoI selector + `validate constrained:` block; corpus baseline 0/0/0; closes R16 program) | [docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md](docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md) |
| `BOOK-METHOD-DOC` | `done` | `R0` | — (closed `2026-05-20`: convention + placement map + 7+2 backfilled trees + close-rule encoded in workflow doctrine; close-rule now structurally enforced via Completion Rules + Required Commit Workflow) | [docs/tasks/BOOK-METHOD-DOC.md](docs/tasks/BOOK-METHOD-DOC.md) |
| `BOOK-USER-FRIENDLY-BACKFILL` | `done` | `R0` | — (closed `2026-05-20`: 10 concept-introducing subsections upgraded across 5 book chapters via per-subsection sub-leaves; BOOK-METHOD-DOC Decisions cross-referenced — standard fully self-referential) | [docs/tasks/BOOK-USER-FRIENDLY-BACKFILL.md](docs/tasks/BOOK-USER-FRIENDLY-BACKFILL.md) |
| `AUDIT-PROVIDER-FRAMING-RECONCILE` | `done` | `R0` | — (CLOSED `2026-05-30`: reconciled the "prose LLM/VLM provider doesn't exist / R16 CVE crux upstream-blocked" framing across ROADMAP/TASK_TREE/INTENTIR_SPEC/RUST_CODEBASE_ANALYSIS — provider is production-default Ollama+Qwen2.5VL; CVE wiring NOT upstream-blocked) | [docs/tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md](docs/tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md) |
| `CVE-PROSE-EXTRACTION` | `done` | `R16` | — (CLOSED `2026-05-31`: live prose→`ActorContract` extractor wired into the R16 constrained-verified surface — `extract-contracts` command (Qwen via Ollama), fails-closed + entailment-gated, folded into `actor_contracts` before fusion; `validate` `schema_rejects` now real. Live inference dispatched but server-gated; wiring proven by unit tests + skip-mode. lib →1159) | [docs/tasks/CVE-PROSE-EXTRACTION.md](docs/tasks/CVE-PROSE-EXTRACTION.md) |
| `R14-SIGNAL-RESOLVE` | `done` | `R14` | — (CLOSED `2026-05-31`: Tier-3 LLM `signal_relation` extraction delivered as the `signal-resolve` command (Qwen via Ollama) — grounded + deduped `ActorSignalRelation` edges appended to the existing `EvidenceIR.actor_signal_relations` KG field; pure classifier 5-path-tested. Live run server-gated; skip-mode verified. lib →1164) | [docs/tasks/R14-SIGNAL-RESOLVE.md](docs/tasks/R14-SIGNAL-RESOLVE.md) |
| `LLM-TEXT-TRANSPORT-DEDUP` | `done` | `R0` | — (CLOSED `2026-05-31`: consolidated the text chat transport `extract_contracts`+`signal_resolve` duplicated into a shared `commands/llm_text.rs`; 3→2 copies (`nlp_enrich` inline left as follow-up); +3 transport tests; no behavior change; CI green) | [docs/tasks/LLM-TEXT-TRANSPORT-DEDUP.md](docs/tasks/LLM-TEXT-TRANSPORT-DEDUP.md) |
| `ISF-RULE-CONFLICT-RESIDUAL` | `done` | `R6` | — (CLOSED `2026-05-31`: `.isf` emitter dedup now records each dropped value-conflicting rule as an explicit `ResidualDecisionPacket` (→ `residual_decisions`) instead of silently dropping it; emitted `.isf` byte-identical/strict-valid; extracted testable `dedup_conflicting_rules`; +2 tests; book subsection) | [docs/tasks/ISF-RULE-CONFLICT-RESIDUAL.md](docs/tasks/ISF-RULE-CONFLICT-RESIDUAL.md) |
| `ROADMAP-TASKTREE-COVERAGE` | `done` | `R0` | — (CLOSED `2026-05-31`: turned the WHOLE roadmap into owned, audited task-trees + locked ROADMAP↔code↔mdBook. R1–R5 + R8–R13 audited-closed; R15c–g audited-owned (stays active); `.5` alignment lock reconciled 2 real book drifts code-is-truth — added `extract-contracts`+`signal-resolve` to the command reference (19/19) + rewrote the fictitious 8-finding `.fsm`-adapter-validator claim to the real `.isf`-only 6-finding surface; full CI green 1169/0; user doctrine directive `2026-05-31`) | [docs/tasks/ROADMAP-TASKTREE-COVERAGE.md](docs/tasks/ROADMAP-TASKTREE-COVERAGE.md) |
| `R1-R5-FOUNDATION-BACKFILL` | `done` | `R1-R5` | — (CLOSED `2026-05-31`: backfilled ownership + meticulous audit for R1–R5 (CLI/IntentIR pivot, SourceIR, EvidenceIR, SemanticIR, IntentIR) — all delivered, tested (source 12 / evidence 120 / semantic 381 / intent 52), book-covered, ROADMAP criteria met; `ROADMAP-TASKTREE-COVERAGE.2`) | [docs/tasks/R1-R5-FOUNDATION-BACKFILL.md](docs/tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| `R8-R13-EXTRACTION-BACKFILL` | `done` | `R8-R13` | — (CLOSED `2026-05-31`: backfilled ownership + meticulous audit for R8–R13 (SourceIR Tier-1 capture, EvidenceIR Tier-2 typed evidence, Tier-3 VLM visual, NLP-L3 enrichment, multi-spec validation, Tier-2 relation extraction) — all delivered + book-covered, ROADMAP criteria met; R9 = Mostly Done with residual robustness-hardening noted; `ROADMAP-TASKTREE-COVERAGE.3`) | [docs/tasks/R8-R13-EXTRACTION-BACKFILL.md](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| `NLP-ENRICH-TRANSPORT-DEDUP` | `done` | `R0` | — (CLOSED `2026-05-31`: completed the text-transport consolidation flagged by `LLM-TEXT-TRANSPORT-DEDUP` — routed `nlp_enrich` through the shared `llm_text::call_text_provider` (2→1 copies, the last duplicate), behavior-preserving via a `max_tokens` param (nlp_enrich keeps 256); removed the duplicated transport + dead `fs`/`Command` imports; 22 nlp_enrich tests pass; corrected a book overclaim re: `enrich`'s image transport; full CI green) | [docs/tasks/NLP-ENRICH-TRANSPORT-DEDUP.md](docs/tasks/NLP-ENRICH-TRANSPORT-DEDUP.md) |
| `REGISTER-MAP-CLASSIFIER-PRECISION` | `done` | `R8` | — (CLOSED `2026-06-01`: fixed a SYSTEMIC `register_map` over-classification found by `CORPUS-HARDENING`. `.3` gated `classify_table_kind` on body register-structure (colon bit-range / access token) → I2C 4→0, APB 1→0, tiling symptom fixed. `.4` added a TOC + data-frame layout guard → clean eMMC re-ingest: register_map 18→2, register_records 48→14, genuine EXT_CSD table kept, I2C/APB unaffected, tiling 0/0. Full CI green; book note in `pipeline/sourceir.md`) | [docs/tasks/REGISTER-MAP-CLASSIFIER-PRECISION.md](docs/tasks/REGISTER-MAP-CLASSIFIER-PRECISION.md) |
| `COMPLETENESS-RECALL-RELATIONS` | `done` (CLOSED) | `R15e` | Both leaves done — per-extractor tagging + the recall gauge now generic over fact kind and cover actor-signal relations too (Pattern @ `evidence` vs LLM @ `signal-resolve`, pre-dedup): `FactKind::ActorSignalRelation` + normalized key + `recall_estimate(_, fact_kind)`; `validate` prints per-kind provenance + dual recall + `recall_estimate_relation_remaining_misses`. CI green (1196 tests), book note added | [docs/tasks/COMPLETENESS-RECALL-RELATIONS.md](docs/tasks/COMPLETENESS-RECALL-RELATIONS.md) |
| `SYMBOL-CLOSURE-CORPUS-VALIDATION` | `done` (CLOSED — no-build) | `R15e` | Corpus-validated the symbol-closure miss detector that `COMPLETENESS-CLOSURE-INVARIANTS.3` had descoped pending "careful corpus validation". Recon over stored SemanticIR (no re-ingest): typed closure is trivially empty (inventory is reference-derived); the real misses are *capture* problems invisible to closure — concretely i2c `SDA` missing from the inventory (90× referenced, prose-only) + inventory noise (`IOL`/`LED`/`START`/`STOP`/`VSS`). Decision: **do not build**; descoping confirmed with evidence. Findings routed to `CORPUS-HARDENING`. Docs-only | [docs/tasks/SYMBOL-CLOSURE-CORPUS-VALIDATION.md](docs/tasks/SYMBOL-CLOSURE-CORPUS-VALIDATION.md) |
| `SIGNAL-TABLE-COLUMNLESS-RECALL` | `done` (CLOSED) | `R12`/`R15e` | Diagnosing the "11 unexplained CHI tables" `CORPUS-HARDENING` follow-up surfaced a CONFIRMED live recall gap: column-less `Signal\|Description` interface tables (CHI REQ/RSP/SNP/DAT channels — `REQFLITPEND`/`REQFLITV`/`REQLCRDV`) yielded **0 declarations** (both stages required direction OR width). User chose **approach A**: `infer_signal_direction_from_description_prose` reads the driver from the description ("the transmitter sets this signal" → output; "the receiver sets…" → input), wired into `synthesize_signal_declarations`. No contract change, nothing fabricated; driver-less signals (REQFLITPEND) stay honest residuals. Reproduction test un-ignored + passing; CI green (1198 tests); book note `pipeline/evidenceir.md` | [docs/tasks/SIGNAL-TABLE-COLUMNLESS-RECALL.md](docs/tasks/SIGNAL-TABLE-COLUMNLESS-RECALL.md) |
| `MEMORY-ARCHITECTURE-DOC` | `done` (CLOSED) | `R0` | Portable, harness-agnostic **durable-memory standard** (`MEMORY_ARCHITECTURE.md`, copyable to other repos) + full in-repo implementation. 4 layers (resume-pointer = demoted `MEMORY.md` 1,395→25 lines · task-trees · `docs/decisions/` 3 migrated ADRs · git) + 4 enforcement gates E1–E4 (bootstrap pointers AGENTS/CLAUDE/.cursorrules/copilot · `scripts/check_memory_architecture.sh` · `.githooks/` via `core.hooksPath` · CI step) + §9.1 agnostic reproduce-anywhere kit. Gates proven to bite; full CI green (memory-arch check first + 1198 tests) | [docs/tasks/MEMORY-ARCHITECTURE-DOC.md](docs/tasks/MEMORY-ARCHITECTURE-DOC.md) |
| `TRACE-SEVERITY-GATING-AUDIT` | `done` (CLOSED — clean) | `R0` | User invariant: severity ≥ warning must never be gated by a verbosity/trace level (a masked error is a silent failure). Exhaustive audit found **zero masking**: no `tracing`/`log` framework or verbosity flag exists; `severity_rank` only sorts findings highest-first + summarizes the worst (never filters); the production `eprintln!("warning: …")` sites (`enrich`/`nlp_enrich`) fire unconditionally on the error path + `main.rs` prints errors unconditionally (ISF dumps are `#[cfg(test)]`); no `Result` silently dropped. Invariant recorded for future trace support in `docs/decisions/0004`. Docs-only | [docs/tasks/TRACE-SEVERITY-GATING-AUDIT.md](docs/tasks/TRACE-SEVERITY-GATING-AUDIT.md) |
| `LITERATURE-GROUNDING` | `done` (CLOSED `2026-06-02`; `.1`–`.13` done) | `R0`/`R15e` | User-directed program: ground every SpecForge aspect in **verified** published research, leveraging prior art while separating genuine out-of-the-box novelty. **All 11 aspects grounded** (`docs/research/grounding/`): document extraction, staged IR, requirements extraction, protocol & temporal semantics, KG/relation extraction, multimodal fusion, neuro-symbolic/bounded-LLM, cross-document learning, extraction evaluation, uncertainty/residual-honesty, spec→hardware. `.13` synthesis (`README.md`: 11-aspect→literature map + cross-aspect novelty throughline + 3-tier reach-full-potential backlog → candidate future trees) + book mirror in `architecture-rationale.md`. **Anti-hallucination held: every citation resolvable; high-risk recent works spot-verified; 1 unverifiable survey dropped.** | [docs/tasks/LITERATURE-GROUNDING.md](docs/tasks/LITERATURE-GROUNDING.md) |
| `NLI-GATE-METRIC` | `done` (CLOSED `2026-06-05`; `.1`) | `R16`/`R15e` | **User directive ("→ surface an nli_* count"):** `nli_demoted_count(&[ResidualDecisionPacket])` (counts `nli_unentailed_` packets; read-only, no LLM) + `metric("nli_demoted_contracts", …)` in `validate_intent_ir` → `specforge validate <intent_ir.json>` surfaces how many contracts the NLI gate demoted. +1 test; book mention; CI green 1254 | [docs/tasks/NLI-GATE-METRIC.md](docs/tasks/NLI-GATE-METRIC.md) |
| `NLI-INTENT-GATE` | `done` (CLOSED `2026-06-05`; `.1`–`.2`) | `R16`/`R15e` | **User pick "→ (b)":** the NLI verifier as an *active* IntentIR gate. A **post-build pass** on `IntentIr` (residuals exist there, unlike EvidenceIR): `obligation_claim_text` (phrasable-only else abstain) renders each contract → claim, NLI-verifies vs `provenance.source_text`, and **demotes** NotEntailed contracts into `residual_decisions` (preserved, never deleted — a verifier error costs a review item, not a lost fact). `nli_gate_contracts` (verifier **injected** → hermetic) + `apply_nli_gate(&mut IntentIr)`; opt-in `intent --nli-verify`. CI green 1253; book + KM in sync | [docs/tasks/NLI-INTENT-GATE.md](docs/tasks/NLI-INTENT-GATE.md) |
| `NLI-ENTAILMENT-VERIFIER` | `done` (CLOSED `2026-06-05`; `.1`–`.3`) | `R16`/`R15e` | **User directive ("→ NLI entailment verifier"),** the culmination of the text-model thread. A *semantic* grounding gate (premise = source statement, hypothesis = extracted claim → keep only ENTAILED), replacing string-match grounding's blind spots; grounded in SNLI/Bowman + hallucination-mitigation. Built on the **validated** `qwen2.5:14b-instruct` (text, 5/6 NLI; the entailment framing beats free-form labeling). `.2` = pure module `ir/nli_verify.rs` (`NliVerdict` + entailment prompt + fail-closed parse + `verify_entailment` reusing `call_text_provider` + the `SPECFORGE_VLM_HELPER` hermetic hook + the **additive** gate where `Unknown`→Abstain so an outage never breaks extraction; NotEntailed→residual); `.3` = live wiring + metric. No CI test depends on Ollama | [docs/tasks/NLI-ENTAILMENT-VERIFIER.md](docs/tasks/NLI-ENTAILMENT-VERIFIER.md) |
| `EVAL-GOLD-INTERANNOTATOR-AGREEMENT` | `done` (CLOSED `2026-06-05`) | `R15e` | **Is the eval answer-key trustworthy?** The constraint gold (`seed_apb.json`) is single-source (agent-drafted) — never checked for reliability. Had an **independent blind agent** re-annotate the 8 statements (text + signals only, no gold) and scored **Cohen's κ over the 18 (statement,signal) units**. **κ = 0.90** (17/18 = 94%, almost-perfect, Landis-Koch) → **gold reliable**. Lone disagreement (`0202`/PENABLE, the "any other control signals" clause) = a documented interpretive ambiguity, not an error. KM card `eval-gold-interannotator-kappa`; grounded in Cohen/Krippendorff/Artstein-Poesio. Cross-model (Ollama qwen) 2nd-rater + relation-task agreement = follow-ups | [docs/tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md](docs/tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md) |
| `FSMGEN-ASSERT-LOWERING` | `done` (CLOSED `2026-06-04`; `.1`–`.3`) | `R6` | **FSMGen shipped both deltas** — `(stable/changed/rose/fell)` (`6700fbb4`) + `(within B MIN MAX)` → `##[MIN:MAX]` (`92d7036b`, `1<=MIN<=MAX` locked per our answer). `.1` re-pin `43b29f5c → 92d7036b`. `.2` **verified-negative**: `(stable s)` lowering would over-assert (phase-scoped `Obligation::Stable { during: Between{tick_phases} }` ≠ unconditional per-tick) → residual correct (KM `stable-obligation-phase-scoped-residual`). `.3` **faithful fix**: a *guarded* windowed-eventual now keeps its antecedent — `(assert (=> g (within s [min] max)))` (the bare monitor dropped it; `min>1` → two-operand window). `IsfContract`/`Contract` carry a pre-built `prop`; both classify paths updated (parity oracle green); 4 tests incl. fsmgen-binary strict-check; CI green 1243 | [docs/tasks/FSMGEN-ASSERT-LOWERING.md](docs/tasks/FSMGEN-ASSERT-LOWERING.md) | [docs/tasks/FSMGEN-ASSERT-LOWERING.md](docs/tasks/FSMGEN-ASSERT-LOWERING.md) | [docs/tasks/FSMGEN-ASSERT-LOWERING.md](docs/tasks/FSMGEN-ASSERT-LOWERING.md) |
| `FSMGEN-MIN-WINDOW-CONFIRM` | `done` (CLOSED `2026-06-04`) | `R6` | **Answered FSMGen's gating question** for the proposed `min > 1` window slice (`(within B MIN MAX)` → `##[MIN:MAX]`). Verified our `cycle_window` bounds are always **integer literals** (`Option<u32>`, no symbolic form); `MIN=0` occurs only as the same-cycle `[0,0]` (already a residual) or a `0`-to-`N` range (= the anchored `(monitor (within S N))` form). SpecForge **guarantees `MIN >= 1`** for the `(within B MIN MAX)` consequent. Recommended FSMGen lock `1 <= MIN <= MAX`. Answer filed in `docs/FSMGEN_FEEDBACK.md`; no code / no re-pin (integration is a future tree once `min>1` ships) | [docs/tasks/FSMGEN-MIN-WINDOW-CONFIRM.md](docs/tasks/FSMGEN-MIN-WINDOW-CONFIRM.md) |
| `FSMGEN-ASSERT-MIGRATE` | `done` (CLOSED `2026-06-04`; `.1`–`.2`) | `R6` | **Consequence of FSMGen's response to the LTL/MTL suggestion.** FSMGen answered: yes, already shipped — the `(contract … (eventually …))` clause was **removed** and generalized into the `(assert/assume/cover …)` verification family (their decisions 0008/0009). Re-pinned `subs/fsmgen` `c0b7eaa7 → 43b29f5c` and migrated SpecForge's bounded-eventually emission `(contract … (eventually s (within N)))` → `(assert (monitor (within s N)))` (empirically strict-valid; `IsfContract.name` dropped). Full CI GREEN **on the new pin** (1239; fsmgen-binary strict-check re-validated). `(stage …)` unchanged; `stable`/`min>1` stay residual pending FSMGen primitives (user requesting). Retired `TEMPORAL-RULE-SVA-RENDER` (superseded); book + KM card `fsmgen-temporal-isf-form` | [docs/tasks/FSMGEN-ASSERT-MIGRATE.md](docs/tasks/FSMGEN-ASSERT-MIGRATE.md) |
| `DEMPSTER-FUSION-COMBINER` | `done` (CLOSED `2026-06-04`; `.1`–`.2`) | `R16`/`R15e` | **User pick (2nd, after prior-decay), delivered.** Grounded gap (`multimodal-fusion.md`, Dempster 1967): `ir/fusion.rs::merge_cluster` combined agreeing contracts' confidence with `min_confidence` (capped at the weakest). Replaced on the agreement path with a **Dempster corroboration** (`mass: High .9/Med .7/Low .5`, combined `m = 1−∏(1−mᵢ)`, mapped back) so independent agreement *raises* confidence — **Medium+Medium → High**, Low+Low → Medium, High caps, single source unchanged. Disagreement still keeps the conservative `min` + routes to a Residual. 6 tests, book subsection, KM card `dempster-fusion`. Conflict mass K=0 here (disagreement pre-split) → Zadeh guard deferred. Churn = one updated unit test; CI green 1239 | [docs/tasks/DEMPSTER-FUSION-COMBINER.md](docs/tasks/DEMPSTER-FUSION-COMBINER.md) |
| `PRIOR-DECAY` | `done` (CLOSED `2026-06-04`; `.1`–`.2`) | `R15c`/`R15e` | **User pick (prior-decay → Dempster), delivered.** Grounded gap (`cross-document-learning.md`, Parisi arXiv:1802.07569): `CorpusMemory` priors **only accrete** — a later doc contradicting an earlier prior was never noticed. Shipped read-only `CorpusMemory::contested_priors()` flagging same-key/different-value priors *within a protocol family* (ActorTaxonomy/SemanticPhrase/TableShape) with competing values + supports + the strongest-supported value as an advisory hint; surfaced in `learn-priors` (`contested_priors:`). 4 tests, book subsection, KM card `contested-priors`. **Additive — no harvest/merge/consultation change** (zero fixture/behavior risk); CI green 1233. Consultation down-weighting + time-staleness deferred | [docs/tasks/PRIOR-DECAY.md](docs/tasks/PRIOR-DECAY.md) |
| `FSMGEN-LTL-MTL-SUGGESTION` | `done` (CLOSED `2026-06-04`) | `R6` | **User directive.** Filed a suggestion in the SpecForge→FSMGen feedback channel (`docs/FSMGEN_FEEDBACK.md`) that **ISF gain first-class LTL/MTL temporal properties** — the full `G(antecedent → X/F[min,max] consequent)` template (generalizing the existing `(eventually s (within N))`) — so SpecForge could lower `temporal_rules` *directly into ISF*. Includes a concrete **proposed ISF shape** (`(temporal-rule (clock…(edge…)) (antecedent <pred>…) (consequent (window min max) <pred>…))`, 1:1 with `TemporalRuleRecord`). Suggestion, not a bug; KM card `fsmgen-feedback-channel` added. SpecForge action complete — decision is FSMGen's | [docs/tasks/FSMGEN-LTL-MTL-SUGGESTION.md](docs/tasks/FSMGEN-LTL-MTL-SUGGESTION.md) |
| `TEMPORAL-RULE-SVA-RENDER` | `superseded` (CLOSED-as-superseded `2026-06-04`; `.1` design kept, `.2` never built) | `R6`/`R15e` | **Decision resolved → FSMGen-native.** The SpecForge-side `.isf`/IntentIR → SVA export was logged-deferred pending the SVA-vs-FSMGen-native choice. FSMGen's response (verification family already ships the full template; SpecForge now lowers temporal rules directly into ISF via `FSMGEN-ASSERT-MIGRATE`) makes a separate SVA export **unnecessary** — retired. `.1` SVA-mapping design kept as a record | [docs/tasks/TEMPORAL-RULE-SVA-RENDER.md](docs/tasks/TEMPORAL-RULE-SVA-RENDER.md) |
| `SPEC-MINING-PROVENANCE` | `done` (CLOSED `2026-06-04`; `.1`–`.3` done) | `R0`/`R15e` | **User directive (2026-06-04), delivered.** (1) **"forward specification mining"** adopted as SpecForge's discipline name (Ammons POPL'02; forward: spec→intent vs the literature's backward implementation→spec) — `README.md` + `architecture-rationale.md` + KM card `spec-mining-framing`. (2) A complete per-author **adopt/defer provenance ledger** (`docs/research/grounding/adopt-defer-ledger.md`): every leveraged author/work → *Take* / *Leave-out + why* / *Instantiated-at*, + a cross-cutting synthesis + KM card `adopt-defer-ledger`. Covers the temporal trio + ~10 other clusters (doc-AI, KG/RE, multimodal, neuro-symbolic, cross-doc learning, eval, uncertainty, requirements, staged-IR, PSL/SVA). Deferred-adopt items flagged as future trees. Verify-every-citation; advisory; KM 7 facts/36 keys; CI green 1229 | [docs/tasks/SPEC-MINING-PROVENANCE.md](docs/tasks/SPEC-MINING-PROVENANCE.md) |
| `AMBIGUITY-PHRASE-DETECTOR` | `done` (CLOSED `2026-06-04`; `.1`–`.2` done) | `R8`/`R15e` | Grounded gap from the `LITERATURE-GROUNDING` backlog (`requirements-extraction.md`; NASA ARM ICSE'97 / Berry-Kamsties), delivered. A **flag-only** detector (`ir/ambiguity.rs`) scans EvidenceIR statement prose for vagueness / under-specification markers (NASA ARM weak phrases + chip-spec "implementation-defined"/"vendor-specific"; modal verbs excluded) and surfaces them in `validate` (Info finding `evidence_ambiguous_statements` + `ambiguous_statements` metric). Residual-honesty; extraction-neutral. 6 tests, book subsection, KM card; CI green 1229. Typed-residual routing of flagged statements deferred | [docs/tasks/AMBIGUITY-PHRASE-DETECTOR.md](docs/tasks/AMBIGUITY-PHRASE-DETECTOR.md) |
| `TEMPORAL-RULE-LTL-RENDER` | `done` (CLOSED `2026-06-04`; `.1`–`.2` done) | `R6`/`R15e` | **Tier-1 headline adopt item delivered from the `LITERATURE-GROUNDING` backlog** (`protocol-temporal-semantics.md`). Mined `temporal_rules` now render in **standard LTL/MTL notation** — `G(antecedent → consequent)` with `X`/`F[min,max]` — grounding SpecForge's temporal model in the spec-mining formalism (Pnueli/GoldMine/Texada), via a pure `ir/temporal_ltl.rs` renderer + atom vocabulary for all 7 predicate variants + 4 exact-string tests + a book subsection + a KM card. **Derived/no-IR-field** (zero fixture churn, zero behavior change). The `--ltl` flag was dropped (10+ `ValidateArgs` sites); the renderer is public API for the downstream `.isf`→PSL/SVA **export** tree (gated on the FSMGen contract). Worked: `G( PSEL==ASSERTED & PENABLE==ASSERTED & PREADY==ASSERTED -> X (drive(Completer,PBUSER) & PBUSER==VALID) )`; CI green 1223 | [docs/tasks/TEMPORAL-RULE-LTL-RENDER.md](docs/tasks/TEMPORAL-RULE-LTL-RENDER.md) |
| `KNOWLEDGE-MAP-ADOPTION` | `done` (CLOSED `2026-06-04`; `.1`–`.2` done) | `R0` | Adopted the user's portable **`KNOWLEDGE_MAP_ARCHITECTURE.md`** bundle (`knowledge-map/`) in SpecForge — an **additive, derived, question-keyed retrieval layer** that makes *archaeology* (re-deriving an already-logged fact) structurally impossible. Composes with `MEMORY_ARCHITECTURE.md` + `docs/decisions/` + task-trees; **replaces nothing, converts nothing**. Bundle copied verbatim; `KNOWLEDGE_MAP.md` derived (3 facts / 15 question keys) from front-mattered cards under `docs/knowledge/`; KM gate wired beside the memory-arch gate (`.githooks/pre-commit` + `run_ci.sh`); registered in every bootstrap surface; 3 seed cards (docling-cpu, llm-provider-default, temporal-eval-stale-FPs). Negative-tested; CI green 1219. Grows lazily — one card per durable fact / caught archaeology | [docs/tasks/KNOWLEDGE-MAP-ADOPTION.md](docs/tasks/KNOWLEDGE-MAP-ADOPTION.md) |
| `RECALL-CHAO-ESTIMATOR` | `done` (CLOSED `2026-06-02`; `.1`–`.2` done) | `R15e` | **Tier-2 adopt item delivered from the `LITERATURE-GROUNDING` backlog** (`extraction-evaluation.md`). Added the **Chao (1987) heterogeneity-robust** richness estimator as a second recall N̂ alongside Lincoln–Petersen in `ir/completeness.rs::recall_estimate` — the two tiers share prose (positive dependence → LP under-estimates) and Chao tolerates unequal catchability. 2-source incidence: `f1=distinct−overlap`, `f2=overlap`, `N̂_chao=distinct+f1²/(2·f2)` (worked: Chao 10 vs LP 8). `validate` now reports the LP/Chao range + 2 Chao metrics; user-friendly book subsection in `pipeline/evidenceir.md`. Additive (`None`-gating unchanged), verified citation (DOI 10.2307/2531532), zero behavior change; CI green 1219 | [docs/tasks/RECALL-CHAO-ESTIMATOR.md](docs/tasks/RECALL-CHAO-ESTIMATOR.md) |
| `TEMPORAL-ANTECEDENT-RECALL` | `done` (CLOSED `2026-06-02`; `.1`–`.2` done) | `R8`/`R15e` | **The second completed cycle of the measure→catch→fix loop** (after `CONSTRAINT-SUBJECT-PRECISION`), surfaced by `TEMPORAL-RULE-EVAL`. The temporal condition parser (`parse_temporal_condition_predicates`, `ir/semantic.rs`) dropped leading signals in a coordinated list (*"PBUSER valid when PSEL, PENABLE, **and** PREADY are asserted"* → antecedent only PREADY). Fix = distribute a single shared trailing value across the coordinated signal list (tightly guarded: ≥2 signals, one distinct value, fill only value-less clauses; no fabrication). **Eval re-run on real APB confirmed it: P 0.400→0.600, R 0.667→1.000, F1 0.500→0.750.** +2 unit tests; book refreshed; CI green 1218, no kg-bench regression. The FP cohort (degenerate header / `*_WIDTH` subject) is upstream constraint-tier + stale-artifact, re-ingest-gated follow-up | [docs/tasks/TEMPORAL-ANTECEDENT-RECALL.md](docs/tasks/TEMPORAL-ANTECEDENT-RECALL.md) |
| `TEMPORAL-RULE-EVAL` | `done` (CLOSED `2026-06-02`; `.1`–`.4` done) | `R15d`/`R15e` | **First Tier-1 item delivered from the `LITERATURE-GROUNDING` reach-full-potential backlog.** Supervised precision/recall/F1 for **mined temporal rules** — SpecForge's previously-unmeasured third extraction surface (after `signal_constraint` + `actor_signal_relation`). Reuses `eval.rs`'s closed-world scorer with a new **semantic, provenance-free** canonical key over `TemporalRuleRecord` (edge + sorted antecedent/consequent predicate-keys + cycle_window); producer = the deterministic temporal parser (EvidenceIR→SemanticIR, built on a temp copy → corpus untouched). 6-item hand-labeled APB seed. **Live end-to-end: `temporal_rule P=0.400 R=0.667 F1=0.500` — surfacing a real antecedent-under-capture + degenerate-header finding** (a candidate fix-tree; measure→catch→fix loop continuing past `CONSTRAINT-SUBJECT-PRECISION`). Pure additive measurement; zero extraction-behavior change | [docs/tasks/TEMPORAL-RULE-EVAL.md](docs/tasks/TEMPORAL-RULE-EVAL.md) |
| `CONSTRAINT-SUBJECT-PRECISION` | `done` (CLOSED) | `R8`/`R15e` | **The `LLM-EXTRACTION-EVAL` harness's first catch — fixed.** The eval scored APB `signal_constraint` at P=0.500 (6TP/6FP); the FPs were real over-extraction by the constraint tier in 3 classes (condition-clause signals from "until…/if…"; a `*_WIDTH` parameter; clock/cross-sentence signals swept into a stability clause). 3 minimal fixes in `ir/evidence.rs` — `until`/`if` + earliest-marker condition stripping; `*_WIDTH` exclusion; narrowing to the constraint-verb sentence (with fallback) — locked by 3 per-class regression tests reproducing the eval's exact FP statements. Full suite 1211/0; book note `pipeline/evidenceir.md` (aggregate eval re-confirm runnable on a clean APB re-ingest) | [docs/tasks/CONSTRAINT-SUBJECT-PRECISION.md](docs/tasks/CONSTRAINT-SUBJECT-PRECISION.md) |
| `LLM-EXTRACTION-EVAL` | `done` (CLOSED) | `R15d`/`R16` | The supervised measurement piece for the LLM passes (was missing — `kg-bench` tests the deterministic pipeline; the recall gauge is unsupervised). New `eval-extraction` command scores `nlp-enrich`/`signal-resolve` against a labeled APB seed (precision/recall/F1): `.2` pure scorer (`eval.rs`) · `.3` 16-item seed (gold drafted from prose) · `.4` runner (temp-redirect over the real command path) · `.5` live qwen2.5vl baseline (honest: no improvement on this small seed; slightly hurt relation precision) + book `quality/extraction-eval.md`. The qwen2.5-vs-qwen3-vl:8b A/B is now one command. CI green (1208 tests) | [docs/tasks/LLM-EXTRACTION-EVAL.md](docs/tasks/LLM-EXTRACTION-EVAL.md) | Building the missing measurement piece: a labeled **precision/recall/F1** eval for the LLM extraction passes (`nlp-enrich`→SignalConstraint, `signal-resolve`→relations, `extract-contracts`→ActorContract), so model A/Bs (qwen2.5vl vs qwen3-vl, Instruct vs Thinking) are rigorous not eyeballed. `kg-bench` (151 fixtures) tests the deterministic pipeline only; the recall gauge is unsupervised — this adds supervised gold. Design: faithful runner over the real command path, closed-world scoring on labeled statements, agent-drafted+review-flagged labels. v1 = 3 text tasks (VLM enrich deferred). Leaves: scorer → labeled data → runner → baseline | [docs/tasks/LLM-EXTRACTION-EVAL.md](docs/tasks/LLM-EXTRACTION-EVAL.md) |
| `DOCLING-DEVICE-CPU-DEFAULT` | `done` (CLOSED) | `R12` | Fixed PDF ingest failing on Apple Silicon after a fresh Docling install: torch 2.12 defaults to the MPS backend, which can't do the float64 ops Docling's models need, so every page errored. The embedded helper now selects the device deliberately — honor `DOCLING_DEVICE`, else CUDA-if-present else CPU, never auto→MPS — with a defensive fallback. Verified end-to-end (fresh no-env ingest converts, 0 MPS errors); CI green (1198); book note `pipeline/sourceir.md`. No env var needed going forward | [docs/tasks/DOCLING-DEVICE-CPU-DEFAULT.md](docs/tasks/DOCLING-DEVICE-CPU-DEFAULT.md) |
| `REGISTER-CLASSIFIER-ENCODING-FP` | `done` (CLOSED) | `R12`/`R15e` | CLOSED `2026-06-01`: the register-tiling detector flagged a CHI register with self-contradictory bits — root-caused to `classify_table_kind` mis-typing DVM **field-encoding** cross-reference tables (`table_0170`/`0171`, "… Security field encodings …" + caption-less continuation) as register maps, emitting 8 phantom bit-range-named registers. Fix = encoding cross-reference positive (caption `"encoding"` ∨ header `\bx in\b` idiom → `encoding`, before the register gate). Corpus harness over 1,989 tables: 2 CHI tables `register_map→encoding` (8 phantoms gone, register_map 9→7), signal_description 210→210, +32 true encoding corrections, 0 regression. CI green; book note `pipeline/sourceir.md`. (Sibling of `REGISTER-MAP-CLASSIFIER-PRECISION`) | [docs/tasks/REGISTER-CLASSIFIER-ENCODING-FP.md](docs/tasks/REGISTER-CLASSIFIER-ENCODING-FP.md) |
| `COMPLETENESS-RECALL-GAUGE` | `done` | `R15e` | — (CLOSED `2026-06-01`: capture–recapture recall gauge live — `signal_constraint_recall_estimate` (`ir/completeness.rs`, 2-extractor Lincoln–Petersen over `fact_provenance`) → estimated remaining misses as an honest LOWER BOUND, gated on ≥2 tiers + overlap (None/"insufficient" otherwise, never fabricated), surfaced in `validate` with assumptions printed. 3 tests; CI green; book note. Research→impl recall-estimation arc complete; Chao-Mh/3rd-extractor = future upgrade) | [docs/tasks/COMPLETENESS-RECALL-GAUGE.md](docs/tasks/COMPLETENESS-RECALL-GAUGE.md) |
| `PER-EXTRACTOR-FACT-TAGGING` | `done` | `R15e` | — (CLOSED `2026-06-01`: the capture–recapture recall-gauge precondition (research `.5` §3) — `EvidenceIr.fact_provenance` index (`ExtractorTier`/`FactKind`/`FactProvenanceRecord` + canonical key in `ir/evidence.rs`) records which extractor found each signal constraint: Pattern@build + Nlp@`nlp-enrich` (pre-dedup, overlap-capturing), surfaced in `validate`. Extraction-neutral (no 140-site field change); 3 tests; CI green; book note. Recall gauge = next owned tree) | [docs/tasks/PER-EXTRACTOR-FACT-TAGGING.md](docs/tasks/PER-EXTRACTOR-FACT-TAGGING.md) |
| `COMPLETENESS-REPORT-SURFACE` | `done` | `R15e` | — (CLOSED `2026-06-01`: `validate` now emits one `Completeness Summary` headline — `candidate_misses` = register overlaps + interior gaps + unexplained tables + prose residuals, with breakdown + anchored-rescan convergence — + Info `evidence_completeness_summary` finding + `completeness_candidate_misses` metric; pure aggregation (behavior-neutral); aggregate==sum wiring test; CI green; book note. Framework §10 headline realized) | [docs/tasks/COMPLETENESS-REPORT-SURFACE.md](docs/tasks/COMPLETENESS-REPORT-SURFACE.md) |
| `COMPLETENESS-REGION-ACCOUNTING` | `done` | `R15e` (x-cut R15c) | — (CLOSED `2026-06-01`: first slice of the region-accounting miss detector (research `.3`) live — `unexplained_intent_bearing_tables` in `ir/completeness.rs` flags any SignalDescription/RegisterMap/TimingParameter table that produced 0 records (exact via verified table→record links), surfaced in `validate` as a Warning + `region_unexplained_tables` metric; extraction-neutral; 7 tests; CI green; book note in `pipeline/evidenceir.md`. Prose/figure regions + unified report = future slices) | [docs/tasks/COMPLETENESS-REGION-ACCOUNTING.md](docs/tasks/COMPLETENESS-REGION-ACCOUNTING.md) |
| `CORPUS-HARDENING` | `active` (standing; `.1`–`.4` complete) | `R12`/`R15e` | standing hardening harness over the real chip-doc corpus (82 PDFs). DONE: `.1` harness proven (I2C/eMMC); `.2` full AMBA core run (APB/AHB/AXI/CHI) — all 4 completeness surfaces validated on real data, register classifier FP fixed; `.3` VLM+NLP-in-the-loop recall pass (recovered 6 timing+1 state+5 constraints+16 decls). Future specs/families = new leaves. Follow-up candidates: ~~1 CHI register overlap~~ RESOLVED (`REGISTER-CLASSIFIER-ENCODING-FP` — encoding table FP, 8 phantom registers killed); 11 unexplained CHI tables still open | [docs/tasks/CORPUS-HARDENING.md](docs/tasks/CORPUS-HARDENING.md) |
| `COMPLETENESS-CLOSURE-INVARIANTS` | `done` | `R15d` | — (CLOSED `2026-06-01`: first completeness detector from the research — register bit-tiling (`ir/completeness.rs`, overlap + interior gap, no width speculation) surfaced in `validate`; corpus-validated precise (1 TP + 0 FP/84) + book-documented (`.4`). `.3` symbol closure DESCOPED to a future careful, corpus-validated tree — genuinely hard (missed-decl vs external indistinguishable; would be noise if rushed)) | [docs/tasks/COMPLETENESS-CLOSURE-INVARIANTS.md](docs/tasks/COMPLETENESS-CLOSURE-INVARIANTS.md) |
| `INTENT-COMPLETENESS-RESEARCH` | `done` | `R15e` (cross-cutting R15c/d/R16) | — (CLOSED `2026-06-01`: research program `.1`–`.7` complete — framework + closed ontology/coverage matrix + region-accounting design + miss-detector catalog + verified literature grounding + recall-estimator/typed-CompletenessReport design + prioritized backlog. Drove the implementation phase: 4 live corpus-validated `validate` completeness surfaces [convergence, register-tiling, region-accounting, completeness-summary] + the register-classifier precision fix. Artifacts: `docs/research/*.md` ×6) | [docs/tasks/INTENT-COMPLETENESS-RESEARCH.md](docs/tasks/INTENT-COMPLETENESS-RESEARCH.md) |
| `R15C-CONVERGENCE-REPORT` | `done` | `R15c` | — (CLOSED `2026-05-31`: made the EvidenceIR anchored-rescan loop first-class/inspectable — emit a typed `EvidenceConvergenceReport` (passes, genuinely-new deduped facts/pass, total, converged-vs-capped), persisted on EvidenceIR + surfaced by `validate` (Info converged / Warning cap-limited) with 3 metrics; extraction outcomes unchanged; 3 unit tests; full CI green; book subsection. Advances R15c "convergence reporting counts genuinely new persisted facts" under `R15C-R15G-LEARNING-PLANE-BACKFILL.1`) | [docs/tasks/R15C-CONVERGENCE-REPORT.md](docs/tasks/R15C-CONVERGENCE-REPORT.md) |
| `R15C-R15G-LEARNING-PLANE-BACKFILL` | `active` | `R15c-R15g` | per-leaf `in_progress` — backfilled ownership + audit for the OPEN learning/eval/corpus lanes R15c–g (KG-guided multimodal rescans, cross-modality conflict arbitration, KG-quality benchmark hardening, cross-document learning plane, corpus knowledge base); each leaf records the audited delivered surface (semantic+conflict surfaces; `kg-bench`+153 fixtures; `learn-priors`/`CorpusMemory`; `corpus-kb`/11 families) AND remaining open scope; advancing any remaining scope = a future owned tree (`ROADMAP-TASKTREE-COVERAGE.4`) | [docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md](docs/tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |

## Directory Layout

```text
docs/TASK_TREE.md
docs/tasks/
  TEMPLATE.md
  <TREE>.md
```

`docs/TASK_TREE.md` is the workflow and active-tree index.
Each top-level task owns one file in `docs/tasks/`.
`docs/tasks/TEMPLATE.md` is copied when creating a new top-level tree.

## Definitions

- Task tree: the recursive decomposition of one top-level task.
- Node: one item in that tree.
- Container node: a node with children. It is not directly executable.
- Leaf node: a node with no children. It is the only unit PNT may implement.
- Current frontier: the ordered set of leaf nodes that are eligible to be
  picked next.
- Slice: one completed leaf task plus its tests, docs, live-doc updates, and
  commit workflow.
- Evidence: the validation output, changed-doc summary, and git commit subject
  that prove a leaf was completed.

## ID Rules

Each task tree has a stable top-level ID.

```text
<TREE>
<TREE>.1
<TREE>.1.1
<TREE>.1.1.1
```

Rules:

- `<TREE>` uses uppercase letters, digits, and hyphens.
- Child IDs append dot-separated positive integers.
- IDs are permanent once published.
- Never renumber closed nodes.
- If a new ordering is needed, add new IDs and mark old nodes `superseded` or
  `deferred` with a reason.
- A commit that completes a task-tree leaf must identify the leaf ID in the
  commit subject or in the first body line.

## Status Vocabulary

Use only these statuses.

| Status | Meaning |
| --- | --- |
| `proposed` | Captured but not yet accepted into the active tree. |
| `active` | The top-level tree is open, or a container has unfinished children. |
| `pending` | Ready to be selected once it reaches the current frontier. |
| `in_progress` | Currently being implemented in the worktree. |
| `blocked` | Cannot proceed without a named blocker and unblock condition. |
| `done` | Completed, validated, documented, and committed. |
| `deferred` | Deliberately postponed with an explicit consequence. |
| `superseded` | Replaced by another node, with the replacement ID named. |

## Required Task File Sections

Every top-level task file must contain:

- Metadata: tree ID, status, roadmap lane, created date, last updated date.
- Goal: the user-visible or project-visible outcome.
- Non-goals: what this tree deliberately does not try to solve.
- Acceptance criteria: concrete conditions that close the top-level task.
- Task tree: all known nodes, with status and short result intent.
- Current frontier: ordered leaf nodes that PNT may select next.
- Decisions: accepted technical decisions and their rationale.
- Open questions: unresolved questions that do not block the whole tree yet.
- Blockers: blockers with unblock conditions.
- Verification log: checks run for completed leaves.
- Commit log: leaf IDs mapped to completion commit subjects.
- Changelog: dated edits to the tree itself.

## Node Rules

Every node must be one of these two shapes.

Container node:

```text
- ID: <TREE>.<n>
  Status: active
  Goal: ...
  Children: <TREE>.<n>.1, <TREE>.<n>.2
```

Leaf node:

```text
- ID: <TREE>.<n>
  Status: pending
  Goal: ...
  Acceptance: ...
  Verification: pending
  Commit: pending
```

A node with children must not be marked `done` until every child is `done`,
`deferred`, or `superseded`, and every non-`done` child has a recorded reason.

## Current Frontier Rules

The current frontier is the only list PNT uses when selecting work from a task
tree.

Rules:

- The frontier contains only leaf nodes.
- The frontier is ordered by intended priority.
- A container never appears in the frontier.
- A blocked node stays out of the frontier until unblocked.
- When a leaf is split, remove that leaf from the frontier, mark it `active`,
  add children, and place the first executable child or children in the
  frontier.
- When a leaf completes, remove it from the frontier and add the next eligible
  leaf or leaves.

## PNT Selection Rules

When PNT is asked to continue and at least one active task tree exists:

1. Read `docs/TASK_TREE.md`.
2. Read the active task file named in the `Active Task Trees` table.
3. Pick the first eligible leaf in that file's `Current Frontier`.
4. Implement only that leaf.
5. If the leaf is too broad, split it before implementation and commit the
   tree update as the leaf's honest outcome.
6. Run the required validation for the leaf.
7. Update the task file, live docs, and roadmap if status changed.
8. Run the full commit workflow before selecting another leaf.

If several active trees exist, choose the first active tree in the table unless
the user names another tree or the roadmap status names a different immediate
lane.

## Splitting Rules

Split a node when any of these are true:

- It cannot be completed to signoff quality in one slice.
- It mixes design, implementation, diagnostics, tests, and docs in ways that
  can be reviewed independently.
- It hides an unresolved policy choice behind implementation wording.
- It would require touching unrelated ownership areas in one commit.
- It discovers a lower-level dependency that should be solved first.

Do not split merely to create vague placeholders. Every child must have a
clear goal and a way to verify completion.

## Completion Rules

A leaf is complete only when all of the following are true:

- Implementation or documentation work for that leaf is finished.
- Focused checks passed, and broader checks ran when warranted.
- The owning task file records the result, validation, and commit subject.
- `MEMORY.md`, `CHANGES.md`, `DEVELOPMENT_NOTES.md`,
  `LIVE_ACHIEVEMENT_STATUS.md`, and `ROADMAP.md` are updated when the
  leaf changes project state.
- **Book method-doc close-rule** (`BOOK-METHOD-DOC`): a tree's
  closing leaf (`.N`) MUST add or refresh that tree's
  implementation+verification subsection in the topically-correct
  mdBook chapter (per the placement map in
  `docs/tasks/BOOK-METHOD-DOC.md`). The subsection is the
  human-facing "how + why + how-verified"; the task-tree file
  remains the machine-tracked authority. A close leaf whose book
  section is missing or stale is incomplete — the book / ROADMAP
  language always describes what the code does (the
  `AUDIT-DOC-RECONCILE` doctrine).
- The commit workflow in `COMMIT.md` has completed.
- `git_message_brief.txt` has been cleared after commit.

Commit hashes are intentionally not required inside the same task-file update:
the final hash cannot be known until after the commit exists. The stable
join key is the leaf ID in the commit subject or first body line. Later status
refreshes may backfill hashes if useful.

## Blocker Rules

A blocked node must record:

- the exact blocker,
- why it blocks the node,
- the unblock condition,
- and the next task that should run instead, if any.

Do not leave a node as `blocked` only because it is large or unclear. Large or
unclear work should be split until a real blocker is visible.

## Relationship To Live Docs

The task tree is the detailed execution ledger.

- `ROADMAP.md` remains the canonical high-level workstream status.
- `MEMORY.md` remains the recovery/handoff continuity log.
- `CHANGES.md` remains the chronological technical history.
- `DEVELOPMENT_NOTES.md` remains design rationale.
- `LIVE_ACHIEVEMENT_STATUS.md` remains the latest completed slice summary.
- The mdBook remains user-facing product/language documentation.

Do not duplicate the whole task tree into those files. Link to the task tree
and summarize only the part that changes live project state.

## SpecForge-Specific Defaults

- All R6 `.fsm` adapter hardening and provenance-field hardening work is
  task-tree-managed by default.
- Active PNT cycles select from the first active tree's current frontier
  unless the user names a different lane.
- The canonical validation command is `cargo test -p specforge --lib`.
- Commit messages for task-tree-managed leaves must include the leaf ID
  (e.g. `PROV-HARDEN.4`) in the commit subject or first body line.
