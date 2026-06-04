---
id: llm-vlm-provider-default
title: SpecForge ships a production Ollama+Qwen2.5VL provider (the default LLM/VLM)
answers:
  - "which LLM or VLM does SpecForge use"
  - "is the LLM/VLM provider missing or not wired up"
  - "what model do converge / enrich / nlp-enrich use by default"
  - "qwen2.5vl vs qwen3-vl which model"
  - "default model for the ollama provider"
date: 2026-06-01
tags: [llm, vlm, provider, ollama]
evidence: docs/decisions/0002-llm-vlm-provider-default.md; crates/specforge/src/commands/llm_text.rs:35
reverify: grep -n "qwen2.5vl" crates/specforge/src/commands/llm_text.rs
---

SpecForge HAS a production local LLM/VLM provider — **Ollama serving `qwen2.5vl:7b`** — used
for VLM enrich and the NLP-L3 commands (`enrich`, `nlp-enrich`, `extract-contracts`,
`signal-resolve`), all fail-closed. It is the default model for `--provider ollama`. **Never
claim the LLM/VLM provider is missing.** The qwen2.5-vs-`qwen3-vl:8b` A/B is one
`eval-extraction … --model qwen3-vl:8b` command away (measured improvement was marginal; the
default stays `qwen2.5vl:7b`).

Canonical home: `docs/decisions/0002-llm-vlm-provider-default.md`.
