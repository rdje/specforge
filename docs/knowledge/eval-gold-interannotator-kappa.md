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

**Result (2026-06-05): κ = 0.90, raw agreement 17/18 = 0.944** → "almost perfect" (Landis-Koch).
The gold is **reliable** — not one annotator's idiosyncrasy. The single disagreement is a genuine
interpretive ambiguity, not an error: `statement_0202` *"PADDR, PWDATA, and **any other control
signals**, must be stable"* — the gold labeled only the named signals; the reviewer extended the
generic "any other control signals" clause to **PENABLE** (`must_be_stable`). Both are defensible;
recorded as a known ambiguity rather than a gold fix (changing it would shift eval scoring on a
debatable call).

Methodology grounded in Cohen (1960) / Krippendorff α / Artstein-Poesio (CL 2008);
`extraction-evaluation.md`. Caveat: 18 units is small → κ is a strong *signal*, not a precise
estimate. **Stronger follow-up:** run the local Ollama **qwen** VLM as a second (cross-model)
rater for genuinely model-independent agreement — not reachable from the sandboxed shell, runs in
the production environment. The `actor_signal_relation` task is an extensible follow-up.
