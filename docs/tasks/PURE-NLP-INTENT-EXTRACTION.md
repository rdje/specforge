# PURE-NLP-INTENT-EXTRACTION: model-based intent extraction (ACTIVE — first increment)

## Metadata

- Tree ID: `PURE-NLP-INTENT-EXTRACTION`
- Status: **`active`** — UN-PARKED `2026-06-05` (owner directive "do all 5 bullets" + the activation
  gate met: `REEXTRACTION-REMEASURE` validated the grammar engine on the real APB spec — constraint
  over-generation 42→16, 69 relations, broad corpus verb coverage).
- Activation gate: ~~only after SpecForge is proven on grammar~~ — **MET** (see above).
- Roadmap lane: `R17+` (post-completion enhancement)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: owner decision (2026-06-05). Today's extraction reads specs with **English
  normative/structural grammar** (`must`/`shall`/`when`) plus document-derived names (per ADR
  `0006`, `[[feedback-no-hardcoded-chip-spec-names]]`). The owner wants to **prove the grammar
  approach works first**; a pure-NLP-model design is a *later* option to make SpecForge more
  powerful across many more PDF types/wordings/languages — "if that can help SpecForge be more
  powerful and address much more different PDF types, why not."

## Why parked (and why not now)

- **Prove-it-first:** the current grammar-based pipeline must be shown complete and excellent before
  swapping its front door. Replacing the extraction grammar with a model now would conflate "does
  the architecture work" with "does the model work."
- **Risk/control:** grammar extraction is deterministic, inspectable, and hermetically testable;
  a model-based extractor trades that for coverage. That trade is only worth making once the
  deterministic baseline is locked and measured (so the model is judged against a known-good bar).
- **Still PDF-name-agnostic either way:** a future model-based extractor must *also* honor ADR
  `0006` — it derives names from the document, never memorizes them. The model would generalize the
  **how**, not bake in **names**.

## Scope (when activated)

- A model-based intent extractor (premise: an LLM reads spec prose/tables and emits the typed
  IR directly or as candidates), reconciled with the deterministic backbone (the model proposes,
  validation/arbitration decide — same bounded-LLM doctrine as the NLI gate). Builds on the
  validated local text model (`qwen2.5:14b-instruct`) and the `commands::llm_text` transport.
- Measured against the grammar baseline on the eval harness; must not regress precision and should
  expand coverage (more protocols, looser wordings, non-AMBA-style prose) to justify itself.

## Task Tree

- ID: `PURE-NLP-INTENT-EXTRACTION` · Status: `active` · Children: `.1` `.2`
- ID: `PURE-NLP-INTENT-EXTRACTION.1` · Status: `done` · Goal: first increment — a **bounded-LLM
  relation extractor** (the cleanest task) establishing the pure-NLP path.
  Done (`2026-06-05`): new module `crates/specforge/src/ir/nlp_relation_extract.rs`. The model
  **proposes** `actor → signal` relations from a sentence (returning JSON `{actor, relation, signal}`);
  the document's own **declared signals decide** which survive (a proposal naming an undeclared
  signal is dropped) — so the model generalizes the *method* while ADR 0006 holds (names from the
  sentence + declared set, never memorized). `relation_extract_prompt`, `parse_nlp_relations`
  (bounded validation + dedup + never-panics), `nlp_extract_relations` (provider-backed, fail-open,
  hermetically mockable via `SPECFORGE_VLM_HELPER`). Opt-in/additive — the deterministic extractor
  stays the default. +3 tests; CI green 1262→1265.
- ID: `PURE-NLP-INTENT-EXTRACTION.2` · Status: `done` · Goal: the next increment, **re-scoped
  after investigating the existing pipeline (`2026-06-05`).**
  **Key finding — the proposed increments largely ALREADY EXIST.** `commands/signal_resolve.rs` *is*
  the production bounded-LLM **relation** extractor: it runs over **all** normative sentences
  (`candidate_work`), asks the provider per sentence (`build_relation_prompt`), grounds the signal
  against the declared set (anti-fabrication gate), tags `ExtractorTier::Nlp`, and dedups — exactly
  the "model proposes, document decides" doctrine. `eval-extraction` already **A/Bs** it vs the
  pattern baseline (`--provider skip`), reusing the same scorecards (now per-kind via
  EVAL-RELATION-GRANULARITY). The bounded-LLM **constraint** path also exists (`nlp_enrich`). So
  `nlp_relation_extract` (`.1`) substantially **duplicates** `signal-resolve`, with **one genuine
  difference**: it parses *multiple* relations per sentence, while `signal-resolve` extracts exactly
  one (`"a single actor→signal relation"`). **The real remaining increment: multi-relation-per-
  sentence recall** — teach `signal-resolve` to extract every relation a sentence states (a sentence
  like "the Manager drives PADDR and reads PREADY" currently yields one), reusing `.1`'s array parse
  + grounding so `.1` becomes used, not redundant. Verify with the eval A/B (recall up, precision
  held by grounding).
  **DONE (`2026-06-05`):** implemented in `commands/signal_resolve.rs` — `build_relation_prompt` now
  requests a JSON **array** (every relation the sentence states); a shared `relation_from_value` gate
  + `classify_relation_responses` parse all grounded edges (ids `r14:<stmt>:<n>`, intra-response
  dedup), and the run loop appends each (provenance-tagged `Nlp`, deduped vs existing). Removed the
  superseded single-object classifier + `RelationOutcome` enum and converted their tests; +2
  multi-relation tests. **Strictly additive recall** — an array of one is the old behavior, bounded
  by the same anti-fabrication grounding gate. CI green 1270. (Real-APB check: end-to-end runs and
  grounds correctly; whether a given spec hits a two-edge sentence is data-dependent.)

## Activation Checklist (gate — all met)

- [x] SpecForge proven on grammar-based extraction (`REEXTRACTION-REMEASURE`: 42→16, 69 relations).
- [x] `PDF-AGNOSTIC-EXTRACTION` closed (no hardcoded chip-spec names; CI guard).
- [x] Grammar-baseline extraction quality measured and recorded (the bar the model must beat).
- [x] Owner activated this tree (directive "do all 5 bullets", 2026-06-05).

## Decisions

- `2026-06-05`: deferred by owner. Grammar + document-derived names is the path *now*; a pure-NLP
  model is a deliberate future enhancement, gated on SpecForge completeness, and itself bound by
  ADR `0006` (no memorized names).

## Changelog

- `2026-06-05`: Created **parked**. Captures the future pure-NLP-model option without activating it,
  so the idea is not lost and not prematurely pursued.
- `2026-06-05`: **UN-PARKED + `.1` done** (owner "do all 5" — item 5/5, the capstone). Activation
  gate met (grammar engine validated on real data). First increment: bounded-LLM relation extractor
  module (`ir/nlp_relation_extract.rs`) — model proposes, declared signals decide (ADR 0006). Opt-in,
  +3 tests, CI green 1265.
