---
id: nli-entailment-verifier
title: NLI entailment verifier — a semantic "does the source entail this claim?" grounding gate
answers:
  - "how does SpecForge verify an extracted claim semantically / catch hallucination"
  - "what is the NLI entailment verifier"
  - "how is a claim's grounding checked beyond a string match"
  - "what model does the NLI verifier use"
  - "what happens when the NLI provider is down"
date: 2026-06-05
tags: [nli, grounding, hallucination, bounded-llm, verification]
evidence: crates/specforge/src/ir/nli_verify.rs
reverify: grep -n "fn verify_entailment" crates/specforge/src/ir/nli_verify.rs
---

`ir/nli_verify.rs` is the NLI entailment grounding gate: premise = source statement, hypothesis =
an extracted claim → a text LLM judges entailment; keep only `ENTAILED`. `NliVerdict { Entailed,
NotEntailed, Unknown }`; `entailment_prompt` (deterministic; states the condition-vs-obligation
rule), `parse_nli_verdict` (**fail-closed to `Unknown`**; `NOT_ENTAILED` beats a substring
`ENTAILED`), `verify_entailment` (reuses `commands::llm_text::call_text_provider` + the
`SPECFORGE_VLM_HELPER` hermetic test hook).

The gate is **additive + fail-safe** — `gate_action`: `NotEntailed` → `RouteResidual` (likely
hallucination), `Entailed` → `Keep`, `Unknown` (provider error/unclear) → **`Abstain`** (leave the
existing rule-based grounding in charge; a provider outage must never nuke extraction). Default
model `DEFAULT_NLI_MODEL = qwen2.5:14b-instruct` — a **text** model, not the VLM (entailment is text
reasoning, and the entailment framing beats free-form labeling — see
`[[local-llm-for-text-reasoning]]`). **No CI test requires Ollama** (pure unit tests + the mock
helper).

It catches the **condition-vs-obligation** hallucination a string-match gate misses (e.g. "PBUSER
valid *when* PSEL asserted" ⇏ "PSEL must be asserted"). `.2` is the module; `.3` shipped the gate
applied to a claim set — `constraint_claim_text` (constraint → NLI hypothesis), `nli_claim_findings`
(premise = `constraint.source_text`, verifier injected → fully testable with no provider), and the
live **`specforge nli-verify <evidence_ir.json>`** command (reports the constraint claims the source
does not entail; `--vlm-provider skip` no-ops; `--model` overrides). Auto-routing NotEntailed claims
to residuals *inside* `converge` is the documented next extension. See
`docs/tasks/NLI-ENTAILMENT-VERIFIER.md`.
