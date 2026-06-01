# 0002 — Production LLM/VLM provider is local Ollama + qwen2.5vl:7b

- Date: 2026-06-01
- Status: accepted
- Tags: llm, vlm, provider, ollama

## Context

SpecForge HAS a working, production LLM/VLM provider — it is not missing or aspirational
(an earlier session wrongly claimed it was absent; that claim was wrong). One
vision-language model, **`qwen2.5vl:7b` served via local Ollama** (`http://localhost:11434`),
backs all four optional LLM passes. It is the default for `ollama`/`lmstudio` in code
(`commands/llm_text.rs::default_model`, `commands/enrich.rs`); OpenAI defaults to `gpt-4o`.

## Decision

Treat local Ollama + `qwen2.5vl:7b` as the production default for:
- `enrich` — VLM image pass: timing diagrams / state machines / waveforms from page images.
- `nlp-enrich` — prose → typed `SignalConstraint` (NLP Level-3).
- `extract-contracts` — prose → typed `ActorContract` (R16 constrained-verified).
- `signal-resolve` — Tier-3 prose → actor→signal `drives`/`reads` relations.

All four are **optional, additive** passes; the deterministic pipeline
(ingest→evidence→semantic→intent) runs without the LLM. They **fail closed** (malformed/
ungrounded/unverifiable output → residual, never fabricated). So the model is a recall
booster, not on the critical path.

## Consequences

- Never describe the LLM/VLM provider as missing. The single-model Ollama server can be
  **saturated** (concurrent `converge` jobs starve it); record server-gated runs honestly.
- Model swaps are config, not code: `--vlm-model` / `--model` at runtime; defaults live
  in `llm_text.rs` + `enrich.rs`. A rigorous comparison needs a labeled eval — see the
  `LLM-EXTRACTION-EVAL` task-tree (the recall gauge is unsupervised; `kg-bench` tests the
  deterministic pipeline only).
- Candidate upgrade: `qwen3-vl:8b` (Apache-2.0, ~6.1G on Ollama vs 6.0G — near drop-in;
  newer joint text+vision pretraining). Evaluate, don't assume; local Ollama is 0.18.2
  (current), so it's compatible.

## Links

- Task-trees: `LLM-EXTRACTION-EVAL`, `CVE-PROSE-EXTRACTION`, `R14-SIGNAL-RESOLVE`.
