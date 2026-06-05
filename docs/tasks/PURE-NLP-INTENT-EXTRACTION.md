# PURE-NLP-INTENT-EXTRACTION: model-based intent extraction (PARKED — future)

## Metadata

- Tree ID: `PURE-NLP-INTENT-EXTRACTION`
- Status: **`parked` (deferred — DO NOT ACTIVATE yet)**
- Activation gate: **only after SpecForge is complete and proven to work *very well* on the current
  grammar-based extraction.** Until then this tree stays parked; no leaf may be started.
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

- ID: `PURE-NLP-INTENT-EXTRACTION` · Status: `parked` · Children: (none until activated)

## Activation Checklist (gate — all required before un-parking)

- [ ] SpecForge declared complete on the current grammar-based extraction.
- [ ] `PDF-AGNOSTIC-EXTRACTION` closed (no hardcoded chip-spec names anywhere).
- [ ] Grammar-baseline extraction quality measured and recorded (the bar the model must beat).
- [ ] Owner explicitly activates this tree.

## Decisions

- `2026-06-05`: deferred by owner. Grammar + document-derived names is the path *now*; a pure-NLP
  model is a deliberate future enhancement, gated on SpecForge completeness, and itself bound by
  ADR `0006` (no memorized names).

## Changelog

- `2026-06-05`: Created **parked**. Captures the future pure-NLP-model option without activating it,
  so the idea is not lost and not prematurely pursued.
