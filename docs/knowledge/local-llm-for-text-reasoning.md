---
id: local-llm-for-text-reasoning
title: Text-reasoning gates (NLI, semantic checks) want a strong TEXT LLM, not a VLM; qwen2.5:14b-instruct is viable for NLI
answers:
  - "which local model should SpecForge use for NLI or entailment verification"
  - "do text-reasoning tasks need a vision model"
  - "is qwen2.5:14b-instruct good enough for NLI"
  - "why is the NLI framing better than free-form labeling"
  - "which local models are pulled and what are they for"
date: 2026-06-05
tags: [llm, nli, ollama, qwen, architecture, verification]
evidence: docs/knowledge/eval-gold-interannotator-kappa.md; docs/tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md
reverify: ollama list  # qwen2.5:14b-instruct (text) + qwen2.5vl:7b (vision); re-run the NLI/kappa probes
---

SpecForge's **text-reasoning gates** (NLI entailment verification, semantic checks, κ-style
judgment) are pure text — **no vision needed**; a vision-language model spends capacity on image
understanding that sits idle and is often *weaker* at nuanced text. Vision earns its keep only on
the genuinely **multimodal** extraction (figures, waveform/state diagrams, table-as-image). So the
right shape is **two local models**: a VLM (`qwen2.5vl:7b`) for multimodal extraction, and a
strong **text-only** instruct LLM for the reasoning/verification gates.

**Empirical (2026-06-05), AMBA APB constraint/relation gold + an NLI probe:**

| model | constraint-label κ | relation-label κ | NLI entailment |
| --- | --- | --- | --- |
| Claude agent (frontier) | 0.90 | 1.00 | — |
| `qwen2.5:14b-instruct` (text) | 0.498 | **1.00** | **5/6** |
| `qwen2.5vl:7b` (vision, default extractor) | 0.285 | — | — |

Key findings: (1) **`qwen2.5:14b-instruct` is viable for NLI** — 5/6, and it correctly rejected
*"PSEL must be asserted"* as NOT-entailed (the condition-vs-obligation nuance), the one "miss"
being a defensible strict read of an "until …" qualifier. (2) The **NLI/entailment framing beats
free-form labeling**: the *same* model that mislabeled condition signals as obligations in the
labeling task judged the entailment of a *specific claim* correctly — so build the verifier as
"does the source entail THIS claim?", not "what constraint does this signal carry?". (3)
qwen2.5:14b is a **perfect cross-model rater on the clean relation task** (κ = 1.00) but only
moderate on the harder constraint-labeling nuance (κ = 0.498) — that nuance needs the frontier
model (κ = 0.90 validates the gold). `qwen3-vl:8b` was removed (its non-disable-able thinking made
it too slow for batches). **Consequence:** the NLI verifier ("gated: needs a model") is now
**unblocked** — `qwen2.5:14b-instruct` is the local model for it.
