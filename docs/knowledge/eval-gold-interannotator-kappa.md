---
id: eval-gold-interannotator-kappa
title: Eval gold is reliable — Cohen's kappa 0.90 (almost-perfect) on the constraint task
answers:
  - "is the eval gold / answer key trustworthy or reliable"
  - "what is the inter-annotator agreement of the eval gold"
  - "did a second annotator validate seed_apb.json"
  - "how was the eval gold checked for idiosyncrasy"
date: 2026-06-05
tags: [eval, gold, inter-annotator, kappa, reliability]
evidence: crates/specforge/test_data/llm_eval/seed_apb.json; docs/tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md
reverify: python3 -c "see EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md Method — re-run a blind second annotation and recompute"
---

The `signal_constraint` eval gold (`seed_apb.json`, `label_status: agent_drafted`,
single-source) was validated by an **independent blind second annotation** (a fresh agent given
only the statement text + the listed signals + the label scheme — no gold, no `label_note`s),
scored as **Cohen's κ** over the 18 *(statement, signal)* units (label = `constraint_kind` or
`NONE`).

**Results (2026-06-05):**
- **`signal_constraint` task: κ = 0.90, raw agreement 17/18 = 0.944** ("almost perfect").
- **`actor_signal_relation` task: κ = 1.00, 11/11 = 1.0** (perfect — a second blind agent labeled
  who *drives* each signal identically to the gold).

So **both halves of the eval foundation are reliable** — not one annotator's idiosyncrasy. The
single constraint disagreement is a genuine interpretive ambiguity, not an error: `statement_0202`
*"PADDR, PWDATA, and **any other control signals**, must be stable"* — the gold labeled only the
named signals; the reviewer extended the generic clause to **PENABLE** (`must_be_stable`). Both
defensible; recorded as a known ambiguity, not a gold fix (changing it shifts eval scoring on a
debatable call).

**Cross-model (local Ollama qwen) attempts — the annotator's competence dominates the number.**
- **qwen3-vl:8b** *is* reachable (start `ollama serve`; pulled) and answers single items
  correctly, but its thinking is **not disable-able** (`think:false` and `/no_think` both ignored;
  ~3400 thinking tokens/signal) → a full batch is too slow here (4/7 statements time out). On the
  6 units it completed it agreed with the gold **5/6** — consistent with a reliable gold.
- **qwen2.5vl:7b** (the project default; no thinking → fast, 21 s for the whole batch) completed
  it but scored only **κ = 0.285**. Crucially, that is a statement about **the model, not the
  gold**: almost every disagreement is qwen2.5vl mislabeling a **condition/trigger** signal as an
  obligation — e.g. "PBUSER must be valid **when PSEL, PENABLE, PREADY are asserted**" → it marked
  the three condition signals `must_be_asserted`; "if PSELx are HIGH" → `must_be_high`; "until
  PREADY is asserted" → `must_be_asserted` — plus it invented an out-of-set label
  (`must_be_same_width`). I.e. it conflates *"X is asserted (condition)"* with *"X must be asserted
  (obligation)"*. Inter-annotator agreement is only meaningful between **competent** annotators;
  qwen2.5vl:7b is too weak at the condition-vs-obligation distinction to validate or challenge the
  gold here. The capable reviewer (Claude agent) agreed at κ = 0.90 — that is the reliability
  signal.

- **qwen2.5:14b-instruct** (text-only, pulled on request; replaced qwen3-vl:8b): fast (~22 s),
  **relation κ = 1.00** (perfect cross-model on the clean task), **constraint κ = 0.498** (better
  than the 7B VLM but still trips on condition-vs-obligation in the *labeling* framing — it reads
  "when PSEL … are asserted" as `must_be_asserted`), and **NLI entailment 5/6** — notably it judged
  *"PSEL must be asserted"* NOT-entailed correctly (the nuance it flubbed in labeling). See
  `[[local-llm-for-text-reasoning]]` for the model-choice conclusion.

**Side-findings:** (1) qwen2.5vl:7b is **SpecForge's default extraction VLM** and reads condition
signals as constrained — so the *raw* VLM extraction likely over-constrains "when X …" conditions
(what the deterministic backbone + grounding catch). (2) The **NLI/entailment framing beats
free-form labeling** for this nuance, and **qwen2.5:14b-instruct is a viable local model for an NLI
verifier** — see `[[local-llm-for-text-reasoning]]`.

Methodology grounded in Cohen (1960) / Krippendorff α / Artstein-Poesio (CL 2008);
`extraction-evaluation.md`. Caveat: 18+11 units is small → κ is a strong *signal*, not a precise
estimate.
